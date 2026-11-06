fix:
    cargo clippy --fix --allow-dirty --allow-staged --workspace --all-targets --all-features
    cargo clippy --fix --allow-dirty --allow-staged --target wasm32-unknown-unknown --no-default-features --features dev
    cargo fmt --all
