// This benchmarking library uses the criterion package to execute (and test)
// the rust client library calls to our psql-ledger stack.
// It requires a locally running psql-ledger service as a separate process listening
// on http://localhost:8080.
//
// Note that these tests can be used to benchmark alternative implementations of psql-ledger
// provided that they expose the same JSON RPC API.

// TODO - not working properly
#![allow(dead_code)]
use criterion::{criterion_group, black_box, criterion_main, Criterion};
use tokio::runtime::Runtime; 
use psql_ledger_rst::client::{create_account, health, status};
use psql_ledger_rst::models::AccountParams;
use rand::{thread_rng, Rng};

// status GET request
fn status_benchmark(c: &mut Criterion) {
    let server_addr = String::from("localhost:8080");

    let rt = Runtime::new();

    c.bench_function("status", |b| {
        // measure the http round-trip time
        b.iter(|| {
            rt.block_on(async {
                // Call the async function and handle errors manually
                match status(server_addr.clone())
                .await {
                    Ok(health) => black_box(health),
                    Err(_) => panic!("healthcheck resulted in an error"),
                };
            });  
        } )
    });
}

// health GET request
fn health_benchmark(c: &mut Criterion) {
    let server_addr = String::from("localhost:8080");

    let rt = Runtime::new().unwrap();

    c.bench_function("health", |b| {
        // measure the http round-trip time
        b.iter(|| {
            rt.block_on(async {
                // Call the async function and handle errors manually
                match health(server_addr.clone())
                .await {
                    Ok(health) => black_box(health),
                    Err(_) => panic!("healthcheck resulted in an error"),
                };
            });  
    })
    });
}

// create_account POST request
fn create_account_benchmark(c: &mut Criterion) {
    let server_addr = String::from("localhost:8080");

    // create random number generator
    let mut rng = thread_rng();

    let rt = Runtime::new().unwrap();

    c.bench_function("create_account", |b| {
        b.iter(|| {
            let random_int: i32 = rng.gen_range(0..1000000);
            let username = format!("john_doe{}", random_int);
            let email = format!("john_doe{}@example.com", random_int);

            // Execute the async block
            rt.block_on(async {
                // Call the async function and handle errors manually
                match create_account(
                    server_addr.clone(),
                    AccountParams {
                        id: None,
                        username: Some(username),
                        email: Some(email),
                        balance: None,
                    },
                ).await {
                    Ok(account) => black_box(account),
                    Err(_) => panic!("create_account resulted in an error"),
                };
            });
        })
    });
}

criterion_group!(benches, create_account_benchmark); // health_benchmark, create_account_benchmark
criterion_main!(benches);
