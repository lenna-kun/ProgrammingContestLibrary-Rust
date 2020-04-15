fn gcd(mut p: usize, mut q: usize) -> usize {
    while q != 0 {
        let r = p % q;
        p = q;
        q = r;
    }
    p
}