[private]
list:
    just --list

# Shortcuts for debug mode
d DAY:
    cargo run -- {{DAY}}

da:
    cargo run

# Shortcuts for release mode
r DAY:
    cargo run --release -- {{DAY}}

ra:
    cargo run --release

