use crate::traits::HashGenerator;

fn generate_hash_set(
    generator: &mut dyn HashGenerator,
    dataset: &[impl AsRef<str>],
) -> (Vec<String>, Vec<String>) {
    let original = dataset
        .iter()
        .map(|msg| generator.generate(msg.as_ref().as_bytes()))
        .collect();

    let modified = dataset
        .iter()
        .map(|msg| msg.as_ref().as_bytes())
        .map(|msg| {
            let mut msg = msg.to_owned();
            msg[0] ^= 0b10000000;
            msg
        })
        .map(|ref msg| generator.generate(msg))
        .collect();

    (original, modified)
}

pub fn check_sac(generator: &mut dyn HashGenerator, dataset: &[impl AsRef<str>]) -> f64 {
    let (original, modified) = generate_hash_set(generator, dataset);

    let change_ratios: Vec<_> = original
        .iter()
        .zip(modified.iter())
        .map(|(x, y)| {
            let x = x.as_bytes();
            let y = y.as_bytes();
            let changes: usize = x
                .iter()
                .zip(y)
                .map(|(bx, by)| if bx != by { 1 } else { 0 })
                .sum();
            changes as f64 / x.len() as f64
        })
        .collect();

    let passed_count = change_ratios
        .iter()
        .filter(|&&ratio| ratio > 0.48 && ratio < 0.52)
        .count() as f64;

    let passed = passed_count / change_ratios.len() as f64;

    passed
}
