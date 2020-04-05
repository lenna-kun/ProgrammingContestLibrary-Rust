use std::io::*;

fn read_int(s: &mut StdinLock) -> usize {
    s.by_ref().bytes().map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .skip_while(|c| c.is_whitespace())
        .fold(0, |a, x| (x as u8 - b'0') as usize + a * 10)
}

fn read_char(s: &mut StdinLock) -> char {
    s.by_ref().bytes().map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .skip_while(|c| c.is_whitespace())
        .last()
        .unwrap()
}
fn read_tuple_int(s: &mut StdinLock) -> (usize, usize) {
    (read_int(s), read_int(s))
}
fn read_line_split_int(s: &mut StdinLock, n: usize) -> Vec<usize> {
    (0..n).map(|_| read_int(s)).collect()
}
fn read_line_split_char(s: &mut StdinLock, n: usize) -> Vec<char> {
    (0..n).map(|_| read_char(s)).collect()
}
fn read_line_int(s: &mut StdinLock) -> Vec<usize> {
    s.by_ref().bytes().map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .skip_while(|c| c.is_whitespace())
        .map(|c| (c as u8 - b'0') as usize)
        .collect()
}
fn read_line_char(s: &mut StdinLock) -> Vec<char> {
    s.by_ref().bytes().map(|c| c.unwrap() as char)
        .take_while(|c| !c.is_whitespace())
        .skip_while(|c| c.is_whitespace())
        .collect()
}
fn read_mat_split_int(s: &mut StdinLock, n: usize, m: usize) -> Vec<Vec<usize>> {
    (0..n).map(|_| read_line_split_int(s, m)).collect()
}
fn read_mat_int(s: &mut StdinLock, n: usize) -> Vec<Vec<usize>> {
    (0..n).map(|_| read_line_int(s)).collect()
}
fn read_mat_char(s: &mut StdinLock, n: usize) -> Vec<Vec<char>> {
    (0..n).map(|_| read_line_char(s)).collect()
}
fn read_mat_tuple_int(s: &mut StdinLock, n: usize) -> Vec<(usize, usize)> {
    (0..n).map(|_| read_tuple_int(s)).collect()
}

// fn main(){
//     let s = stdin();
//     let mut s = s.lock();
//     println!("{}", read_char(&mut s));
// }