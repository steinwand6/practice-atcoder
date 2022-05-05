use std::collections::HashMap;

use proconio::input;
fn main() {
    const M: usize = 2 * 10 ^ 5;
    input! {
        n: usize,
        ary: [usize;n],
    };
    let mut map: HashMap<usize, usize> = HashMap::new();
    for &a in ary.iter() {
        if map.contains_key(&a) {
            map.insert(a, map.get(&a).unwrap() + 1);
        } else {
            map.insert(a, 1);
        }
    }
    let mut ans = 0;
    for a in 1..M {
        for b in 1..(M / a) {
            let c = a * b;
            match map.get(&a) {
                Some(x) => match map.get(&b) {
                    Some(y) => match map.get(&c) {
                        Some(z) => ans += x * y * z,
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        }
    }
    println!("{}", ans);
}
