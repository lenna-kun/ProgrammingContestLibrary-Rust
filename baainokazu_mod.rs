
// ax + by = gcd(a, b) となるような (x, y) を求める
// 多くの場合 a と b は互いに素として ax + by = 1 となる (x, y) を求める
fn ext_gcd(a: usize, b: usize, x: &mut i64, y: &mut i64) -> usize {
    if b == 0 {
        *x = 1;
        *y = 1;
        return a;
    }
    let d: usize = ext_gcd(b, a%b, y, x); // 再帰的に解く
    *y -= (a / b) as i64 * *x;
    d
}

// 逆元計算 (ここでは a と m が互いに素であることが必要)
fn modinv(a: usize, m: usize) -> usize {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    ext_gcd(a, m, &mut x, &mut y);
    let m = m as i64;
    return ((x % m + m) % m) as usize; // 気持ち的には x % m だが、x が負かもしれないので
}

struct CPH {
    fac: Vec<usize>, // i!
    fac_inv: Vec<usize>, // (i!)^-1
    m: usize,
}

impl CPH {
    fn new(n_max: usize, m: usize) -> Self {
        let mut fac: Vec<usize> = vec![0; n_max];
        let mut fac_inv: Vec<usize> = vec![0; n_max];
        fac[0] = 1;
        fac_inv[0] = 1;
        for i in 1..n_max {
            fac[i] = fac[i - 1] * i % m;
        }
        fac_inv[n_max-1] = modinv(fac[n_max-1], m);
        for i in (2..n_max).rev() {
            fac_inv[i-1] = fac_inv[i] * i % m;
        }
        CPH {
            fac: fac,
            fac_inv: fac_inv,
            m: m,
        }
    }
    // 二項係数計算
    #[allow(non_snake_case)]
    fn C(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.fac_inv[k] * self.fac_inv[n - k] % self.m) % self.m
        }
    }

    #[allow(non_snake_case)]
    fn P(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.fac_inv[n - k] % self.m) % self.m
        }
    }

    #[allow(non_snake_case)]
    fn H(&self, n: usize, k: usize) -> usize {
        if n == 0 && k == 0 {
            1
        } else if k == 0 {
            0
        } else {
            self.fac[n+k-1] * (self.fac_inv[k] * self.fac_inv[n - 1] % self.m) % self.m
        }
    }
}

struct CPH2 {
    fac: Vec<usize>, // i!
    fac_inv: Vec<usize>, // (i!)^-1
    #[allow(dead_code)] inv: Vec<usize>,
    m: usize,
}
impl CPH2 {
    fn new(n_max: usize, m: usize) -> Self {
        let mut fac: Vec<usize> = vec![0; n_max];
        let mut fac_inv: Vec<usize> = vec![0; n_max];
        let mut inv: Vec<usize> = vec![0; n_max];
        fac[0] = 1;
        fac[1] = 1;
        fac_inv[0] = 1;
        fac_inv[1] = 1;
        inv[1] = 1;
        for i in 2..n_max {
            fac[i] = fac[i - 1] * i % m;
            inv[i] = m - inv[m%i] * (m / i) % m;
            fac_inv[i] = fac_inv[i - 1] * inv[i] % m;
        }
        CPH2 {
            fac: fac,
            fac_inv: fac_inv,
            inv: inv,
            m: m,
        }
    }
    // 二項係数計算
    #[allow(non_snake_case)]
    fn C(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.fac_inv[k] * self.fac_inv[n - k] % self.m) % self.m
        }
    }

    #[allow(non_snake_case)]
    fn P(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fac[n] * (self.fac_inv[n - k] % self.m) % self.m
        }
    }

    #[allow(non_snake_case)]
    fn H(&self, n: usize, k: usize) -> usize {
        if n == 0 && k == 0 {
            1
        } else if k == 0 {
            0
        } else {
            self.fac[n+k-1] * (self.fac_inv[k] * self.fac_inv[n - 1] % self.m) % self.m
        }
    }
}

fn main() {
    let cph = CPH::new(5000, 1000000007);
    let cph2 = CPH2::new(5000, 1000000007);
    println!("{} == {}", cph.C(101, 2), cph2.C(101, 2));
    println!("{} == {}", cph.P(71, 2), cph2.P(71, 2));
    println!("{} == {}", cph.H(100, 2), cph2.H(100, 2));
}