use proconio;

fn main() {
    proconio::input! {
        n: u32,
        a: u32,
        b: u32,
    };
    solver(n, a, b);
}

fn solver(n: u32, a: u32, b: u32) {
    let mut flg = false;
    let mut x = 0;
    for y in 0..(n * a) {
        x = 0;
        if y % 2 == 1 {
            while x < n * b {
                if flg {
                    prace_tile('#', b);
                    x += b;
                    if x == n * b {
                        break;
                    }
                    prace_tile('.', b);
                    x += b;
                } else {
                    prace_tile('.', b);
                    x += b;
                    if x == n * b {
                        break;
                    }
                    prace_tile('#', b);
                    x += b;
                }
            }
            println!("")
        } else {
            x = 0;
            while x < n * b {
                if !flg {
                    prace_tile('.', b);
                    x += b;
                    if x == n * b {
                        break;
                    }
                    prace_tile('#', b);
                    x += b;
                } else {
                    prace_tile('#', b);
                    x += b;
                    if x == n * b {
                        break;
                    }
                    prace_tile('.', b);
                    x += b;
                }
            }
            println!("");
        }
        if (y + 1) % a == 0 {
            flg = !flg
        }
    }
}

fn prace_tile(c: char, n: u32) {
    for _ in 0..n {
        print!("{}", c)
    }
}
