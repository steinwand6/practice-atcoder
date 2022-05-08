use proconio;

fn main() {
    proconio::input! {
        n:u32,
        q:u32,
        op:[u32;q],
    }
    let mut ary = vec![0; n as usize];
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

fn solver(mut ary: Vec<u32>, op: Vec<u32>) -> Vec<u32> {
    let mut tmp = 0;
    let l = ary.len();
    let op_l = op.len();
    for i in 0..op_l {
        let j = ary.iter().position(|x| *x == op[i]);
        match j {
            Some(j) => {
                if j == l - 1 {
                    tmp = ary[j];
                    ary[j] = ary[j - 1];
                    ary[j - 1] = tmp;
                } else {
                    tmp = ary[j];
                    ary[j] = ary[j + 1];
                    ary[j + 1] = tmp;
                }
            }
            _ => (),
        }
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
