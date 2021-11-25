use rug::Integer;

#[derive(Debug, Clone)]
pub struct CompParameters {
    pub a: Vec<Integer>,
    pub b: Vec<Integer>,
    pub p: Integer,
    pub w: Integer,
    pub order: Integer
}

#[cfg(test)]
mod tests {
    use rug::Integer;
    use rug_fft::bit_rev_radix_2_ntt;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);

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
    }
}
