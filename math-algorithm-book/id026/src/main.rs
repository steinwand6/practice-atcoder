use proconio::{self, input};
fn main() {
    input! {
        n: usize
    };
    println!("{}", coin_gacha(n));
}

fn coin_gacha(n: usize) -> f64 {
    // 例えば3枚のコインがあるとき
    // 1枚目の期待値：1ドル
    // 2枚目の期待値: 1枚目を1/3で引くとまた突っこむ必要がある、1 + 1/3 + 1/9 +...
    // 3枚目の期待値：1枚目と2枚目を引くとなので、1 + 2/3 + 4/9 + 8/27 + ...
    // 和の公式により、2枚目のときは (1 / (1 - 1/3)) → 3/2回
    // 3枚目のときは(1 / (1 - 2/3)) → 3回
    // N枚のコインがあるとき、i枚目を引くまでの期待値は...
    // (1 / (1 - (i-1)/N))
    let mut result: f64 = 0.0;
    for i in 1..=n {
        result += 1.0 / (1.0 - (i as f64 - 1.0) / n as f64);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::coin_gacha;

    #[test]
    fn sample1() {
        assert!((coin_gacha(5) - 11.416666666667).abs() < 0.000001)
    }
}
