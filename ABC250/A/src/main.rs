use proconio;

fn main() {
    proconio::input! {
        h: u32,
       w: u32,
        r: u32,
        c: u32,
    };
    println!("{}", solver(h, w, r, c))
}

fn solver(h: u32, w: u32, r: u32, c: u32) -> u32 {
    // h, w : 全体の高さ、幅
    // r, c : ；主役の座標
    let mut ans = 0;
    if r != h {
        ans += 1
    }
    if r != 1 {
        ans += 1
    }
    if c != w {
        ans += 1
    }
    if c != 1 {
        ans += 1
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::solver;

    #[test]
    fn sample1() {
        assert_eq!(solver(3, 4, 2, 2), 4)
    }
    #[test]
    fn sample2() {
        assert_eq!(solver(3, 4, 1, 3), 3)
    }
}
