function! TabLabel(n)
	  let buflist = tabpagebuflist(a:n)
	  let winnr = tabpagewinnr(a:n)

	  return buflist[winnr - 1] . ') ' . bufname(buflist[winnr - 1])
endfunction

function! TabLine()
  let s = '' " complete tabline goes here

  for t in range(tabpagenr('$'))
    " Active tab gets its own chip; the modified marker keeps its own colour
    " so it stays visible on both active and inactive tabs.
    let s:grp    = (t + 1 == tabpagenr()) ? 'TabLineSel'    : 'TabLine'
    let s:modgrp = (t + 1 == tabpagenr()) ? 'TabLineModSel' : 'TabLineMod'
    let s .= '%#' . s:grp . '#'

    let s .= ' '
    let s .= '%' . (t + 1) . 'T'
    let s .= t + 1 . ' '
    let n = '' 
    let m = 0
    let bc = len(tabpagebuflist(t + 1))

    for b in tabpagebuflist(t + 1)
      if getbufvar( b, "&buftype" ) == 'help'
        let n .= '[H]' . fnamemodify( bufname(b), ':t:s/.txt$//' )
      elseif getbufvar( b, "&buftype" ) == 'quickfix'
        let n .= '[Q]'
      else
        let n .= pathshorten(bufname(b))
        "let n .= bufname(b)
      endif

      if getbufvar( b, "&modified" )
        let m += 1
      endif

      if bc > 1
        let n .= ' '
      endif
      let bc -= 1
    endfor

    if m > 0
      "let s .= '[' . m . '+]'
      let s .= '%#' . s:modgrp . '#+ %#' . s:grp . '#'
    endif

    if n == ''
      let s .= '[No Name]'
    else
      let s .= n
    endif

    let s .= ' '
  endfor

  let s .= '%#TabLineFill#%T'

  if tabpagenr('$') > 1
    let s .= '%=%#TabLineClose#%999XX'
  endif

  return s
endfunction

set tabline=%!TabLine()
