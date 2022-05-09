use proconio;
use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize;n],
    };
    println!("{}", convenience_store1(a));
}

fn convenience_store1(goods: Vec<usize>) -> usize {
    let mut price_map = HashMap::<usize, usize>::new();
    for g in goods {
        let count = price_map.entry(g).or_insert(0);
        *count += 1
    }
    return *price_map.get(&100).unwrap_or(&0) * *price_map.get(&400).unwrap_or(&0)
        + *price_map.get(&200).unwrap_or(&0) * *price_map.get(&300).unwrap_or(&0);
}

#[cfg(test)]
mod tests {
    use crate::convenience_store1;

    #[test]
    fn sample1() {
        assert_eq!(convenience_store1(vec![100, 300, 400, 400, 200]), 3);
    }
}
