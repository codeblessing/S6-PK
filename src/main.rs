#![deny(clippy::pedantic)]

use bitvec::macros::internal::funty::Fundamental;
mod primes;
mod bbs;
mod bit_test;

fn main() {
    // let bitseq = bbs::bbs(20000, 1024);
    // let seq = bitseq.iter().fold(String::with_capacity(bitseq.len()), |mut acc, val| {
    //     acc.push_str(&val.as_i32().to_string());
    //     acc
    // });
    // println!("{seq}");

    let bits = std::fs::read_to_string("sequence.txt").expect("Cannot open file with sequence");
    // let bits = seq;
    let single_bits = bit_test::single_bit_test(&bits);
    println!("Single bit: {single_bits:?}");
    let series = bit_test::series_test(&bits);
    println!("Series: {series:?}");
    let long_series = bit_test::long_series_test(&bits);
    println!("Long series: {long_series:?}");
}
