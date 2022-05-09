use proconio;
use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize;n]
    };
    println!("{}", choose_cards3(a));
}

fn choose_cards3(a: Vec<usize>) -> usize {
    let mut hmap = HashMap::<usize, usize>::new();
    for i in a {
        let count = hmap.entry(i).or_insert(0);
        *count += 1;
    }
    let mut result = 0;
    for (k, v) in &hmap {
        let target = hmap.get(&(100000 - *k));
        match target {
            Some(x) => {
                if *k == 50000 {
                    result += x * (x - 1) / 2
                } else if *k < 50000 {
                    result += x * v;
                }
            }

            None => (),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::choose_cards3;

    #[test]
    fn sample1() {
        assert_eq!(
            choose_cards3(vec![40000, 50000, 20000, 80000, 50000, 30000]),
            2
        );
    }
    #[test]
    fn sample2() {
        assert_eq!(
            choose_cards3(vec![50000, 50000, 50000, 50000, 50000, 50000]),
            15
        )
    }
}
