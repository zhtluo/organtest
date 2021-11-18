use criterion::{criterion_group, criterion_main, Criterion};
use rug::{rand::RandState, Integer};
use rug_fft::{bit_rev_radix_2_intt, bit_rev_radix_2_ntt};
use std::iter::repeat_with;

fn compute(
    a: &Vec<Integer>,
    b: &Vec<Integer>,
    p: &Integer,
    w: &Integer,
    order: &Integer,
) -> Vec<Integer> {
    let mut c: Vec<Integer> = (0..a.len()).map(|i| Integer::from(&a[i] * &b[i])).collect();
    bit_rev_radix_2_intt(&mut c, p, w);
    for i in 0..c.len() {
        c[i] = Integer::from(&c[i] * order) / p;
    }
    c
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut xs = vec![1, 4]
        .into_iter()
        .map(Integer::from)
        .collect::<Vec<_>>();
    let p = Integer::from(7);
    let w = Integer::from(6);
    bit_rev_radix_2_ntt(&mut xs, &p, &w);
    let ys_ex = vec![5, 4]
        .into_iter()
        .map(Integer::from)
        .collect::<Vec<_>>();
    assert_eq!(xs, ys_ex);

    let len = 13; // 2^len = 8196
    let p = Integer::from_str_radix("615318156686744305707043916358806301172428827813979806125310234541438764580027412187489137576809400194366963713", 10).unwrap();
    // (262 * (2 ** 360)) + 1
    let w = {
        let mut tmp = Integer::from_str_radix("262", 10).unwrap();
        let mut lnd = 360 - 13;
        while lnd >= 0 {
            tmp.square_mut();
            tmp %= &p;
            lnd -= 1;
        }
        tmp
    };
    let mut rand = RandState::new();
    let mut input1: Vec<Integer> = repeat_with(|| p.clone().random_below(&mut rand))
        .take(1 << len)
        .collect();
    let mut input2: Vec<Integer> = repeat_with(|| p.clone().random_below(&mut rand))
        .take(1 << len)
        .collect();
    bit_rev_radix_2_ntt(&mut input1, &p, &w);
    bit_rev_radix_2_ntt(&mut input2, &p, &w);
    let zq_order = Integer::from_str_radix(
        "01ffffffffffffffffffffffffffffffffffe9ae2ed07577265dff7f94451e061e163c61",
        16,
    )
    .unwrap();

    c.bench_function("compute", |b| b.iter(|| compute(&input1, &input2, &p, &w, &zq_order)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
