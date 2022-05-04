// https://atcoder.jp/contests/abc249/tasks/abc249_b

use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        str: String,
    }
    let u_flg = str.chars().any(|c| c.is_uppercase());
    let l_flg = str.chars().any(|c| c.is_lowercase());
    let set: HashSet<char> = str.chars().collect();
    let is_dup = !(str.len() == set.len());
    if u_flg && l_flg && !is_dup {
        println!("Yes");
    } else {
        println!("No");
    }
}
