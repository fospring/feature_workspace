https://doc.rust-lang.org/cargo/commands/cargo-test.html

cargo test --color=always tests::test_show_string  --manifest-path /Users/bytedance/workspace/rust/fospring/feature_workspace/fospring/Cargo.toml
cargo test --color=always tests::test_show_string --no-fail-fast --manifest-path /Users/bytedance/workspace/rust/fospring/feature_workspace/fospring/Cargo.toml -- --exact -Z unstable-options --format=json --show-output

cargo test --color=always tests::test_show_string  --manifest-path /Users/bytedance/workspace/rust/fospring/feature_workspace/fospring/Cargo.toml
cargo test --color=always tests::test_show_string  --manifest-path /Users/bytedance/workspace/rust/fospring/feature_workspace/fospring/Cargo.toml -- --exact --show-output
cargo test --color=always tests::test_show_string --no-fail-fast --manifest-path /Users/bytedance/workspace/rust/fospring/feature_workspace/fospring/Cargo.toml -- --exact -Z unstable-options --format=json --show-output

build cmd:
## fospring vc
```shell script
$ cargo build --features fospring_vc
```
## neo
```shell script
cargo build --features neo
```

