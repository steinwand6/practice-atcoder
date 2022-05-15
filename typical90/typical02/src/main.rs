use proconio::input;

fn main() {
    input! {
        n: usize
    };
    encyclopedia_of_parentheses(n);
}

fn encyclopedia_of_parentheses(n: usize) {
    if n % 2 == 1 {
        return;
    }
    for i in 0..(1 << n) {
        let mut j = n as i32 - 1;
        let mut s = String::from("");
        while j >= 0 {
            if i & 1 << j == 0 {
                s = s + "(";
            } else {
                s = s + ")";
            }
            j -= 1;
        }
        if judge_parentheses(&s) {
            println!("{}", s);
        }
    }
}

fn judge_parentheses(s: &str) -> bool {
    let mut result = 0;
    for p in s.chars() {
        match p {
            '(' => result += 1,
            ')' => result -= 1,
            _ => return false,
        }
        if result < 0 {
            return false;
        }
    }
    result == 0
}
