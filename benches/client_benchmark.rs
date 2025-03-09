// This benchmarking library uses the criterion package to execute (and test)
// the rust client library calls to our psql-ledger stack.
// It requires a locally running psql-ledger service as a separate process listening
// on http://localhost:8080.
//
// Note that these tests can be used to benchmark alternative implementations of psql-ledger
// provided that they expose the same JSON RPC API.

// TODO - not working properly
#![allow(dead_code)]
use criterion::{criterion_group, criterion_main, Criterion};

extern crate psql_ledger_rst; 
use psql_ledger_rst::client::{create_account, health, status};
use psql_ledger_rst::model::Account;

// status GET request
fn status_benchmark(c: &mut Criterion) {
    let server_addr = String::from("localhost:8080");

    c.bench_function("status", |b| {
        // measure the http round-trip time
        b.iter(|| status(server_addr.clone()))
    });
}

// health GET request
fn health_benchmark(c: &mut Criterion) {
    let server_addr = String::from("localhost:8080");

    c.bench_function("health", |b| {
        // measure the http round-trip time
        b.iter(|| health(server_addr.clone()))
    });
}

// create_account POST request
fn create_account_benchmark(c: &mut Criterion) {
    let server_addr = String::from("localhost:8080");

    c.bench_function("create_account", |b| {
        // measure the http round-trip time
        // includes write to postgres
        b.iter(|| {
            create_account(
                server_addr.clone(),
                Account {
                    id: Some(0),
                    username: Some(String::from("john_doe")),
                    email: Some(String::from("john_doe@example.com")),
                    balance: Some(0),
                    created_at: None,
                },
            )
        })
    });
}

criterion_group!(benches, status_benchmark); // health_benchmark, create_account_benchmark
criterion_main!(benches);
