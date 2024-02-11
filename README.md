# A simple transaction ledger implemented in Rust and PostgreSQL

Web backend tooling for creating user accounts and recording transactions between users.

## Components

* Rust web server built with [Actix Web](https://github.com/actix)
* PostgreSQL interface powered by [deadpool_postgres](https://crates.io/crates/deadpool-postgres), [tokio_pg_mapper](https://crates.io/crates/tokio-pg-mapper-derive) and [cornucopia](https://github.com/cornucopia-rs/cornucopia/#)