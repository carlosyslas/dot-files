set nocompatible
set showmatch
set ignorecase
set hlsearch
set tabstop=2
set softtabstop=4
set expandtab
set shiftwidth=4
set autoindent
set number
set wildmode=longest,list
set invspell
set background=dark
set autochdir

call plug#begin()
Plug 'tpope/vim-fugitive'
Plug 'aonemd/kuroi.vim'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
Plug 'junegunn/fzf.vim'
Plug 'itchyny/lightline.vim'
call plug#end()

" Coc config
let g:coc_global_extensions = [
    \ 'coc-tsserver',
    \ 'coc-snippets',
    \ 'coc-eslint',
    \ 'coc-prettier',
    \ 'coc-pairs',
    \ ]

colorscheme kuroi
