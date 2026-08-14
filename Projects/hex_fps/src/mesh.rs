use hex::{
    anyhow,
    assets::{Mesh, mesh::Vertex3},
    context::Context3,
    nalgebra::{Vector2, Vector3},
};
use std::f32::consts::{PI, TAU};

/// Each face as (normal, u axis, v axis). Winding the quad as
/// `(-u,-v) -> (+u,-v) -> (+u,+v) -> (-u,+v)` makes every triangle
/// counter-clockwise when seen from outside the cube.
const FACES: [([f32; 3], [f32; 3], [f32; 3]); 6] = [
    ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    ([0.0, 0.0, -1.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    ([1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]),
    ([-1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]),
    ([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
    ([0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
];

/// Unit cube centred on the origin, with flat per-face normals.
pub fn cube(context: &Context3) -> anyhow::Result<Mesh> {
    let mut vertices = Vec::with_capacity(24);
    let mut indices = Vec::with_capacity(36);

    for (normal, u, v) in FACES {
        let (normal, u, v) = (Vector3::from(normal), Vector3::from(u), Vector3::from(v));
        let base = vertices.len() as u32;

        for (su, sv) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            vertices.push(Vertex3::new(
                (normal + u * su + v * sv) * 0.5,
                normal,
                Vector2::new((su + 1.0) * 0.5, (sv + 1.0) * 0.5),
            ));
        }

        indices.extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    Mesh::new(context, &vertices, &indices)
}

/// UV sphere of diameter 1, so it scales like the cube: `scale = 2 * radius`.
pub fn sphere(rings: u32, sectors: u32, context: &Context3) -> anyhow::Result<Mesh> {
    let mut vertices = Vec::with_capacity(((rings + 1) * (sectors + 1)) as usize);

    for ring in 0..=rings {
        let v = ring as f32 / rings as f32;
        let (sin_phi, cos_phi) = (v * PI).sin_cos();

        for sector in 0..=sectors {
            let u = sector as f32 / sectors as f32;
            let (sin_theta, cos_theta) = (u * TAU).sin_cos();
            let normal = Vector3::new(sin_phi * cos_theta, cos_phi, sin_phi * sin_theta);

            vertices.push(Vertex3::new(normal * 0.5, normal, Vector2::new(u, v)));
        }
    }

    let stride = sectors + 1;
    let mut indices = Vec::with_capacity((rings * sectors * 6) as usize);

    for ring in 0..rings {
        for sector in 0..sectors {
            let top = ring * stride + sector;
            let bottom = top + stride;

            indices.extend([top, bottom + 1, bottom, top, top + 1, bottom + 1]);
        }
    }

    Mesh::new(context, &vertices, &indices)
}

/// A flat-top hexagonal prism extruded along Y, centred on the origin.
/// `size` is the centre-to-corner distance, `height` the extrusion length.
/// All triangles wind counter-clockwise when viewed from outside.
pub fn hex_prism(size: f32, height: f32, context: &Context3) -> anyhow::Result<Mesh> {
    let h = height / 2.0;
    let corners: Vec<Vector3<f32>> = (0..6)
        .map(|i| {
            let a = i as f32 * PI / 3.0;

            Vector3::new(size * a.cos(), 0.0, size * a.sin())
        })
        .collect();

    let mut vertices: Vec<Vertex3> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    fn push(
        vertices: &mut Vec<Vertex3>,
        position: Vector3<f32>,
        normal: Vector3<f32>,
        uv: Vector2<f32>,
    ) -> u32 {
        vertices.push(Vertex3::new(position, normal, uv));

        (vertices.len() - 1) as u32
    }

    // Top cap, normal +Y.
    let top_center = push(
        &mut vertices,
        Vector3::new(0.0, h, 0.0),
        Vector3::y(),
        Vector2::new(0.5, 0.5),
    );
    let top: Vec<u32> = corners
        .iter()
        .map(|c| {
            push(
                &mut vertices,
                Vector3::new(c.x, h, c.z),
                Vector3::y(),
                Vector2::new(0.5 + c.x / (2.0 * size), 0.5 + c.z / (2.0 * size)),
            )
        })
        .collect();
    for i in 0..6 {
        let n = (i + 1) % 6;

        indices.extend_from_slice(&[top_center, top[n], top[i]]);
    }

    // Bottom cap, normal -Y.
    let bottom_center = push(
        &mut vertices,
        Vector3::new(0.0, -h, 0.0),
        -Vector3::y(),
        Vector2::new(0.5, 0.5),
    );
    let bottom: Vec<u32> = corners
        .iter()
        .map(|c| {
            push(
                &mut vertices,
                Vector3::new(c.x, -h, c.z),
                -Vector3::y(),
                Vector2::new(0.5 + c.x / (2.0 * size), 0.5 + c.z / (2.0 * size)),
            )
        })
        .collect();
    for i in 0..6 {
        let n = (i + 1) % 6;

        indices.extend_from_slice(&[bottom_center, bottom[i], bottom[n]]);
    }

    // Side walls, each with its own outward normal.
    for i in 0..6 {
        let n = (i + 1) % 6;
        let normal = (corners[i] + corners[n]).normalize();

        let ti = push(
            &mut vertices,
            corners[i] + Vector3::y() * h,
            normal,
            Vector2::new(0.0, 1.0),
        );
        let bi = push(
            &mut vertices,
            corners[i] - Vector3::y() * h,
            normal,
            Vector2::new(0.0, 0.0),
        );
        let bn = push(
            &mut vertices,
            corners[n] - Vector3::y() * h,
            normal,
            Vector2::new(1.0, 0.0),
        );
        let tn = push(
            &mut vertices,
            corners[n] + Vector3::y() * h,
            normal,
            Vector2::new(1.0, 1.0),
        );

        indices.extend_from_slice(&[ti, bn, bi, ti, tn, bn]);
    }

    Mesh::new(context, &vertices, &indices)
}
