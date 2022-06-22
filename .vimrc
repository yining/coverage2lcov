"
" plugins for coding in rust
"
silent! packadd! vim-textobj-user
silent! packadd! vim-textobj-function
silent! packadd! vim-textobj-function-rust

let g:ale_rust_rustfmt_options = '+nightly'

