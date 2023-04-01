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
set completeopt-=preview

call neomake#configure#automake('nrwi')

let g:neomake_open_list = 2
let g:netrw_banner=0

lua << EOF

require('tabline').setup({})

EOF
