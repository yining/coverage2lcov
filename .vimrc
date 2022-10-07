"
" plugins for coding in rust
"
silent! packadd! vim-textobj-user
silent! packadd! vim-textobj-function
silent! packadd! vim-textobj-function-rust

let g:ale_rust_rustfmt_options = '+nightly'

let g:cargomutants_ale_enabled = 1

let g:ale_linters['rust'] += ['cargomutants']
let g:cargomutants_ale_msg_pattern = '\v^\s*\[.*\]\s*(.{-})\s*$'
let g:cargomutants_ale_msg_idx = 1
let g:cargomutants_ale_source_name = 'mutants'
" let g:cargomutants_output_dir = 'foo'
