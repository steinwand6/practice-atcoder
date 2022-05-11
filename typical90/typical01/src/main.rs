use proconio::input;

fn main() {
    input! {
        n: usize,
        length: usize,
        key: usize,
        v: [usize;n]
    };
    println!("{}", yokan_party(length, key, v));
}

fn yokan_party(length: usize, key: usize, v: Vec<usize>) -> usize {
    // meguru's binary search
    // just copied the sample answers...
    let mut left = 0;
    let mut right = length;
    let mut mid = left + (right - left) / 2;

    while right - left > 1 {
        if solve(&mid, &length, &key, &v) {
            left = mid;
        } else {
            right = mid;
        }
        mid = left + (right - left) / 2;
    }
    left as usize
}

fn solve(m: &usize, l: &usize, key: &usize, v: &Vec<usize>) -> bool {
    let mut cnt = 0;
    let mut pre = 0;
    for i in 0..v.len() {
        if l - v[i] >= *m && v[i] - pre >= *m {
            cnt += 1;
            pre = v[i];
        }
    }
    return cnt >= *key;
}

#[cfg(test)]
mod tests {
    use crate::yokan_party;

    #[test]
    fn sample1() {
        assert_eq!(yokan_party(34, 1, vec![8, 13, 26]), 13)
    }
    #[test]
    fn sample2() {
        assert_eq!(yokan_party(45, 2, vec![7, 11, 16, 20, 28, 34, 38]), 12)
    }
    #[test]
    fn sample3() {
        assert_eq!(yokan_party(100, 1, vec![28, 54, 81]), 46)
    }
}
