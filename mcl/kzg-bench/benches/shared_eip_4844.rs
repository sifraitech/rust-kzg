use criterion::{criterion_group, criterion_main, Criterion};
use kzg_bench::benches::eip_4844::{
    bench_compute_aggregate_kzg_proof, bench_verify_aggregate_kzg_proof,
};
use mcl_rust::eip_4844::*;
use mcl_rust::mcl_methods::init;
use mcl_rust::CurveType;

fn compute_aggregate_kzg_proof_bench(c: &mut Criterion) {
    assert!(init(CurveType::BLS12_381));
    bench_compute_aggregate_kzg_proof(c, &load_trusted_setup, &compute_aggregate_kzg_proof)
}

fn verify_aggregate_kzg_proof_bench(c: &mut Criterion) {
    assert!(init(CurveType::BLS12_381));
    bench_verify_aggregate_kzg_proof(
        c,
        &load_trusted_setup,
        &blob_to_kzg_commitment,
        &compute_aggregate_kzg_proof,
        &verify_aggregate_kzg_proof,
    )
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = compute_aggregate_kzg_proof_bench, verify_aggregate_kzg_proof_bench
}

criterion_main!(benches);
