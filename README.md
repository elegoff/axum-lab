-  cargo run
- browse http://localhost:8080/hello

alias cw='cargo watch -q -c -w src/ -x run'
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

