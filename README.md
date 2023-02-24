# fizzbuzz_rs
Fizzbuzz server in Rust

## Test

```
RUST_LOG=debug cargo run
```

In an other terminal run:
```
curl 'localhost:8080/v1/fb?i1=3&i2=5&limit=35&str1=fizz&str2=buzz'
```

## TODO

- split v1/mod.rs file
- security: limit tasks, rate limiter, check parameter values (max size of strings ?)
- perf: algo + caching (ajout LRU pour limiter ?)
- metrics
- docker
- tests
