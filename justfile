alias b := build
alias c := check
alias d := dev
alias e := example
alias r := run
alias t := test
alias tdb := test_sqlserver
alias tr := test_rpc
alias td := test_derive

build:
  cargo build 

check:
  cargo check 

run:
  cargo run 

dev:
  cargo run  --example quick_dev

example:
  cargo run --example sqlserver

test:
  cargo test 

test_sqlserver:
  cargo test  -p lib-sqlserver

test_rpc:
  cargo test  -p lib-rpc

test_derive:
  cargo test  -p lib-derive
  
