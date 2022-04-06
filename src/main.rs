#![deny(clippy::pedantic)]

use bitvec::macros::internal::funty::Fundamental;
mod bbs;
mod primes;
mod tests;

fn main() {
    let bitseq = bbs::bbs(20000, 2048);
    let seq = bitseq.iter().fold(String::with_capacity(bitseq.len()), |mut acc, val| {
        acc.push_str(&val.as_i32().to_string());
        acc
    });
    println!("{seq}");
    let bits = seq;

    // let bits = std::fs::read_to_string("sequence.txt").expect("Cannot open file with sequence");
    
    let single_bits = tests::single_bit_test(&bits);
    println!("Single bit: {single_bits:?}");
    let series = tests::series_test(&bits);
    println!("Series: {series:?}");
    let long_series = tests::long_series_test(&bits);
    println!("Long series: {long_series:?}");
    let poker = tests::poker_test(&bits);
    println!("Poker: {poker:?}");
}
