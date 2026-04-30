
-- theme2.lua — Neovim colorscheme (Lua format)
-- Compatible with Neovim 0.8+
-- Place in ~/.config/nvim/colors/theme2.lua
-- Usage: :colorscheme theme2
 
vim.cmd("highlight clear")
if vim.fn.exists("syntax_on") then
  vim.cmd("syntax reset")
end
 
vim.g.colors_name = "theme2"
 
-- Palette (cterm indices mapped to hex equivalents)
local c = {
  gray_900  = { cterm = 233, hex = "#121212" }, -- Normal bg
  dark_gray = { cterm = 234, hex = "#1c1c1c" }, -- Gutter/panel bg
  light_gray = { cterm = 239, hex = "#4e4e4e" }, -- Folds
  black = { cterm = 16, hex = "#000000" }, -- Cursor line
  gray_800  = { cterm = 242, hex = "#6c6c6c" }, -- Comments
  gray_700  = { cterm = 246, hex = "#949494" }, -- Type, Special, Character
  gray_500  = { cterm = 244, hex = "#808080" }, -- Statement, Underlined
  gray_400  = { cterm = 249, hex = "#b2b2b2" }, -- Normal fg, Identifier
  hi        = { cterm = 6,   hex = "#008080" }, -- Accent: Function, Constant, PreProc
  none      = "NONE",
}
 
-- Helper: set a highlight group
---@param group string
---@param opts table
local function hi(group, opts)
  local o = {
    fg        = opts.fg and opts.fg.hex or nil,
    bg        = opts.bg and opts.bg.hex or nil,
    ctermfg   = opts.fg and opts.fg.cterm or nil,
    ctermbg   = opts.bg and opts.bg.cterm or nil,
    bold      = opts.bold or nil,
    underline = opts.underline or nil,
    sp        = opts.sp and opts.sp.hex or nil,
  }
  vim.api.nvim_set_hl(0, group, o)
end
 
-- ── Core syntax ─────────────────────────────────────────────────────────────
hi("Normal",     { fg = c.gray_400,  bg = c.gray_900 })
hi("Comment",    { fg = c.gray_800 })
hi("Constant",   { fg = c.hi })
hi("Character",  { fg = c.gray_700 })
hi("Identifier", { fg = c.gray_400 })
hi("Statement",  { fg = c.gray_500 })
hi("PreProc",    { fg = c.hi })
hi("Type",       { fg = c.gray_700 })
hi("Special",    { fg = c.gray_700 })
hi("Underlined", { fg = c.gray_500, underline = true })
hi("Error",      { fg = c.hi,       bg = c.light_gray })
hi("Todo",       { fg = c.hi,       bg = c.light_gray })
hi("Function",   { fg = c.hi })
 
-- ── UI chrome ────────────────────────────────────────────────────────────────
hi("ColorColumn",  { bg = c.light_gray })
hi("Conceal",      { fg = c.gray_800 })
hi("Cursor",       { fg = c.gray_900 })
hi("CursorColumn", { bg = c.light_gray })
hi("CursorLine",   { bg = c.black })
hi("Directory",    { fg = c.gray_500 })
 
-- ── Diff ─────────────────────────────────────────────────────────────────────
hi("DiffAdd",    { fg = c.hi,       bg = c.light_gray })
hi("DiffChange", { fg = c.gray_400, bg = c.light_gray })
hi("DiffDelete", { fg = c.hi,       bg = c.light_gray })
hi("DiffText",   { fg = c.gray_400, bg = c.gray_800 })
 
-- ── Messages & prompts ───────────────────────────────────────────────────────
hi("ErrorMsg",   { fg = c.gray_400, bg = c.hi })
hi("MoreMsg",    { fg = c.gray_900, bg = c.gray_700 })
hi("Question",   { fg = c.gray_400, bg = c.light_gray })
hi("WarningMsg", { fg = c.hi })
hi("Title",      { fg = c.gray_500 })
 
-- ── Splits & borders ─────────────────────────────────────────────────────────
hi("VertSplit",  { fg = c.dark_gray, bg = c.dark_gray })
hi("WinSeparator", { fg = c.dark_gray, bg = c.dark_gray }) -- Neovim 0.9+ alias
 
-- ── Folds ────────────────────────────────────────────────────────────────────
hi("Folded",     { fg = c.gray_900, bg = c.light_gray })
hi("FoldColumn", { fg = c.gray_900, bg = c.light_gray })
 
-- ── Sign column & line numbers ───────────────────────────────────────────────
hi("SignColumn",     { bg = c.dark_gray })
hi("LineNr",         { fg = c.light_gray, bg = c.dark_gray })
hi("CursorLineNr",   { fg = c.gray_400,   bg = c.light_gray, bold = true })
 
-- ── Search ───────────────────────────────────────────────────────────────────
hi("IncSearch", { fg = c.gray_900, bg = c.gray_400 })
hi("Search",    { fg = c.gray_900, bg = c.gray_400 })
 
-- ── Popup menu ───────────────────────────────────────────────────────────────
hi("Pmenu",      { fg = c.light_gray, bg = c.dark_gray })
hi("PmenuSel",   { fg = c.gray_400,   bg = c.light_gray, bold = true })
hi("PmenuSbar",  { bg = c.dark_gray })
hi("PmenuThumb", { bg = c.light_gray })
 
-- ── Visual ───────────────────────────────────────────────────────────────────
hi("Visual",     { bg = c.light_gray, bold = true })
 
-- ── Matching & special keys ──────────────────────────────────────────────────
hi("MatchParen", { bg = c.gray_800 })
hi("NonText",    { fg = c.light_gray, bg = c.gray_900 })
hi("SpecialKey", { fg = c.gray_700,   bg = c.gray_900 })
 
-- ── Spell ────────────────────────────────────────────────────────────────────
hi("SpellBad",   { fg = c.hi,        underline = true })
hi("SpellCap",   { fg = c.gray_400,  underline = true })
hi("SpellLocal", { fg = c.gray_700 })
hi("SpellRare",  { fg = c.hi })
 
-- ── Status & tab lines ───────────────────────────────────────────────────────
hi("StatusLine",   { fg = c.gray_400,   bg = c.light_gray, bold = true })
hi("StatusLineNC", { fg = c.dark_gray,  bg = c.light_gray })
hi("TabLine",      { fg = c.light_gray, bg = c.dark_gray })
hi("TabLineFill",  { fg = c.gray_900 })
hi("TabLineSel",   { fg = c.gray_400,   bg = c.light_gray, bold = true })
 
-- ── Treesitter (Neovim 0.8+ @-prefixed groups) ───────────────────────────────
hi("@comment",            { fg = c.gray_800 })
hi("@constant",           { fg = c.hi })
hi("@constant.builtin",   { fg = c.hi })
hi("@function",           { fg = c.hi })
hi("@function.builtin",   { fg = c.hi })
hi("@function.call",      { fg = c.hi })
hi("@keyword",            { fg = c.gray_500 })
hi("@keyword.function",   { fg = c.gray_500 })
hi("@keyword.return",     { fg = c.gray_500 })
hi("@operator",           { fg = c.gray_500 })
hi("@parameter",          { fg = c.gray_400 })
hi("@string",             { fg = c.gray_700 })
hi("@string.escape",      { fg = c.hi })
hi("@type",               { fg = c.gray_700 })
hi("@type.builtin",       { fg = c.gray_700 })
hi("@variable",           { fg = c.gray_400 })
hi("@variable.builtin",   { fg = c.gray_500 })
hi("@punctuation",        { fg = c.gray_400 })
hi("@punctuation.bracket",{ fg = c.gray_400 })
hi("@punctuation.delimiter",{ fg = c.gray_400 })
hi("@namespace",          { fg = c.gray_700 })
hi("@tag",                { fg = c.gray_500 })
hi("@tag.attribute",      { fg = c.gray_700 })
hi("@tag.delimiter",      { fg = c.gray_400 })
 
-- ── LSP semantic tokens ───────────────────────────────────────────────────────
hi("@lsp.type.function",  { fg = c.hi })
hi("@lsp.type.method",    { fg = c.hi })
hi("@lsp.type.variable",  { fg = c.gray_400 })
hi("@lsp.type.parameter", { fg = c.gray_400 })
hi("@lsp.type.keyword",   { fg = c.gray_500 })
hi("@lsp.type.type",      { fg = c.gray_700 })
hi("@lsp.type.namespace", { fg = c.gray_700 })
hi("@lsp.type.string",    { fg = c.gray_700 })
hi("@lsp.type.number",    { fg = c.hi })
hi("@lsp.type.comment",   { fg = c.gray_800 })
 
-- ── Diagnostic ────────────────────────────────────────────────────────────────
hi("DiagnosticError",          { fg = c.hi })
hi("DiagnosticWarn",           { fg = c.gray_700 })
hi("DiagnosticInfo",           { fg = c.gray_500 })
hi("DiagnosticHint",           { fg = c.gray_800 })
hi("DiagnosticUnderlineError", { underline = true, sp = c.hi })
hi("DiagnosticUnderlineWarn",  { underline = true, sp = c.gray_700 })
hi("DiagnosticUnderlineInfo",  { underline = true, sp = c.gray_500 })
hi("DiagnosticUnderlineHint",  { underline = true, sp = c.gray_800 })
 
-- ── NvimTree / neo-tree sidebar ───────────────────────────────────────────────
hi("NvimTreeNormal",       { fg = c.gray_400, bg = c.dark_gray })
hi("NvimTreeRootFolder",   { fg = c.hi })
hi("NvimTreeGitDirty",     { fg = c.gray_700 })
hi("NvimTreeGitNew",       { fg = c.hi })
hi("NvimTreeIndentMarker", { fg = c.light_gray })
 
-- ── Telescope ────────────────────────────────────────────────────────────────
hi("TelescopeBorder",        { fg = c.light_gray })
hi("TelescopePromptBorder",  { fg = c.gray_500 })
hi("TelescopeSelection",     { fg = c.gray_400, bg = c.light_gray })
hi("TelescopeMatching",      { fg = c.hi })
