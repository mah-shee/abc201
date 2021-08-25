#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut s_t: [(String, usize); n],
    }
    s_t.sort_by_key(|k| k.1);
    println!("{}", s_t[n - 2].0);
}
