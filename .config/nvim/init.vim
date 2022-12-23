source ~/.config/nvim/plugs.vim

colorscheme theme

set number
set relativenumber
set nowrap
set mouse=
set cursorline
set autoread
set autowrite
set guifont="pango:RobotoMono 12"
set completeopt=menu,menuone,preview,noselect,noinsert

call neomake#configure#automake('nrwi')

let g:neomake_open_list = 2

lua << EOF

require('tabline').setup({})

EOF
