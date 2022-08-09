set rtp+="~/.config/nvim/colors"

call plug#begin()
	Plug 'neovim/nvim-lspconfig'
	Plug 'ms-jpq/coq_nvim', {'branch': 'coq'}
	Plug 'ms-jpq/coq.artifacts', {'branch': 'artifacts'}
	Plug 'ms-jpq/coq.thirdparty', {'branch': '3p'}
	Plug 'preservim/nerdtree'
	Plug 'neomake/neomake'
	Plug 'sangdol/mintabline.vim'
call plug#end()

