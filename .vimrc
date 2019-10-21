call plug#begin()
Plug 'dracula/vim',{'as':'dracula'}
Plug 'sheerun/vim-polyglot'
Plug 'itchyny/lightline.vim'
Plug 'airblade/vim-gitgutter'
call plug#end()

set number
set laststatus=2
set termguicolors
colorscheme dracula
let g:lightline = { 'colorscheme': 'dracula' }
