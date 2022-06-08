use std::ops::Add;

use crate::modulo::inverse;
use crate::primes;
use mod_exp::mod_exp;
use rand::prelude::random;

pub fn make_shares(
    secret: u64,
    share_count: usize,
    recreate_share_count: usize,
) -> (u128, Vec<(u128, u128)>) {
    let coefficients = (1..recreate_share_count)
        .map(|_| random::<u32>())
        .collect::<Vec<_>>();
    let prime = primes::generate(secret.max(share_count as u64) + 1);

    let mut shares: Vec<(u128, u128)> = Vec::with_capacity(share_count);

    for i in 1..=u128::try_from(share_count).expect("usize somehow not fit into u128") {
        let sum = (1..u128::try_from(recreate_share_count)
            .expect("usize somehow not fit into u128"))
            .zip(coefficients.iter())
            .map(|(j, &a)| {
                u128::try_from(a).expect("usize somehow not fit into u128") * mod_exp(i, j, prime)
            })
            .sum::<u128>()
            .rem_euclid(prime)
            .add(u128::from(secret))
            .rem_euclid(prime);

        shares.push((i, sum))
    }

    (prime, shares)
}

pub fn recreate_secret(prime: u128, shares: &[(u128, u128)]) -> u128 {
    shares
        .iter()
        .map(|&(index, share)| share * lagrange_coefficient(index, prime, shares))
        .reduce(|val, next| (val + next).rem_euclid(prime))
        .expect("No shares were given.")
}

fn lagrange_coefficient(i: u128, prime: u128, shares: &[(u128, u128)]) -> u128 {
    // let prime = i128::try_from(prime).expect(format!("Incorrect prime: {prime}").as_str());
    // let x_i = match shares.iter().find(|&&(index, _)| index == i) {
    //     Some(&(_, value)) => i128::try_from(value).expect("u128 not fit into i128"),
    //     None => return 0,
    // };
    let prime = i128::try_from(prime).expect("Incorrect prime");
    let i = i128::try_from(i).expect("Incorrect index");
    let xs = shares
        .iter()
        .map(|&(x, _)| i128::try_from(x).expect("Incorrect index"))
        .filter(|&x| x != i)
        .collect::<Vec<_>>();

    let nominator = xs
        .iter()
        .map(|&x| -x)
        .reduce(|value, next| value * next)
        .expect("No shares given!");

    let denominator = xs
        .iter()
        .map(|x| i - x)
        .reduce(|value, next| value * next)
        .expect("No shares given!");

    if nominator.rem_euclid(denominator) != 0 {
        let denominator = inverse(
            u64::try_from(denominator).expect("denominator not fit into u64"),
            u64::try_from(prime).expect("prime not fit into u64"),
        )
        .expect("Cannot calculate inverse of denominator.");

        (u128::try_from(nominator).expect("i128 not fit into u128") * denominator)
            .rem_euclid(u128::try_from(prime).expect("prime not fit in u128"))
    } else {
        u128::try_from((nominator / denominator).rem_euclid(prime)).expect("prime not fit in u128")
    }
}

#[cfg(test)]
mod test_shamir_share {
    use super::*;

    #[test]
    fn test_calculate_lagrange_coefficient() {
        let shares = [(2, 383), (3, 1045), (4, 308)];
        let coefficients = shares
            .iter()
            .map(|&(index, _)| lagrange_coefficient(index, 1523, &shares))
            .collect::<Vec<_>>();

        assert_eq!(coefficients, vec![6, 1515, 3]);
    }
}
