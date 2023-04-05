use std::collections::{BTreeSet, HashSet};
use std::process;

use rustc_hash::FxHashSet;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use uuid::Uuid;

const SIZE: usize = 100;

fn find_in_vec(uuids: &Vec<Uuid>, uuid: &Uuid) -> bool {
    uuids.contains(uuid)
}

fn find_in_hashset(uuids: &HashSet<Uuid>, uuid: &Uuid) -> bool {
    uuids.contains(uuid)
}

fn find_in_fxhashset(uuids: &FxHashSet<Uuid>, uuid: &Uuid) -> bool {
    uuids.contains(uuid)
}

fn find_in_array(uuids: &[Uuid; SIZE], uuid: &Uuid) -> bool {
    uuids.contains(uuid)
}

fn find_in_sorted_array(uuids: &[Uuid; SIZE], uuid: &Uuid) -> bool {
    uuids.binary_search(uuid).is_ok()
}

fn find_in_btreeset(uuids: &BTreeSet<Uuid>, uuid: &Uuid) -> bool {
    uuids.contains(uuid)
}

fn criterion_benchmark(c: &mut Criterion) {
    let uuids: Vec<Uuid> = (0..SIZE).map(|_| Uuid::new_v4()).collect();
    let unknown_uuid = Uuid::new_v4();
    if uuids.contains(&unknown_uuid) {
        process::exit(1);
    }
    println!("{uuids:?}");
    println!("{unknown_uuid}");

    c.bench_function("find unknown in vec", |b| {
        b.iter(|| find_in_vec(black_box(&uuids), black_box(&unknown_uuid)))
    });

    let uuids_hash: HashSet<Uuid> = HashSet::from_iter(uuids.clone());

    c.bench_function("find unknown in hashset", |b| {
        b.iter(|| find_in_hashset(black_box(&uuids_hash), black_box(&unknown_uuid)))
    });

    let uuids_fxhash: FxHashSet<Uuid> = FxHashSet::from_iter(uuids.clone());

    c.bench_function("find unknown in fxhashset", |b| {
        b.iter(|| find_in_fxhashset(black_box(&uuids_fxhash), black_box(&unknown_uuid)))
    });

    let uuids_array: [Uuid; SIZE] = uuids.clone().try_into().unwrap();

    c.bench_function("find unknown in array", |b| {
        b.iter(|| find_in_array(black_box(&uuids_array), black_box(&unknown_uuid)))
    });

    let mut sorted_uuids: Vec<Uuid> = uuids.clone();
    sorted_uuids.sort();
    let sorted_uuids: [Uuid; SIZE] = sorted_uuids.try_into().unwrap();

    c.bench_function("find unknown in sorted array", |b| {
        b.iter(|| find_in_sorted_array(black_box(&sorted_uuids), black_box(&unknown_uuid)))
    });

    let uuids_btreeset: BTreeSet<Uuid> = BTreeSet::from_iter(uuids.clone());

    c.bench_function("find unknown in btreeset", |b| {
        b.iter(|| find_in_btreeset(black_box(&uuids_btreeset), black_box(&unknown_uuid)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
