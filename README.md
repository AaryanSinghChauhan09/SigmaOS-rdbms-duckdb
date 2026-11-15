# SigmaOS-rdbms-duckdb

> **DuckDB — Analytical DB** — Relational database / data platform integration for SigmaOS.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
In-process analytical SQL database engine — DuckDB Foundation

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate implements the `duckdb` subsystem for SigmaOS:
- Fully real Rust implementation (no stubs)
- Fail-fast validation with typed errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Zero-Copy where applicable

## API

```rust
use sigmaos_rdbms_duckdb::*;
// See src/lib.rs for full public API
```

## Documentation

Full documentation is in the [SigmaOS GitHub Wiki → Data-Science-Tools](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Data-Science-Tools).

## License

MIT
