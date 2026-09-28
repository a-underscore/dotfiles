source ~/.config/nvim/plugs.vim
source ~/.config/nvim/tabline.vim

" ── Appearance ──────────────────────────────────────────────────────────────
" Without termguicolors Neovim ignores theme2's hex palette and renders the
" 256-colour cterm approximations instead (the accent came out bright cyan).
if has('termguicolors') && !has('gui_running')
  if get(g:, 'theme2_truecolor', $COLORTERM =~# '\v^(truecolor|24bit)$' || $TERM =~# 'direct')
    set termguicolors
  endif
endif
set background=dark
if has('nvim-0.11')
  set winborder=rounded          " rounded borders on floats/hover/completion
endif
set fillchars=vert:│,fold:·   " thinner split line, quieter fold marker

colorscheme theme2

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
set showtabline=2

let g:netrw_banner=0
let g:ale_set_float = 1

lua << EOF
  -- Set up nvim-cmp.
  local cmp = require'cmp'

  cmp.setup({
    snippet = {
      -- REQUIRED - you must specify a snippet engine
      expand = function(args)
        vim.fn["vsnip#anonymous"](args.body) -- For `vsnip` users.
        -- require('luasnip').lsp_expand(args.body) -- For `luasnip` users.
        -- require('snippy').expand_snippet(args.body) -- For `snippy` users.
        -- vim.fn["UltiSnips#Anon"](args.body) -- For `ultisnips` users.
      end,
    },
    window = {
      -- Bordered popups: keep them on Normal/Float surfaces with a visible
      -- FloatBorder, and use the theme's teal selection for the chosen item.
      completion = cmp.config.window.bordered({
        winhighlight = 'Normal:NormalFloat,FloatBorder:FloatBorder,CursorLine:Visual,Search:None',
      }),
      documentation = cmp.config.window.bordered({
        winhighlight = 'Normal:NormalFloat,FloatBorder:FloatBorder,CursorLine:Visual,Search:None',
      }),
    },
    mapping = cmp.mapping.preset.insert({
      ['<C-b>'] = cmp.mapping.scroll_docs(-4),
      ['<C-f>'] = cmp.mapping.scroll_docs(4),
      ['<C-Space>'] = cmp.mapping.complete(),
      ['<C-e>'] = cmp.mapping.abort(),
      ['<CR>'] = cmp.mapping.confirm({ select = true }), -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
    }),
    sources = cmp.config.sources({
      { name = 'nvim_lsp' },
      { name = 'vsnip' },
    }, {
      { name = 'buffer' },
    })
  })

  cmp.setup.filetype('gitcommit', {
    sources = cmp.config.sources({
      { name = 'git' },
    }, {
      { name = 'buffer' },
    })
  })

  cmp.setup.cmdline({ '/', '?' }, {
    mapping = cmp.mapping.preset.cmdline(),
    sources = {
      { name = 'buffer' }
    }
  })

  cmp.setup.cmdline(':', {
    mapping = cmp.mapping.preset.cmdline(),
    sources = cmp.config.sources({
      { name = 'path' }
    }, {
      { name = 'cmdline' }
    })
  })

EOF
