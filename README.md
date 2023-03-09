# fizzbuzz_rs

DRAFT: Fizzbuzz server in Rust

## Test

```
RUST_LOG=debug cargo run
```

In an other terminal run:
```
curl 'localhost:8080/v1/fb?i1=3&i2=5&limit=35&str1=fizz&str2=buzz'
```
