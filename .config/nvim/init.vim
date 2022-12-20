"Plugins"
source ~/.config/nvim/plugs.vim

colorscheme theme

set number
set relativenumber
set nowrap
set mouse=
set autoread

lua << EOF

require('tabline').setup({
    show_index = true,
    show_modify = true,
    modify_indicator = '[+]',
})

EOF
