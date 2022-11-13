"
" plugins for coding in rust
"
silent! packadd! vim-textobj-user
silent! packadd! vim-textobj-function
silent! packadd! vim-textobj-function-rust

let g:ale_rust_rustfmt_options = '+nightly'

let g:cargomutants_ale_enabled = 1
let g:cargomutants_cmd_opts = ''

let g:cargomutants_error_type_map = {
      \ 'missed': 'E',
      \ 'unviable': v:null,
      \ 'timeout': 'I',
      \ }

