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

- check code quality
- clap: connection limit, ip req limiter configuration (LRU size ...), log level, configuration path
- ip req limiter: be proxy aware (allow to retrieve IP from HTTP header ?)
- security: limit tasks, rate limiter, check parameter values (max size of strings ?)
- perf: algo + caching (ajout LRU pour limiter ?)
- metrics
- docker
- optimize docker build: https://www.lpalmieri.com/posts/fast-rust-docker-builds/
- tests
