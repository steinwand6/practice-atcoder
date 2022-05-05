//https://atcoder.jp/contests/abc249/tasks/abc249_c

use proconio::{input, marker::Bytes};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Bytes;n]
    };
    let mut answer = 0;
    for bit in 0..1 << n {
        let mut cnt = vec![0; 26];
        let mut tmp = 0;
        for i in 0..n {
            if bit >> i & 1 == 1 {
                for &c in s[i].iter() {
                    cnt[(c - b'a') as usize] += 1;
                }
            }
        }
        for i in 0..26 {
            if cnt[i] == k {
                tmp += 1;
            }
        }
        answer = answer.max(tmp);
    }
    println!("{}", answer);
}
