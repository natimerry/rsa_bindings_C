# Usage
Compile the code with:
```shell
cargo build --release
```
The static library will now be available at `target/release/librsa_bindings.a`

### Functions:

- generate_priv_keys(bits)
  - Generates a new private key with the specified number of bits. (2048 default)
- generate_public_key(private_key)
  - Generates a new public key from the given private key using a random seed.
- encrypt_string(public_key,string)
  - Returns a padded byte array of length 256 bytes with the encrypted data
- decrypt_string(private_key,data)
  - Returns a C string with the unencrypted data.

Header files will be generated at `c_bindings/librsa.h`
### Linkage:
Link with C code with:
```shell
gcc ... librsa_bindings.a -lm foo.c
```
### Dependencies:
- libmath 