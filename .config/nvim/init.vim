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

let g:netrw_banner=0

call neomake#configure#automake('nrwi')

let g:neomake_open_list = 2

lua << EOF

require('tabline').setup({})

EOF
