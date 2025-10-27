use crabrace::providers::registry::ProviderRegistry;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_load_providers(c: &mut Criterion) {
    c.bench_function("load_providers", |b| {
        b.iter(|| {
            let registry = ProviderRegistry::new().unwrap();
            let providers = registry.get_all().unwrap();
            black_box(providers)
        })
    });
}

fn bench_provider_search(c: &mut Criterion) {
    let registry = ProviderRegistry::new().unwrap();
    let providers = registry.get_all().unwrap();

    c.bench_function("find_provider_by_id", |b| {
        b.iter(|| {
            let result = providers.iter().find(|p| p.id == "openai");
            black_box(result)
        })
    });
}

fn bench_model_search(c: &mut Criterion) {
    let registry = ProviderRegistry::new().unwrap();
    let providers = registry.get_all().unwrap();

    c.bench_function("find_model_across_providers", |b| {
        b.iter(|| {
            for provider in &providers {
                let result = provider.models.iter().find(|m| m.id.contains("gpt-4"));
                black_box(result);
            }
        })
    });
}

fn bench_serialize_providers(c: &mut Criterion) {
    let registry = ProviderRegistry::new().unwrap();
    let providers = registry.get_all().unwrap();

    c.bench_function("serialize_all_providers", |b| {
        b.iter(|| {
            let json = serde_json::to_string(&providers).unwrap();
            black_box(json)
        })
    });
}

fn bench_serialize_single_provider(c: &mut Criterion) {
    let registry = ProviderRegistry::new().unwrap();
    let providers = registry.get_all().unwrap();
    let provider = providers.first().unwrap();

    c.bench_function("serialize_single_provider", |b| {
        b.iter(|| {
            let json = serde_json::to_string(provider).unwrap();
            black_box(json)
        })
    });
}

fn bench_provider_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("provider_operations");

    let registry = ProviderRegistry::new().unwrap();
    let providers = registry.get_all().unwrap();

    group.bench_function("count_providers", |b| b.iter(|| black_box(providers.len())));

    group.bench_function("count_all_models", |b| {
        b.iter(|| {
            let count: usize = providers.iter().map(|p| p.models.len()).sum();
            black_box(count)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_load_providers,
    bench_provider_search,
    bench_model_search,
    bench_serialize_providers,
    bench_serialize_single_provider,
    bench_provider_count
);
criterion_main!(benches);
