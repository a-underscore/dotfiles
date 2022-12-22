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

lua << EOF

require('tabline').setup({})

EOF
