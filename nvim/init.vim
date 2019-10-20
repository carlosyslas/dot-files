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
call plug#end()

colorscheme kuroi
