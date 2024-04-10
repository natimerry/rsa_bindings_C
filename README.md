# Usage
Compile the code with:
```shell
cargo build --release
```
The static library will now be available at `target/release/librsa_bindings.a`

### Linkage:
Link with C code with:
```shell
gcc ... librsa_bindings.a -lm foo.c
```
### Dependencies:
- libmath 