source ~/.config/nvim/plugs.vim

colorscheme theme

set number
set relativenumber
set nowrap
set mouse=
set cursorline
set autoread
set autowrite
set signcolumn=yes 
set shortmess=I

let g:netrw_banner=0

lua << EOF

require('tabline').setup({})

EOF
