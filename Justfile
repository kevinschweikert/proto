build:
    cargo build -p proto

test:
    cargo test -p proto

update-readme:
    cargo run -p xtask

run *ARGS:
    cargo run -p proto -- {{ARGS}}

