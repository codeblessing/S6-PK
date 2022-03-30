#[derive(Clone, Copy, Debug)]
pub enum Result {
    Passed,
    Failed,
}

pub fn single_bit_test(bits: &str) -> Result {
    assert!(bits.len() == 20_000);
    let count = bits.chars().filter(|c| c == &'1').count();

    if count > 9725 && count < 10275 {
        return Result::Passed;
    }

    Result::Failed
}

pub fn series_test(bits: &str) -> Result {
    let ones: Vec<&str> = bits.split('0').filter(|s| !s.is_empty()).collect();
    let zeros: Vec<&str> = bits.split('1').filter(|s| !s.is_empty()).collect();

    let seq_1_zeros = zeros.iter().filter(|s| s.len() == 1).count();
    let seq_1_ones = ones.iter().filter(|s| s.len() == 1).count();
    let seq_2_zeros = zeros.iter().filter(|s| s.len() == 2).count();
    let seq_2_ones = ones.iter().filter(|s| s.len() == 2).count();
    let seq_3_zeros = zeros.iter().filter(|s| s.len() == 3).count();
    let seq_3_ones = ones.iter().filter(|s| s.len() == 3).count();
    let seq_4_zeros = zeros.iter().filter(|s| s.len() == 4).count();
    let seq_4_ones = ones.iter().filter(|s| s.len() == 4).count();
    let seq_5_zeros = zeros.iter().filter(|s| s.len() == 5).count();
    let seq_5_ones = ones.iter().filter(|s| s.len() == 5).count();
    let seq_6_zeros = zeros.iter().filter(|s| s.len() > 5).count();
    let seq_6_ones = ones.iter().filter(|s| s.len() > 5).count();

    println!("length = 1:  {seq_1_zeros:<5} | {seq_1_ones:<5}");
    println!("length = 2:  {seq_2_zeros:<5} | {seq_2_ones:<5}");
    println!("length = 3:  {seq_3_zeros:<5} | {seq_3_ones:<5}");
    println!("length = 4:  {seq_4_zeros:<5} | {seq_4_ones:<5}");
    println!("length = 5:  {seq_5_zeros:<5} | {seq_5_ones:<5}");
    println!("length = 6+: {seq_6_zeros:<5} | {seq_6_ones:<5}");
    if seq_1_zeros < 2315
        || seq_1_zeros > 2685
        || seq_2_zeros < 1114
        || seq_2_zeros > 1386
        || seq_3_zeros < 527
        || seq_3_zeros > 723
        || seq_4_zeros < 240
        || seq_4_zeros > 384
        || seq_5_zeros < 103
        || seq_5_zeros > 209
        || seq_6_zeros < 103
        || seq_6_zeros > 209
    {
        return Result::Failed;
    }

    if seq_1_ones < 2315
        || seq_1_ones > 2685
        || seq_2_ones < 1114
        || seq_2_ones > 1386
        || seq_3_ones < 527
        || seq_3_ones > 723
        || seq_4_ones < 240
        || seq_4_ones > 384
        || seq_5_ones < 103
        || seq_5_ones > 209
        || seq_6_ones < 103
        || seq_6_ones > 209
    {
        return Result::Failed;
    }

    Result::Passed
}

pub fn long_series_test(bits: &str) -> Result {
    let ones: Vec<&str> = bits.split('0').filter(|s| !s.is_empty()).collect();
    let zeros: Vec<&str> = bits.split('1').filter(|s| !s.is_empty()).collect();

    if zeros.iter().filter(|s| s.len() > 25).count() + ones.iter().filter(|s| s.len() > 25).count()
        > 0
    {
        return Result::Failed;
    }

    Result::Passed
}
