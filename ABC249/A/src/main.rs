// https://atcoder.jp/contests/abc249/tasks/abc249_a

use proconio::input;

fn main() {
    input! {
        t:[usize; 3],
        a:[usize; 3],
        x: usize
    }
    // X秒後、歩きと休みが何サイクル繰り返されたか求め、
    // そのサイクル分だけ進んだ距離を出す
    let mut takahashi = (x / (t[0] + t[2])) * (t[0] * t[1]);
    let mut aoki = (x / (a[0] + a[2])) * (a[0] * a[1]);

    // 上で求めたサイクルの余りでどれだけ進んでいたかを足す
    takahashi += if (x % (t[0] + t[2])) >= t[0] {
        t[0] * t[1]
    } else {
        (x % (t[0] + t[2])) * t[1]
    };
    aoki += if (x % (a[0] + a[2])) >= a[0] {
        a[0] * a[1]
    } else {
        (x % (a[0] + a[2])) * a[1]
    };

    if takahashi > aoki {
        println!("Takahashi");
    } else if takahashi < aoki {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
