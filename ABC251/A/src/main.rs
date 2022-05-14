use proconio::input;

fn main() {
    input! {
        str: String,
    };
    println!("{}", six_characters(&str))
}

fn six_characters(s: &str) -> String {
    let mut length = 0;
    let mut ans = "".to_string();
    while length < 6 {
        ans = ans + s;
        length = ans.len();
    }
    ans
}

#[test]
fn sample1() {
    assert_eq!(six_characters("abc"), "abcabc".to_string())
}
#[test]
fn sample2() {
    assert_eq!(six_characters("zz"), "zzzzzz".to_string())
}
#[test]
fn my_test1() {
    assert_eq!(six_characters("a"), "aaaaaa".to_string())
}
