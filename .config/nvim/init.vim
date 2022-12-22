source ~/.config/nvim/plugs.vim

colorscheme theme

set number
set relativenumber
set nowrap
set mouse=
set autoread
set autowrite

lua << EOF

require('tabline').setup({})

EOF
