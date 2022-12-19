"Plugins"
source ~/.config/nvim/plugs.vim

colorscheme theme

set number
set relativenumber
set nowrap
set mouse=
set autoread

let g:neomake_open_list = 2

let NERDTreeMinimalUI = 1

lua << EOF

require('tabline').setup({
    show_index = true,
    show_modify = true,
    modify_indicator = '[+]',
})

EOF
