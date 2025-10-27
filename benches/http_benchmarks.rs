use crabrace::CrabraceClient;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_client_creation(c: &mut Criterion) {
    c.bench_function("create_client", |b| {
        b.iter(|| {
            let client = CrabraceClient::new("http://localhost:8080");
            black_box(client)
        })
    });
}

fn bench_client_with_custom_url(c: &mut Criterion) {
    c.bench_function("create_client_with_url", |b| {
        b.iter(|| {
            let client = CrabraceClient::new("http://example.com:9090");
            black_box(client)
        })
    });
}

// Note: These benchmarks require a running server
// They are commented out by default but can be enabled for end-to-end testing
/*
fn bench_http_get_providers(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let client = CrabraceClient::new();

    c.bench_function("http_get_providers", |b| {
        b.to_async(&rt).iter(|| async {
            let result = client.get_providers().await;
            black_box(result)
        })
    });
}

fn bench_http_health_check(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let client = CrabraceClient::new();

    c.bench_function("http_health_check", |b| {
        b.to_async(&rt).iter(|| async {
            let result = client.health().await;
            black_box(result)
        })
    });
}
*/

criterion_group!(
    benches,
    bench_client_creation,
    bench_client_with_custom_url,
    // bench_http_get_providers,  // Uncomment if server is running
    // bench_http_health_check,   // Uncomment if server is running
);
criterion_main!(benches);
