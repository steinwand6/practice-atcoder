use proconio::input;

fn main() {
    input! {
        n:u64,
        array:[u64;n],
    };
    println!("{}", lcm_n(array));
}

fn lcm_n(array: Vec<u64>) -> u64 {
    let mut ans = array[0];
    let mut tmp;
    for a in array {
        tmp = lcm(a, ans);
        ans = ans.max(tmp);
    }
    ans
}

fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => return a,
        _ => gcd(b, a % b),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    let gcd_val = gcd(a, b);
    (a / gcd_val) * (b / gcd_val) * gcd_val
}

#[cfg(test)]
mod tests {
    use crate::{lcm, lcm_n};

    #[test]
    fn sample_lcm() {
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(18, 27), 54);
        assert_eq!(lcm(95, 20), 380);
        assert_eq!(lcm(12, 30), 60);
        assert_eq!(lcm(12, 15), 60);
        assert_eq!(lcm(30, 15), 30);
    }
    #[test]
    fn sample_lcm_n() {
        assert_eq!(lcm_n(vec![12, 30, 15]), 60);
        assert_eq!(lcm_n(vec![12, 18, 14]), 252);
    }
}
