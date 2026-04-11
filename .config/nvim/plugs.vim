set rtp+="~/.config/nvim/colors"

call plug#begin()
	Plug 'mfussenegger/nvim-lint'
	Plug 'hrsh7th/cmp-nvim-lsp'
	Plug 'hrsh7th/cmp-buffer'
	Plug 'hrsh7th/cmp-path'
	Plug 'hrsh7th/cmp-cmdline'
	Plug 'hrsh7th/nvim-cmp'
	Plug 'hrsh7th/cmp-vsnip'
	Plug 'hrsh7th/vim-vsnip'
	Plug 'OmniSharp/omnisharp-vim'
	Plug 'dense-analysis/ale'
call plug#end()
