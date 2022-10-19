colorscheme theme

set number
set relativenumber
set mouse=

autocmd VimEnter * COQnow -s

call neomake#configure#automake('nrwi', 500)

let g:neomake_open_list = 2
let NERDTreeMinimalUI = 1
