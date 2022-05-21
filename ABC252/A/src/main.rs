use proconio::input;
use std::char::from_u32;
fn main() {
    input! {
        n: u32
    }
    println!("{}", ascii_code(n))
}

fn ascii_code(n: u32) -> char {
    from_u32(n).unwrap()
}

#[test]
fn sample1() {
    assert_eq!(ascii_code(97), 'a');
}

#[test]
fn sample2() {
    assert_eq!(ascii_code(122), 'z');
}
