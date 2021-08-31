#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Bytes; h]
    }
    let mut dp = vec![vec![0i32; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if (i, j) == (h - 1, w - 1) {
                continue;
            }
            let mut val = std::i32::MIN;
            if i + 1 < h {
                let c = if a[i + 1][j] == b'+' { 1 } else { -1 };
                val = val.max(c - dp[i + 1][j]);
            }
            if j + 1 < w {
                let c = if a[i][j + 1] == b'+' { 1 } else { -1 };
                val = val.max(c - dp[i][j + 1]);
            }
            dp[i][j] = val;
        }
    }
    if dp[0][0] == 0 {
        println!("Draw");
    } else if dp[0][0] > 0 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
