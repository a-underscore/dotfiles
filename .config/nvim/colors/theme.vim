let gray_900 = 233
let gray_800 = 239
let gray_700 = 243
let gray_600 = 246
let gray_500 = 249
let gray_400 = 252
let zinc_gray_900 = 59
let dark_gray = 16
let teal = 36

highlight clear

syntax reset

let g:colors_name = "theme"

exec "hi Normal ctermfg=".gray_400." ctermbg=".gray_900
exec "hi Comment ctermfg=".gray_800
exec "hi Constant ctermfg=".teal
exec "hi Character ctermfg=".gray_700
exec "hi Identifier ctermfg=".gray_400." cterm=none"
exec "hi Statement ctermfg=".gray_500
exec "hi PreProc ctermfg=".teal
exec "hi Type ctermfg=".gray_700
exec "hi Special ctermfg=".gray_700
exec "hi Underlined ctermfg=".gray_500
exec "hi Error ctermfg=".teal." ctermbg=".zinc_gray_900
exec "hi Todo ctermfg=".teal." ctermbg=".zinc_gray_900
exec "hi Function ctermfg=".teal
exec "hi ColorColumn ctermbg=".zinc_gray_900
exec "hi Conceal ctermfg=".gray_800
exec "hi Cursor ctermfg=".gray_900
exec "hi CursorColumn ctermbg=".zinc_gray_900
exec "hi CursorLine cterm=none ctermbg=".dark_gray
exec "hi Directory ctermfg=".gray_500
exec "hi DiffAdd ctermfg=".teal." ctermbg=".zinc_gray_900
exec "hi DiffChange ctermfg=".gray_400." ctermbg=".zinc_gray_900
exec "hi DiffDelete ctermfg=".teal." ctermbg=".zinc_gray_900
exec "hi DiffText ctermfg=".gray_400." ctermbg=".gray_800
exec "hi ErrorMsg ctermfg=".gray_400." ctermbg=".teal
exec "hi VertSplit ctermfg=".gray_400." ctermbg=".gray_400
exec "hi Folded ctermfg=".gray_600." ctermbg=".zinc_gray_900
exec "hi FoldColumn ctermfg=".gray_600." ctermbg=".zinc_gray_900
exec "hi SignColumn ctermbg=".gray_900
exec "hi IncSearch ctermfg=".gray_900." ctermbg=".gray_400
exec "hi LineNr ctermfg=".gray_900." ctermbg=".gray_400
exec "hi CursorLineNr cterm=bold ctermfg=".gray_400." ctermbg=".dark_gray
exec "hi MatchParen ctermbg=".gray_800
exec "hi MoreMsg ctermfg=".gray_900." ctermbg=".gray_700
exec "hi NonText ctermfg=".zinc_gray_900." ctermbg=".gray_900
exec "hi Pmenu ctermfg=".gray_900." ctermbg=".gray_400
exec "hi PmenuSel cterm=bold ctermfg=".gray_400." ctermbg=".dark_gray
exec "hi PmenuSbar ctermfg=".teal." ctermbg=".zinc_gray_900
exec "hi PmenuThumb ctermfg=".teal." ctermbg=".teal
exec "hi Question ctermfg=".gray_400." ctermbg=".zinc_gray_900
exec "hi Search ctermfg=".gray_900." ctermbg=".gray_400
exec "hi SpecialKey ctermfg=".gray_700." ctermbg=".gray_900
exec "hi SpellBad ctermfg=".teal." ctermbg=NONE cterm=undercurl"
exec "hi SpellCap ctermfg=".gray_400." ctermbg=NONE cterm=undercurl"
exec "hi SpellLocal ctermfg=".gray_700
exec "hi SpellRare ctermfg=".teal
exec "hi StatusLine cterm=bold ctermfg=".gray_400." ctermbg=".dark_gray
exec "hi TabLine ctermfg=".gray_900." ctermbg=".gray_400." cterm=none"
exec "hi TabLineFill ctermfg=".gray_400
exec "hi TabLineSel cterm=bold ctermfg=".gray_400." ctermbg=".dark_gray
exec "hi Title ctermfg=".gray_500
exec "hi WarningMsg ctermfg=".teal

unlet gray_900 zinc_gray_900 gray_800 gray_700 gray_600 gray_500 gray_400 teal
