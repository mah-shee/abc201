#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    let mut ans = 0;
    for i in 0..=9999 {
        let mut x = i;
        let mut flag = vec![false; 10];
        for _ in 0..4 {
            flag[x % 10] = true;
            x /= 10;
        }
        let mut flag2 = true;
        for j in 0..10 {
            if s[j] == 'o' && !flag[j] {
                flag2 = false;
            }
            if s[j] == 'x' && flag[j] {
                flag2 = false;
            }
        }
        if flag2 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
