use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [(String, usize);n]
    };
    println!("{}", poem_online_judge(a))
}

fn poem_online_judge(a: Vec<(String, usize)>) -> usize {
    let mut hm = HashMap::<String, usize>::new();
    let mut max = 0;
    for el in &a {
        if hm.contains_key(&el.0) {
            continue;
        }
        let res = hm.insert((&el.0).to_string(), el.1);
        if res == None {
            max = max.max(el.1);
        }
    }
    let mut i = 0;
    for el in &a {
        i += 1;
        let val = hm.get(&el.0).unwrap();
        if *val == max {
            return i;
        }
    }
    return 0;
}

#[test]
fn sample1() {
    assert_eq!(
        poem_online_judge(vec![
            ("aaa".to_string(), 10),
            ("bbb".to_string(), 20),
            ("aaa".to_string(), 30)
        ]),
        2
    )
}
#[test]
fn sample2() {
    assert_eq!(
        poem_online_judge(vec![
            ("aaa".to_string(), 9),
            ("bbb".to_string(), 10),
            ("ccc".to_string(), 10),
            ("ddd".to_string(), 10),
            ("bbb".to_string(), 11)
        ]),
        2
    )
}
#[test]
fn sample3() {
    assert_eq!(
        poem_online_judge(vec![
            ("bb".to_string(), 3),
            ("ba".to_string(), 1),
            ("aa".to_string(), 4),
            ("bb".to_string(), 1),
            ("ba".to_string(), 5),
            ("aa".to_string(), 9),
            ("aa".to_string(), 2),
            ("ab".to_string(), 6),
            ("bb".to_string(), 5),
            ("ab".to_string(), 3),
        ]),
        8
    )
}
