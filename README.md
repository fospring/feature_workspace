# rust feature gate test 
## view macros detail
### use cargo expand 
cargo-expand - Print the result of macro expansion and #[derive]
expansion.  
if you haven't install expand,run:
```shell script
cargo +nightly install cargo-expand
```
view macros' detail by:
```shell
cargo expand > xxx.log
```