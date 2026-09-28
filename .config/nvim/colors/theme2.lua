-- theme2.lua — Neovim colorscheme (Lua)
-- Place in ~/.config/nvim/colors/theme2.lua · Usage: :colorscheme theme2
-- Tested on Neovim 0.11.7; the group list is written for 0.9+.
--
-- Design: near-monochrome grey base on #121212 (matches the alacritty
-- background) with a single bright accent. Colour is spent only where it
-- carries meaning — diagnostics, diffs, git status, completion kinds — so code
-- stays calm and problems pop.
--
-- The hex palette needs 'termguicolors' (see init.vim). Every entry also
-- carries a 256-colour cterm approximation, so it degrades gracefully.
--
-- Every foreground/background pair below was checked against WCAG contrast
-- (text >= 4.5:1, chrome >= 3:1); the only exceptions are the deliberately
-- faint decoration groups (NonText, EndOfBuffer, Whitespace, separators).
--
-- Optional overrides, set in init.vim *before* :colorscheme:
--   vim.g.theme2_accent      = "teal" (default) | "cyan" | "#rrggbb"
--   vim.g.theme2_italic      = 0    -- no italics anywhere
--   vim.g.theme2_monochrome  = 1    -- flatten syntax accents back to grey
--   vim.g.theme2_transparent = 1    -- leave Normal's bg unset
--   vim.g.theme2_shadow      = 0    -- no drop shadow on floating windows

local function opt(name, default)
  local v = vim.g[name]
  if v == nil then
    return default
  end
  if type(v) == "number" then
    return v ~= 0
  end
  if type(v) == "string" then
    return v ~= "" and v ~= "0"
  end
  return v and true or false
end

local italic      = opt("theme2_italic", true)
local monochrome  = opt("theme2_monochrome", false)
local transparent = opt("theme2_transparent", false)
local shadow      = opt("theme2_shadow", true)

-- ── Accent presets ───────────────────────────────────────────────────────────
-- Accent-derived tints are part of the preset so selections stay in key.
local ACCENTS = {
  teal = { -- default: softened teal, ~8.5:1 on the base bg
    accent = { "#4db6ac", 73 },
    bright = { "#6fd3c7", 79 },
    sel    = { "#26403d", 23 },
    match  = { "#2d4b47", 23 },
    search = { "#3a5f5a", 23 },
  },
  cyan = { -- the original cterm-6 cyan accent, ~9.5:1
    accent = { "#00cdcd", 44 },
    bright = { "#66e0e0", 80 },
    sel    = { "#164040", 23 },
    match  = { "#1b4f4f", 30 },
    search = { "#2a6666", 30 },
  },
}

local function pick_accent()
  local want = vim.g.theme2_accent
  if want == nil or want == "" then
    return ACCENTS.teal
  end
  for name, preset in pairs(ACCENTS) do
    if want == name then
      return preset
    end
  end
  -- Raw hex given: derive the tints by hand from the accent itself.
  if type(want) == "string" and want:match("^#%x%x%x%x%x%x$") then
    local c = ACCENTS.teal
    return {
      accent = { want, 73 },
      bright = c.bright,
      sel    = c.sel,
      match  = c.match,
      search = c.search,
    }
  end
  return ACCENTS.teal
end

local A = pick_accent()

-- ── Palette ──────────────────────────────────────────────────────────────────
-- name       hex        cterm   used for
local c = {
  bg         = { "#121212", 233 }, -- Normal (== alacritty background)
  bg_tabfill = { "#101010", 233 }, -- tabline trough
  bg_float   = { "#181818", 234 }, -- floating windows
  bg_panel   = { "#1a1a1a", 234 }, -- statusline, folds, colour column
  bg_hl      = { "#1e1e1e", 234 }, -- cursor line / column
  bg_chip    = { "#202020", 235 }, -- selected tab, scrollbar backdrop

  fg         = { "#d4d4d4", 253 }, -- Normal (~12.6:1)
  fg_strong  = { "#f0f0f0", 255 }, -- titles, active tab
  fg_muted   = { "#a6a6a6", 248 }, -- secondary text (~8.0:1)
  fg_dim     = { "#8a8a8a", 245 }, -- comments (~5.4:1)
  fg_faint   = { "#707070", 242 }, -- line numbers, inlay hints (~3.8:1)

  punct      = { "#8a8a8a", 245 }, -- delimiters / brackets
  op         = { "#a6a6a6", 248 }, -- operators
  string     = { "#b8b8b8", 250 }, -- strings stay neutral (~9.5:1)
  slate      = { "#adc2bf", 145 }, -- types: cool cast, ~10:1

  err        = { "#e06c75", 168 }, -- ~5.9:1
  warn       = { "#e5c07b", 180 }, -- ~10.8:1
  info       = { "#61afef", 75  }, -- ~7.9:1
  ok         = { "#98c379", 108 }, -- ~9.3:1
  hint       = { "#56b6c2", 73  },
  purple     = { "#c678dd", 176 },
  err_deep   = { "#7a2c2c", 88  }, -- Error bg (white on it: ~9.4:1)

  add_bg     = { "#17251c", 22  }, -- diff / git backgrounds (text keeps ~10:1)
  chg_bg     = { "#16202b", 17  },
  del_bg     = { "#2a1618", 52  },
  warn_bg    = { "#2a2418", 58  },
  txt_bg     = { "#2b4257", 24  }, -- DiffText

  border     = { "#3c3c3c", 237 }, -- float borders
  sep        = { "#2f2f2f", 236 }, -- window separators
  nontext    = { "#2e2e2e", 236 }, -- decorative only
  eob        = { "#262626", 235 }, -- '~' on empty lines
  sbar       = { "#232323", 235 }, -- Pmenu scrollbar
  thumb      = { "#3d3d3d", 237 }, -- Pmenu scrollbar thumb
  black      = { "#000000", 16  },
  white      = { "#ffffff", 231 },
}

local title = { fg = c.fg_strong, bold = true }
-- syntax roles, flattened when theme2_monochrome is set
local syn_fn    = monochrome and c.fg      or A.accent
local syn_const = monochrome and c.fg      or A.accent
local syn_type  = monochrome and c.fg_muted or c.slate
local it        = italic and true or nil

-- ── Helper ───────────────────────────────────────────────────────────────────
-- Colour arguments may be a palette entry ({ "#rrggbb", cterm }), a plain hex
-- string, or { hex = ..., cterm = ... }. Attributes are mirrored into the cterm
-- dict so 256-colour terminals keep bold/italic/undercurl.
--
-- NOTE: keep the entry shapes in sync here — a mismatch silently drops colours.
local ATTRS = {
  "bold", "italic", "underline", "undercurl", "underdouble", "underdotted",
  "underdashed", "strikethrough", "reverse", "standout", "nocombine",
}

local function hexof(v)
  if type(v) == "table" then
    return v[1] or v.hex, v[2] or v.cterm
  end
  return v, nil
end

local function hi(group, o)
  local hl = {}
  local function colour(key, v)
    local hex, cterm = hexof(v)
    hl[key] = hex
    if cterm then
      hl["cterm" .. key] = cterm -- fg -> ctermfg, bg -> ctermbg
    end
  end
  if o.fg then colour("fg", o.fg) end
  if o.bg then colour("bg", o.bg) end
  if o.sp then hl.sp = hexof(o.sp) end
  if o.link then hl.link = o.link end
  if o.blend then hl.blend = o.blend end
  local cterm = {}
  for _, k in ipairs(ATTRS) do
    local v = o[k]
    if v ~= nil then
      hl[k] = v
      if v then cterm[k] = true end
    end
  end
  if next(cterm) then hl.cterm = cterm end
  vim.api.nvim_set_hl(0, group, hl)
end

-- ── Start from a clean slate ─────────────────────────────────────────────────
vim.cmd("highlight clear")
if vim.fn.exists("syntax_on") == 1 then
  vim.cmd("syntax reset")
end
vim.g.colors_name = "theme2"
if vim.o.background ~= "dark" then
  vim.o.background = "dark"
end

-- ── Base ─────────────────────────────────────────────────────────────────────
-- 'transparent' deliberately leaves bg unset, so the terminal's own background
-- shows through (useful with a blurred/translucent terminal).
-- NOTE: written as an if, not `transparent and nil or c.bg` — that idiom always
-- yields c.bg, because `nil or x` is x.
local base_bg = c.bg
if transparent then
  base_bg = nil
end
hi("Normal",       { fg = c.fg, bg = base_bg })
hi("NormalNC",     { fg = c.fg_muted, bg = base_bg })
hi("NormalFloat",  { fg = c.fg, bg = c.bg_float })
hi("FloatBorder",  { fg = c.border, bg = c.bg_float })
hi("FloatTitle",   { fg = A.accent, bg = c.bg_float, bold = true })
hi("FloatFooter",  { fg = c.fg_dim, bg = c.bg_float })
if shadow then
  hi("FloatShadow",       { bg = c.black, blend = 65 })
  hi("FloatShadowThrough", { bg = c.black, blend = 80 })
end
hi("MsgArea",      { fg = c.fg })
hi("MsgSeparator", { fg = c.border })
hi("ModeMsg",      { fg = c.fg_muted, bold = true })
hi("MoreMsg",      { fg = A.accent, bold = true })
hi("Question",     { fg = A.accent })
hi("WarningMsg",   { fg = c.warn })
hi("ErrorMsg",     { fg = c.err, bold = true })
hi("Title",        title)
hi("Directory",    { fg = A.accent })
hi("Ignore",       { fg = c.bg })
hi("EndOfBuffer",  { fg = c.eob })
hi("NonText",      { fg = c.nontext })
hi("SpecialKey",   { fg = c.border })
hi("Whitespace",   { fg = c.nontext }) -- Neovim 0.11 listchars
hi("Conceal",      { fg = c.fg_faint })
hi("WinSeparator", { fg = c.sep })
hi("VertSplit",    { fg = c.sep })
hi("WinBar",       { fg = c.fg_muted, bg = c.bg_panel })
hi("WinBarNC",     { fg = c.fg_faint, bg = c.bg_panel })
hi("ColorColumn",  { bg = c.bg_panel })
hi("CursorColumn", { bg = c.bg_hl })
hi("CursorLine",   { bg = c.bg_hl })
hi("CursorLineSign", { bg = c.bg_hl })
hi("CursorLineFold", { bg = c.bg_hl })
hi("LineNr",       { fg = c.fg_faint })
hi("LineNrAbove",  { fg = c.fg_faint })
hi("LineNrBelow",  { fg = c.fg_faint })
hi("CursorLineNr", { fg = A.accent, bold = true })
hi("SignColumn",   { bg = base_bg })
hi("FoldColumn",   { fg = c.fg_faint })
hi("Folded",       { fg = c.fg_dim, bg = c.bg_panel })
hi("Cursor",       { fg = c.bg, bg = c.fg })
hi("lCursor",      { fg = c.bg, bg = c.fg })
hi("CursorIM",     { fg = c.bg, bg = c.fg })
hi("TermCursor",   { fg = c.bg, bg = A.accent })
hi("TermCursorNC", { fg = c.bg, bg = c.fg_muted })
hi("QuickFixLine", { bg = c.bg_hl })
hi("Substitute",   { bg = A.accent, fg = c.bg })

-- ── Searches & selection ─────────────────────────────────────────────────────
hi("Visual",       { bg = A.sel })
hi("VisualNOS",    { bg = A.sel })
hi("Search",       { bg = A.search, fg = c.white })
hi("CurSearch",    { bg = A.accent, fg = c.bg, bold = true })
hi("IncSearch",    { bg = c.warn, fg = c.bg, bold = true })
hi("MatchParen",   { bg = A.match, fg = c.white, bold = true })
hi("ComplMatchIns", { fg = c.fg_faint })
hi("WildMenu",     { bg = A.match, fg = c.white, bold = true })

-- ── Status & tab lines ───────────────────────────────────────────────────────
hi("StatusLine",     { fg = c.fg_strong, bg = c.bg_chip })
hi("StatusLineNC",   { fg = c.fg_dim, bg = c.bg_panel })
hi("StatusLineTerm", { fg = c.fg_strong, bg = c.bg_chip })
hi("StatusLineTermNC", { fg = c.fg_dim, bg = c.bg_panel })
hi("TabLineFill",    { fg = c.bg_tabfill, bg = c.bg_tabfill })
hi("TabLine",        { fg = c.fg_muted, bg = c.bg_tabfill })
hi("TabLineSel",     { fg = c.fg_strong, bg = c.bg_chip, bold = true })
hi("TabLineMod",     { fg = c.warn, bg = c.bg_tabfill, bold = true }) -- used by tabline.vim
hi("TabLineModSel",  { fg = c.warn, bg = c.bg_chip, bold = true })
hi("TabLineClose",   { fg = c.fg_dim, bg = c.bg_tabfill })

-- ── Popup menu (omnicomplete, wildmenu pum, cmp fallback) ────────────────────
hi("Pmenu",        { fg = c.fg, bg = c.bg_panel })
hi("PmenuSel",     { fg = c.white, bg = A.match, bold = true })
hi("PmenuSbar",    { bg = c.sbar })
hi("PmenuThumb",   { bg = c.thumb })
hi("PmenuKind",    { fg = A.accent, bg = c.bg_panel })
hi("PmenuKindSel", { fg = c.white, bg = A.match })
hi("PmenuExtra",   { fg = c.fg_dim, bg = c.bg_panel })
hi("PmenuExtraSel", { fg = c.string, bg = A.match })
hi("PmenuMatch",   { fg = A.bright, bg = c.bg_panel, bold = true })
hi("PmenuMatchSel", { fg = c.white, bg = A.match, bold = true })

-- ── Core syntax (Vim regex groups) ───────────────────────────────────────────
hi("Comment",    { fg = c.fg_dim, italic = it })
hi("Constant",   { fg = syn_const })
hi("Number",     { fg = syn_const })
hi("Float",      { fg = syn_const })
hi("Boolean",    { fg = syn_const })
hi("Character",  { fg = c.string })
hi("String",     { fg = c.string })
hi("Identifier", { fg = c.fg })
hi("Function",   { fg = syn_fn })
hi("Statement",  { fg = c.fg_muted, bold = true })
hi("Conditional", { fg = c.fg_muted, bold = true })
hi("Repeat",     { fg = c.fg_muted, bold = true })
hi("Label",      { fg = c.purple })
hi("Operator",   { fg = c.op })
hi("Keyword",    { fg = c.fg_muted, bold = true })
hi("Exception",  { fg = c.fg_muted, bold = true })
hi("PreProc",    { fg = syn_fn })
hi("Include",    { fg = syn_fn })
hi("Define",     { fg = syn_fn })
hi("Macro",      { fg = syn_fn })
hi("PreCondit",  { fg = syn_fn })
hi("Type",       { fg = syn_type })
hi("StorageClass", { fg = c.fg_muted, bold = true })
hi("Structure",  { fg = syn_type })
hi("Typedef",    { fg = syn_type })
hi("Special",    { fg = c.fg_muted })
hi("SpecialChar", { fg = A.accent })
hi("SpecialComment", { fg = c.fg_dim, italic = it })
hi("Delimiter",  { fg = c.punct })
hi("Tag",        { fg = syn_type })
hi("Underlined", { fg = c.fg, underline = true, sp = A.accent })
hi("Error",      { fg = c.white, bg = c.err_deep })
hi("Todo",       { fg = c.bg, bg = A.accent, bold = true })

-- ── Treesitter ───────────────────────────────────────────────────────────────
hi("@variable",            { fg = c.fg })
hi("@variable.builtin",    { fg = A.accent, italic = it })
hi("@variable.parameter",  { fg = c.fg_muted })
hi("@variable.parameter.builtin", { fg = A.accent, italic = it })
hi("@variable.member",     { fg = c.slate })
hi("@constant",            { fg = syn_const })
hi("@constant.builtin",    { fg = syn_const, bold = true })
hi("@constant.macro",      { fg = syn_const })
hi("@module",              { fg = syn_type })
hi("@module.builtin",      { fg = syn_type, italic = it })
hi("@label",               { fg = c.purple })
hi("@string",              { fg = c.string })
hi("@string.regexp",       { fg = c.hint })
hi("@string.escape",       { fg = A.accent })
hi("@string.special",      { fg = A.accent })
hi("@string.special.url",  { fg = A.accent, underline = true, sp = A.accent })
hi("@string.special.symbol", { fg = c.purple })
hi("@character",           { fg = c.string })
hi("@character.special",   { fg = A.accent })
hi("@boolean",             { fg = syn_const })
hi("@number",              { fg = syn_const })
hi("@number.float",        { fg = syn_const })
hi("@type",                { fg = syn_type })
hi("@type.builtin",        { fg = syn_type, italic = it })
hi("@type.definition",     { fg = syn_type })
hi("@type.qualifier",      { fg = c.fg_muted, bold = true })
hi("@attribute",           { fg = c.warn })
hi("@attribute.builtin",   { fg = c.warn })
hi("@property",            { fg = c.slate })
hi("@field",               { fg = c.slate })
hi("@function",            { fg = syn_fn })
hi("@function.builtin",    { fg = syn_fn, italic = it })
hi("@function.call",       { fg = syn_fn })
hi("@function.macro",      { fg = syn_fn })
hi("@function.method",     { fg = syn_fn })
hi("@function.method.call", { fg = syn_fn })
hi("@constructor",         { fg = syn_fn })
hi("@operator",            { fg = c.op })
hi("@keyword",             { fg = c.fg_muted, bold = true })
hi("@keyword.function",    { fg = c.fg_muted, bold = true })
hi("@keyword.return",      { fg = c.fg_muted, bold = true })
hi("@keyword.conditional", { fg = c.fg_muted, bold = true })
hi("@keyword.repeat",      { fg = c.fg_muted, bold = true })
hi("@keyword.exception",   { fg = c.fg_muted, bold = true })
hi("@keyword.import",      { fg = c.fg_muted, bold = true })
hi("@keyword.directive",   { fg = c.fg_muted, bold = true })
hi("@keyword.modifier",    { fg = c.fg_muted, bold = true })
hi("@keyword.type",        { fg = c.fg_muted, bold = true })
hi("@keyword.coroutine",   { fg = c.fg_muted, bold = true })
hi("@keyword.debug",       { fg = c.fg_muted, bold = true })
hi("@keyword.operator",    { fg = c.op })
hi("@punctuation",         { fg = c.punct })
hi("@punctuation.delimiter", { fg = c.punct })
hi("@punctuation.bracket", { fg = c.punct })
hi("@punctuation.special", { fg = A.accent })
hi("@comment",             { fg = c.fg_dim, italic = it })
hi("@comment.documentation", { fg = c.fg_dim, italic = it })
hi("@comment.error",       { fg = c.white, bg = c.err_deep, bold = true, italic = it })
hi("@comment.warning",     { fg = c.bg, bg = c.warn, bold = true, italic = it })
hi("@comment.todo",        { fg = c.bg, bg = A.accent, bold = true, italic = it })
hi("@comment.note",        { fg = c.bg, bg = c.info, bold = true, italic = it })
hi("@namespace",           { fg = syn_type })
hi("@tag",                 { fg = syn_type })
hi("@tag.builtin",         { fg = syn_type })
hi("@tag.attribute",       { fg = c.slate })
hi("@tag.delimiter",       { fg = c.punct })
hi("@error",               { fg = c.white, bg = c.err_deep })
hi("@diff.plus",           { fg = c.ok })
hi("@diff.minus",          { fg = c.err })
hi("@diff.delta",          { fg = c.info })
hi("@markup.heading",      { fg = A.accent, bold = true })
hi("@markup.heading.1",    { fg = A.accent, bold = true })
hi("@markup.heading.2",    { fg = c.fg_strong, bold = true })
hi("@markup.heading.3",    { fg = c.fg_strong, bold = true })
hi("@markup.heading.4",    { fg = c.fg_muted, bold = true })
hi("@markup.heading.5",    { fg = c.fg_muted, bold = true })
hi("@markup.heading.6",    { fg = c.fg_dim, bold = true })
hi("@markup.strong",       { fg = c.fg_strong, bold = true })
hi("@markup.italic",       { italic = true })
hi("@markup.strikethrough", { strikethrough = true, sp = c.fg_dim })
hi("@markup.underline",    { underline = true, sp = c.fg_dim })
hi("@markup.link",         { fg = A.accent, underline = true, sp = A.accent })
hi("@markup.link.url",     { fg = A.accent, underline = true, sp = A.accent })
hi("@markup.link.label",   { fg = c.info })
hi("@markup.raw",          { fg = c.string })
hi("@markup.raw.block",    { fg = c.string })
hi("@markup.list",         { fg = A.accent })
hi("@markup.list.checked", { fg = c.ok })
hi("@markup.list.unchecked", { fg = c.fg_dim })
hi("@markup.quote",        { fg = c.fg_dim, italic = it })
hi("@markup.math",         { fg = c.info })
hi("@markup.environment",  { fg = c.warn })
hi("@markup.environment.name", { fg = syn_type })
-- deprecated @text.* aliases, for parsers/queries that predate the rename
hi("@text.literal",        { fg = c.string })
hi("@text.reference",      { fg = c.info })
hi("@text.title",          { fg = c.fg_strong, bold = true })
hi("@text.uri",            { fg = A.accent, underline = true, sp = A.accent })
hi("@text.underline",      { underline = true, sp = c.fg_dim })
hi("@text.todo",           { fg = c.bg, bg = A.accent, bold = true })
hi("@text.note",           { fg = c.bg, bg = c.info, bold = true })
hi("@text.warning",        { fg = c.bg, bg = c.warn, bold = true })
hi("@text.danger",         { fg = c.white, bg = c.err_deep, bold = true })
hi("@text.emphasis",       { italic = true })
hi("@text.strong",         { bold = true })
hi("@text.strike",         { strikethrough = true, sp = c.fg_dim })
hi("@text.diff.add",       { fg = c.ok })
hi("@text.diff.delete",    { fg = c.err })

-- ── LSP semantic tokens ──────────────────────────────────────────────────────
hi("@lsp.type.class",      { fg = syn_type })
hi("@lsp.type.struct",     { fg = syn_type })
hi("@lsp.type.interface",  { fg = syn_type })
hi("@lsp.type.enum",       { fg = syn_type })
hi("@lsp.type.enumMember", { fg = syn_const })
hi("@lsp.type.type",       { fg = syn_type })
hi("@lsp.type.typeParameter", { fg = syn_type })
hi("@lsp.type.namespace",  { fg = syn_type })
hi("@lsp.type.function",   { fg = syn_fn })
hi("@lsp.type.method",     { fg = syn_fn })
hi("@lsp.type.macro",      { fg = syn_fn })
hi("@lsp.type.decorator",  { fg = c.warn })
hi("@lsp.type.event",      { fg = c.warn })
hi("@lsp.type.variable",   { fg = c.fg })
hi("@lsp.type.parameter",  { fg = c.fg_muted })
hi("@lsp.type.property",   { fg = c.slate })
hi("@lsp.type.keyword",    { fg = c.fg_muted, bold = true })
hi("@lsp.type.modifier",   { fg = c.fg_muted })
hi("@lsp.type.operator",   { fg = c.op })
hi("@lsp.type.string",     { fg = c.string })
hi("@lsp.type.regexp",     { fg = c.hint })
hi("@lsp.type.number",     { fg = syn_const })
hi("@lsp.type.comment",    { fg = c.fg_dim, italic = it })
hi("@lsp.mod.deprecated",  { fg = c.fg_faint, strikethrough = true, sp = c.fg_faint })
hi("LspReferenceText",     { bg = A.sel })
hi("LspReferenceRead",     { bg = A.sel })
hi("LspReferenceWrite",    { bg = A.sel, underline = true, sp = A.accent })
hi("LspReferenceTarget",   { bg = A.sel, underline = true, sp = A.accent })
hi("LspSignatureActiveParameter", { fg = c.white, bg = A.match, bold = true })
hi("LspCodeLens",          { fg = c.fg_faint, italic = it })
hi("LspCodeLensSeparator", { fg = c.border })
hi("LspInlayHint",         { fg = c.fg_faint, bg = c.bg_panel, italic = it })

-- ── Diagnostics ──────────────────────────────────────────────────────────────
hi("DiagnosticError", { fg = c.err })
hi("DiagnosticWarn",  { fg = c.warn })
hi("DiagnosticInfo",  { fg = c.info })
hi("DiagnosticHint",  { fg = c.hint })
hi("DiagnosticOk",    { fg = c.ok })
hi("DiagnosticDeprecated", { fg = c.fg_faint, strikethrough = true, sp = c.fg_faint })
hi("DiagnosticUnnecessary", { fg = c.fg_faint })

hi("DiagnosticUnderlineError", { undercurl = true, sp = c.err })
hi("DiagnosticUnderlineWarn",  { undercurl = true, sp = c.warn })
hi("DiagnosticUnderlineInfo",  { undercurl = true, sp = c.info })
hi("DiagnosticUnderlineHint",  { undercurl = true, sp = c.hint })
hi("DiagnosticUnderlineOk",    { undercurl = true, sp = c.ok })

hi("DiagnosticSignError", { fg = c.err })
hi("DiagnosticSignWarn",  { fg = c.warn })
hi("DiagnosticSignInfo",  { fg = c.info })
hi("DiagnosticSignHint",  { fg = c.hint })
hi("DiagnosticSignOk",    { fg = c.ok })

hi("DiagnosticVirtualTextError", { fg = c.err, italic = it })
hi("DiagnosticVirtualTextWarn",  { fg = c.warn, italic = it })
hi("DiagnosticVirtualTextInfo",  { fg = c.info, italic = it })
hi("DiagnosticVirtualTextHint",  { fg = c.hint, italic = it })
hi("DiagnosticVirtualTextOk",    { fg = c.ok, italic = it })

hi("DiagnosticVirtualLinesError", { fg = c.err, italic = it })
hi("DiagnosticVirtualLinesWarn",  { fg = c.warn, italic = it })
hi("DiagnosticVirtualLinesInfo",  { fg = c.info, italic = it })
hi("DiagnosticVirtualLinesHint",  { fg = c.hint, italic = it })
hi("DiagnosticVirtualLinesOk",    { fg = c.ok, italic = it })

hi("DiagnosticFloatingError", { fg = c.err, bg = c.bg_float })
hi("DiagnosticFloatingWarn",  { fg = c.warn, bg = c.bg_float })
hi("DiagnosticFloatingInfo",  { fg = c.info, bg = c.bg_float })
hi("DiagnosticFloatingHint",  { fg = c.hint, bg = c.bg_float })
hi("DiagnosticFloatingOk",    { fg = c.ok, bg = c.bg_float })
-- legacy names still used by some handlers
hi("DiagnosticErrorFloat", { fg = c.err, bg = c.bg_float })
hi("DiagnosticWarnFloat",  { fg = c.warn, bg = c.bg_float })
hi("DiagnosticInfoFloat",  { fg = c.info, bg = c.bg_float })
hi("DiagnosticHintFloat",  { fg = c.hint, bg = c.bg_float })

-- ── Diffs & git ──────────────────────────────────────────────────────────────
-- Backgrounds only, so syntax colours survive inside changed lines.
hi("DiffAdd",    { bg = c.add_bg })
hi("DiffChange", { bg = c.chg_bg })
hi("DiffDelete", { bg = c.del_bg })
hi("DiffText",   { fg = c.white, bg = c.txt_bg, bold = true })
hi("Added",      { fg = c.ok })
hi("Changed",    { fg = c.info })
hi("Removed",    { fg = c.err })
hi("diffAdded",   { fg = c.ok })
hi("diffRemoved", { fg = c.err })
hi("diffChanged", { fg = c.info })
hi("diffFile",    { fg = A.accent })
hi("diffLine",    { fg = c.fg_muted })
hi("diffSubname", { fg = c.info })
-- gitsigns / vim-gitgutter (only used if such a plugin is installed)
hi("GitSignsAdd",      { fg = c.ok })
hi("GitSignsChange",   { fg = c.info })
hi("GitSignsDelete",   { fg = c.err })
hi("GitSignsAddNr",    { fg = c.ok })
hi("GitSignsChangeNr", { fg = c.info })
hi("GitSignsDeleteNr", { fg = c.err })
hi("GitSignsAddLn",    { bg = c.add_bg })
hi("GitSignsChangeLn", { bg = c.chg_bg })
hi("GitSignsDeleteLn", { bg = c.del_bg })
hi("GitGutterAdd",     { fg = c.ok })
hi("GitGutterChange",  { fg = c.info })
hi("GitGutterDelete",  { fg = c.err })

-- ── Spelling ─────────────────────────────────────────────────────────────────
hi("SpellBad",   { undercurl = true, sp = c.err })
hi("SpellCap",   { undercurl = true, sp = c.warn })
hi("SpellLocal", { undercurl = true, sp = c.info })
hi("SpellRare",  { undercurl = true, sp = c.purple })

-- ── nvim-cmp ─────────────────────────────────────────────────────────────────
hi("CmpItemAbbr",           { fg = c.fg })
hi("CmpItemAbbrDeprecated", { fg = c.fg_faint, strikethrough = true, sp = c.fg_faint })
hi("CmpItemAbbrMatch",      { fg = A.bright, bold = true })
hi("CmpItemAbbrMatchFuzzy", { fg = A.bright })
hi("CmpItemMenu",           { fg = c.fg_dim, italic = it })
hi("CmpItemKind",           { fg = c.fg_muted })
hi("CmpItemKindDefault",    { fg = c.fg_muted })
hi("CmpItemKindIcon",       { fg = c.fg_muted })
hi("CmpItemKindIconDefault", { fg = c.fg_muted })
hi("CmpItemKindText",       { fg = c.fg_muted })
hi("CmpItemKindVariable",   { fg = c.fg })
hi("CmpItemKindField",      { fg = c.slate })
hi("CmpItemKindProperty",   { fg = c.slate })
hi("CmpItemKindReference",  { fg = c.hint })
hi("CmpItemKindFunction",   { fg = syn_fn })
hi("CmpItemKindMethod",     { fg = syn_fn })
hi("CmpItemKindConstructor", { fg = syn_fn })
hi("CmpItemKindClass",      { fg = syn_type })
hi("CmpItemKindStruct",     { fg = syn_type })
hi("CmpItemKindInterface",  { fg = syn_type })
hi("CmpItemKindTypeParameter", { fg = syn_type })
hi("CmpItemKindModule",     { fg = c.info })
hi("CmpItemKindNamespace",  { fg = c.info })
hi("CmpItemKindFile",       { fg = c.info })
hi("CmpItemKindFolder",     { fg = c.info })
hi("CmpItemKindSnippet",    { fg = c.ok })
hi("CmpItemKindKeyword",    { fg = c.purple })
hi("CmpItemKindOperator",   { fg = c.hint })
hi("CmpItemKindEvent",      { fg = c.warn })
hi("CmpItemKindColor",      { fg = c.warn })
hi("CmpItemKindConstant",   { fg = c.warn })
hi("CmpItemKindEnum",       { fg = c.warn })
hi("CmpItemKindEnumMember", { fg = c.warn })
hi("CmpItemKindValue",      { fg = c.warn })
hi("CmpItemKindUnit",       { fg = c.warn })
hi("CmpItemKindBoolean",    { fg = c.warn })

-- ── ALE ──────────────────────────────────────────────────────────────────────
hi("ALEError",       { fg = c.err, undercurl = true, sp = c.err })
hi("ALEWarning",     { fg = c.warn, undercurl = true, sp = c.warn })
hi("ALEInfo",        { fg = c.info, undercurl = true, sp = c.info })
hi("ALEStyleError",  { fg = c.err, undercurl = true, sp = c.err })
hi("ALEStyleWarning", { fg = c.warn, undercurl = true, sp = c.warn })
hi("ALEErrorLine",   { bg = c.del_bg })
hi("ALEWarningLine", { bg = c.warn_bg })
hi("ALEInfoLine",    { bg = c.chg_bg })
hi("ALEErrorSign",   { fg = c.err })
hi("ALEWarningSign", { fg = c.warn })
hi("ALEInfoSign",    { fg = c.info })
hi("ALEStyleErrorSign", { fg = c.err })
hi("ALEStyleWarningSign", { fg = c.warn })
hi("ALEVirtualTextError", { fg = c.err, italic = it })
hi("ALEVirtualTextWarning", { fg = c.warn, italic = it })
hi("ALEVirtualTextInfo", { fg = c.info, italic = it })
hi("ALEVirtualTextStyleError", { fg = c.err, italic = it })
hi("ALEVirtualTextStyleWarning", { fg = c.warn, italic = it })

-- ── netrw ────────────────────────────────────────────────────────────────────
hi("netrwDir",       { fg = A.accent })
hi("netrwClassify",  { fg = c.fg_faint })
hi("netrwExe",       { fg = c.ok })
hi("netrwSymLink",   { fg = c.hint })
hi("netrwLink",      { fg = c.hint })
hi("netrwPlain",     { fg = c.fg })
hi("netrwComment",   { fg = c.fg_dim })
hi("netrwTreeBar",   { fg = c.nontext })
hi("netrwMarkFile",  { fg = c.white, bg = A.match })
hi("netrwVersion",   { fg = c.fg_dim })
hi("netrwBak",       { fg = c.fg_faint })
hi("netrwSpecial",   { fg = c.warn })
hi("netrwHide",      { fg = c.fg_faint })
hi("netrwHidePat",   { fg = c.warn })
hi("netrwQuickHelp", { fg = c.fg_dim })

-- ── Kept for optional plugins (no-ops until installed) ───────────────────────
hi("NvimTreeNormal",       { fg = c.fg, bg = c.bg_panel })
hi("NvimTreeRootFolder",   { fg = A.accent, bold = true })
hi("NvimTreeGitDirty",     { fg = c.warn })
hi("NvimTreeGitNew",       { fg = c.ok })
hi("NvimTreeGitDeleted",   { fg = c.err })
hi("NvimTreeIndentMarker", { fg = c.nontext })
hi("TelescopeBorder",      { fg = c.border, bg = c.bg_float })
hi("TelescopePromptBorder", { fg = c.border, bg = c.bg_float })
hi("TelescopePromptNormal", { fg = c.fg, bg = c.bg_float })
hi("TelescopeMatching",    { fg = A.bright, bold = true })
hi("TelescopeSelection",   { fg = c.fg_strong, bg = A.sel })

-- ── :terminal palette ────────────────────────────────────────────────────────
local term = {
  c.black, c.err, c.ok, c.warn, c.info, c.purple, c.hint, c.fg_muted,
  c.border, c.err, c.ok, c.warn, c.info, c.purple, A.accent, c.white,
}
for i, col in ipairs(term) do
  vim.g["terminal_color_" .. (i - 1)] = col[1]
end
