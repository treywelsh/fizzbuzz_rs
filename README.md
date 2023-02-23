# fizzbuzz_rs
Fizzbuzz server in Rust

## Test

```
RUST_LOG=debug cargo run
```

## TODO

- Faire retour tableau json
- split v1/mod.rs file
- security: limit tasks, rate limiter, check parameter values (max size of strings ?)
- perf: algo + caching (ajout LRU pour limiter ?)
- metrics
- docker
- tests
