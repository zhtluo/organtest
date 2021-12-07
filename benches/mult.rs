use criterion::{criterion_group, criterion_main, Criterion};
use organ::CompParameters;
use rug::{rand::RandState, Integer};
use rug_fft::{bit_rev_radix_2_intt, bit_rev_radix_2_ntt};
use std::iter::repeat_with;
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

/*
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
*/

fn compute(param: Arc<CompParameters>) -> Vec<Integer> {
    let mut c: Vec<Integer> = (0..param.a.len())
        .map(|i| Integer::from(&param.a[i] * &param.b[i]))
        .collect();
    bit_rev_radix_2_intt(&mut c, &param.p, &param.w);
    for i in 0..c.len() {
        c[i] = Integer::from(&c[i] * &param.order) / &param.p;
    }
    c
}

fn multithread_compute(param: CompParameters) {
    const NTHREADS: u32 = 100;
    let mut children = vec![];
    let (tx, rx) = mpsc::channel();
    let param_arc = Arc::new(param);
    for _ in 0..NTHREADS {
        let txc = tx.clone();
        let paramc = Arc::clone(&param_arc);
        children.push(thread::spawn(move || {
            txc.send(compute(paramc)).unwrap();
        }));
    }
    for _ in 0..NTHREADS {
        rx.recv().unwrap();
    }
}

pub fn criterion_benchmark_bulk(c: &mut Criterion) {
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

    /*
    BULK_PARAMS = {
        'P': PBULK,
        'Q': QBULK,
        'RING_V': (7 * (2 ** 290)) + 1,
        'VECTOR_LENGTH': 8192,
        'BITS': 226
    }
    PBULK = 2 ** 226 - 5
    QBULK = int(group256.order())
    group256: secp256k1
    https://neuromancer.sk/std/secg/secp256k1
        */

    let len = 13; // 2^len = 8192
    let p: Integer = Integer::from(Integer::u_pow_u(2, 226)) - 5;
    let w = Integer::from(Integer::u_pow_u(2, 290)) * 7 + 1;
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
        "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap();

    let param = CompParameters {
        a: input1,
        b: input2,
        p: p,
        w: w,
        order: zq_order,
    };

    c.bench_function("bulk_compute", |b| {
        b.iter(|| multithread_compute(param.clone()))
    });
}

pub fn criterion_benchmark_base(c: &mut Criterion) {
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

    /*
    BASE_PARAMS = {
        'P': 2 ** 32 - 5,
        #'Q': 2 ** 56 - 5,
        'Q': group112.order(),

        'RING_V': (57 * (2 ** 96)) + 1,
        'VECTOR_LENGTH': 2048,
        'BITS': 32
    }
    group112 : secp112r1
    https://neuromancer.sk/std/secg/secp112r1
        */

    let len = 11; // 2^len = 2048
    let p: Integer = Integer::from(Integer::u_pow_u(2, 32)) - 5; // 2 ** 32 - 5
    let w = Integer::from(Integer::u_pow_u(2, 96)) * 57 + 1;
    let mut rand = RandState::new();
    let mut input1: Vec<Integer> = repeat_with(|| p.clone().random_below(&mut rand))
        .take(1 << len)
        .collect();
    let mut input2: Vec<Integer> = repeat_with(|| p.clone().random_below(&mut rand))
        .take(1 << len)
        .collect();
    bit_rev_radix_2_ntt(&mut input1, &p, &w);
    bit_rev_radix_2_ntt(&mut input2, &p, &w);
    let zq_order = Integer::from_str_radix("db7c2abf62e35e7628dfac6561c5", 16).unwrap();

    let param = CompParameters {
        a: input1,
        b: input2,
        p: p,
        w: w,
        order: zq_order,
    };

    c.bench_function("base_compute", |b| {
        b.iter(|| multithread_compute(param.clone()))
    });
}

criterion_group!(benches, criterion_benchmark_base, criterion_benchmark_bulk);
criterion_main!(benches);
