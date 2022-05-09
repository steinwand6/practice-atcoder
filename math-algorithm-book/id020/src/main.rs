use proconio;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize;n]
    };

    println!("{}", choose_cards(a));
}

fn choose_cards(a: Vec<usize>) -> usize {
    let mut ans = 0;

    let size = a.len();
    for i in 0..size {
        for j in i + 1..size {
            for k in j + 1..size {
                for l in k + 1..size {
                    for m in l + 1..size {
                        if a[i] + a[j] + a[k] + a[l] + a[m] == 1000 {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::choose_cards;

    #[test]
    fn sample1() {
        assert_eq!(choose_cards(vec![100, 150, 200, 250, 300]), 1);
        assert_eq!(
            choose_cards(vec![
                243, 156, 104, 280, 142, 286, 196, 132, 128, 195, 265, 300, 130
            ]),
            4
        );
    }
}
