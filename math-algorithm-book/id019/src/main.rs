use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize;n]
    };
    println!("{}", choose_cards(a));
}

fn choose_cards(a: Vec<usize>) -> usize {
    let mut hm = HashMap::<usize, usize>::new();
    for i in a {
        let count = hm.entry(i).or_insert(0);
        *count += 1;
    }
    let r_c = hm.get(&1).unwrap_or(&0);
    let y_c = hm.get(&2).unwrap_or(&0);
    let b_c = hm.get(&3).unwrap_or(&0);
    calc_comb(*r_c, 2) + calc_comb(*y_c, 2) + calc_comb(*b_c, 2)
}

fn calc_comb(mut n: usize, r: usize) -> usize {
    if n < r {
        return 0;
    }
    let mut result = 1;
    for _ in 0..r {
        result *= n;
        n -= 1
    }
    result / factorial(r)
}

fn factorial(n: usize) -> usize {
    if n == 1 {
        return 1;
    }
    return n * factorial(n - 1);
}

#[cfg(test)]
mod tests {
    use crate::{calc_comb, choose_cards};

    #[test]
    fn calc_comb_test() {
        assert_eq!(calc_comb(6, 3), 20);
        assert_eq!(calc_comb(5, 2), 10);
    }
    #[test]
    fn sample1() {
        assert_eq!(choose_cards(vec![1, 3, 2, 1, 1, 2]), 4);
    }
}
