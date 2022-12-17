colorscheme theme

set number
set relativenumber
set nowrap
set mouse=
set autoread

autocmd VimEnter * COQnow -s

call neomake#configure#automake('nrwi', 100)

let g:neomake_open_list = 2

let NERDTreeMinimalUI = 1
