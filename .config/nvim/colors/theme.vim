let cterm_gray_900 = 233
let cterm_gray_800 = 239
let cterm_gray_700 = 243
let cterm_gray_600 = 246
let cterm_gray_500 = 249
let cterm_gray_400 = 252
let cterm_zinc_gray_900 = 59
let cterm_teal = 36

highlight clear

syntax reset

let g:colors_name = "theme"

exec "hi Normal ctermfg=".cterm_gray_400." ctermbg=".cterm_gray_900
exec "hi Comment ctermfg=".cterm_gray_800
exec "hi Constant ctermfg=".cterm_teal
exec "hi Character ctermfg=".cterm_gray_700
exec "hi Identifier ctermfg=".cterm_gray_400." cterm=none"
exec "hi Statement ctermfg=".cterm_gray_500
exec "hi PreProc ctermfg=".cterm_teal
exec "hi Type ctermfg=".cterm_gray_700
exec "hi Special ctermfg=".cterm_gray_700
exec "hi Underlined ctermfg=".cterm_gray_500
exec "hi Error ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900
exec "hi Todo ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900
exec "hi Function ctermfg=".cterm_teal
exec "hi ColorColumn ctermbg=".cterm_zinc_gray_900
exec "hi Conceal ctermfg=".cterm_gray_800
exec "hi Cursor ctermfg=".cterm_gray_900
exec "hi CursorColumn ctermbg=".cterm_zinc_gray_900
exec "hi CursorLine cterm=none ctermbg=".cterm_zinc_gray_900
exec "hi Directory ctermfg=".cterm_gray_500
exec "hi DiffAdd ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900
exec "hi DiffChange ctermfg=".cterm_gray_400." ctermbg=".cterm_zinc_gray_900
exec "hi DiffDelete ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900
exec "hi DiffText ctermfg=".cterm_gray_400." ctermbg=".cterm_gray_800
exec "hi ErrorMsg ctermfg=".cterm_gray_400." ctermbg=".cterm_teal
exec "hi VertSplit ctermfg=".cterm_gray_400." ctermbg=".cterm_gray_400
exec "hi Folded ctermfg=".cterm_gray_600." ctermbg=".cterm_zinc_gray_900
exec "hi FoldColumn ctermfg=".cterm_gray_600." ctermbg=".cterm_zinc_gray_900
exec "hi SignColumn ctermbg=".cterm_gray_900
exec "hi IncSearch ctermfg=".cterm_gray_900." ctermbg=".cterm_gray_400
exec "hi LineNr ctermfg=".cterm_zinc_gray_900." ctermbg=".cterm_gray_900
exec "hi CursorLineNr cterm=none ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900
exec "hi MatchParen ctermbg=".cterm_gray_800
exec "hi MoreMsg ctermfg=".cterm_gray_900." ctermbg=".cterm_gray_700
exec "hi NonText ctermfg=".cterm_zinc_gray_900." ctermbg=".cterm_gray_900
exec "hi Pmenu ctermfg=".cterm_gray_400." ctermbg=".cterm_zinc_gray_900
exec "hi PmenuSel ctermfg=".cterm_gray_700." ctermbg=".cterm_zinc_gray_900
exec "hi PmenuSbar ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900
exec "hi PmenuThumb ctermfg=".cterm_teal." ctermbg=".cterm_gray_800
exec "hi Question ctermfg=".cterm_gray_400." ctermbg=".cterm_zinc_gray_900
exec "hi Search ctermfg=".cterm_gray_900." ctermbg=".cterm_gray_400
exec "hi SpecialKey ctermfg=".cterm_gray_700." ctermbg=".cterm_gray_900
exec "hi SpellBad ctermfg=".cterm_teal." ctermbg=NONE cterm=undercurl"
exec "hi SpellCap ctermfg=".cterm_gray_400." ctermbg=NONE cterm=undercurl"
exec "hi SpellLocal ctermfg=".cterm_gray_700
exec "hi SpellRare ctermfg=".cterm_teal
exec "hi StatusLine ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900." cterm=none"
exec "hi TabLine ctermfg=".cterm_gray_900." ctermbg=".cterm_gray_400
exec "hi TabLineFill ctermbg=".cterm_zinc_gray_900
exec "hi TabLineSel ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900
exec "hi Title ctermfg=".cterm_gray_500
exec "hi Visual ctermbg=".cterm_zinc_gray_900
exec "hi VisualNOS ctermfg=".cterm_teal." ctermbg=".cterm_zinc_gray_900
exec "hi WarningMsg ctermfg=".cterm_teal
exec "hi WildMenu ctermfg=".cterm_gray_700." ctermbg=".cterm_zinc_gray_900

unlet cterm_gray_900 cterm_zinc_gray_900 cterm_gray_800 cterm_gray_700 cterm_gray_600 cterm_gray_500 cterm_gray_400 cterm_teal
