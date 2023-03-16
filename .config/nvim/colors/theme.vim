let gray_900 = 233
let gray_600 = 235
let gray_800 = 242
let gray_700 = 246
let gray_500 = 244
let gray_400 = 249
let light_gray = 239
let dark_gray = 234
let teal = 37

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
exec "hi Error ctermfg=".teal." ctermbg=".light_gray
exec "hi Todo ctermfg=".teal." ctermbg=".light_gray
exec "hi Function ctermfg=".teal
exec "hi ColorColumn ctermbg=".light_gray
exec "hi Conceal ctermfg=".gray_800
exec "hi Cursor ctermfg=".gray_900
exec "hi CursorColumn ctermbg=".light_gray
exec "hi CursorLine cterm=none ctermbg=".light_gray
exec "hi Directory ctermfg=".gray_500
exec "hi DiffAdd ctermfg=".teal." ctermbg=".light_gray
exec "hi DiffChange ctermfg=".gray_400." ctermbg=".light_gray
exec "hi DiffDelete ctermfg=".teal." ctermbg=".light_gray
exec "hi DiffText ctermfg=".gray_400." ctermbg=".gray_800
exec "hi ErrorMsg ctermfg=".gray_400." ctermbg=".teal
exec "hi VertSplit cterm=bold ctermfg=".dark_gray." ctermbg=".dark_gray." cterm=none"
exec "hi Folded ctermfg=".gray_900." ctermbg=".light_gray
exec "hi FoldColumn ctermfg=".gray_900." ctermbg=".light_gray
exec "hi SignColumn ctermbg=".dark_gray
exec "hi IncSearch ctermfg=".gray_900." ctermbg=".gray_400
exec "hi LineNr ctermfg=".light_gray." ctermbg=".dark_gray
exec "hi CursorLineNr cterm=bold ctermfg=".gray_400." ctermbg=".light_gray
exec "hi MatchParen ctermbg=".gray_800
exec "hi MoreMsg ctermfg=".gray_900." ctermbg=".gray_700
exec "hi NonText ctermfg=".light_gray." ctermbg=".gray_900
exec "hi Pmenu ctermfg=".light_gray." ctermbg=".dark_gray
exec "hi PmenuSel cterm=bold ctermfg=".gray_400." ctermbg=".light_gray
exec "hi Visual cterm=bold ctermbg=".light_gray
exec "hi PmenuSbar ctermbg=".dark_gray
exec "hi PmenuThumb ctermbg=".light_gray
exec "hi Question ctermfg=".gray_400." ctermbg=".light_gray
exec "hi Search ctermfg=".gray_900." ctermbg=".gray_400
exec "hi SpecialKey ctermfg=".gray_700." ctermbg=".gray_900
exec "hi SpellBad ctermfg=".teal." ctermbg=NONE cterm=underline"
exec "hi SpellCap ctermfg=".gray_400." ctermbg=NONE cterm=underline"
exec "hi SpellLocal ctermfg=".gray_700
exec "hi SpellRare ctermfg=".teal
exec "hi StatusLine cterm=bold ctermfg=".gray_400." ctermbg=".light_gray
exec "hi StatusLineNC ctermfg=".dark_gray." ctermbg="light_gray
exec "hi TabLine ctermfg=".light_gray." ctermbg=".dark_gray." cterm=none"
exec "hi TabLineFill ctermfg=".gray_900
exec "hi TabLineSel cterm=bold ctermfg=".gray_400." ctermbg=".light_gray
exec "hi Title ctermfg=".gray_500
exec "hi WarningMsg ctermfg=".teal

unlet gray_900 light_gray gray_800 gray_700 gray_500 gray_400 teal
