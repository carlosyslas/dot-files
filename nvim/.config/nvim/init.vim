set nocompatible
set showmatch
set ignorecase
set hlsearch
set tabstop=2
set softtabstop=4
set expandtab
set shiftwidth=4
set autoindent
set nu
set wildmode=longest,list
set invspell
set background=dark
set autochdir
set nohlsearch
set termguicolors

call plug#begin()
Plug 'tpope/vim-fugitive'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
Plug 'junegunn/fzf.vim'
Plug 'itchyny/lightline.vim'
Plug 'airblade/vim-rooter'
Plug 'kevinoid/vim-jsonc'
Plug 'jparise/vim-graphql'
Plug 'leafgarland/typescript-vim'
Plug 'preservim/nerdcommenter'
Plug 'preservim/nerdtree'
Plug 'christoomey/vim-tmux-navigator'
Plug 'morhetz/gruvbox'
call plug#end()

" Coc config
let g:coc_global_extensions = [
    \ 'coc-tsserver',
    \ 'coc-snippets',
    \ 'coc-eslint',
    \ 'coc-prettier',
    \ 'coc-pairs',
    \ ]

"" Nerd commenter
filetype plugin on
nmap <C-_> <Plug>NERDCommenterToggle
vmap <C-_> <Plug>NERDCommenterToggle<CR>gv

nnoremap <C-p> :GFiles<CR>
nnoremap <C-t> :NERDTree<CR>

colorscheme gruvbox

