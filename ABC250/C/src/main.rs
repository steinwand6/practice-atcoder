use proconio;

fn main() {
    proconio::input! {
        n:usize,
        q:usize,
        op:[usize;q],
    }
    let mut ary = vec![0; n];
    for i in 0..n {
        ary[i as usize] = i + 1;
    }
    ary = solver(ary, op);
    for i in 0..n {
        if i != 0 {
            print!(" ")
        }
        print!("{}", ary[i as usize]);
    }
    println!("")
}

fn solver(mut ary: Vec<usize>, op: Vec<usize>) -> Vec<usize> {
    let mut pos: Vec<usize> = vec![0; ary.len()];
    for i in 0..ary.len() {
        pos[i] = i
    }
    for o in op {
        let p1 = pos[o - 1];
        let p2 = if p1 == ary.len() - 1 { p1 - 1 } else { p1 + 1 };
        let v1 = ary[p1];
        let v2 = ary[p2];
        ary.swap(p1, p2);
        pos.swap(v1 - 1, v2 - 1);
    }
    ary
}

#[cfg(test)]
mod tests {
    use crate::solver;

    #[test]
    fn sample1() {
        assert_eq!(
            solver(vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]),
            vec![1, 2, 3, 5, 4]
        )
    }
    #[test]
    fn sample2() {
        assert_eq!(
            solver(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 2, 3, 4, 5, 6, 7]),
            vec![1, 2, 3, 4, 5, 7, 6]
        )
    }
    #[test]
    fn sample3() {
        assert_eq!(
            solver(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![1, 5, 2, 9, 6, 6]),
            vec![1, 2, 3, 4, 5, 7, 6, 8, 10, 9]
        )
    }
}
