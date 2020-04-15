fn decimal_to_vec(d: usize, digits: usize) -> Vec<usize> {
    (0..digits).fold(vec![d; digits], |mut acc, x| {
        if x != digits-1 { acc[digits-x-2] = acc[digits-x-1] / 10 }
        acc[digits-x-1] %= 10;
        acc
    })
}
fn main() {
    println!("{:?}", decimal_to_vec(1234, 4));
}