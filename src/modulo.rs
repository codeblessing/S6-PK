#![warn(clippy::pedantic)]

pub fn inverse(inverted: u64, module: u64) -> Result<u128, InversionError> {
    if inverted == 0 {
        return Ok(0);
    }

    let mut inversion = 0;
    let mut next_inversion = 1;
    let mut remainder = i128::from(module);
    let mut next_remainder = i128::from(inverted);

    while next_remainder != 0 {
        let quotient = remainder / next_remainder;
        let t = inversion - (quotient * next_inversion);
        let r = remainder - (quotient * next_remainder);
        inversion = next_inversion;
        next_inversion = t;
        remainder = next_remainder;
        next_remainder = r;
    }

    if remainder > 1 {
        return Err(InversionError::NonCoprimes);
    }

    if inversion < 0 {
        inversion += i128::from(module);
    }

    #[allow(clippy::cast_sign_loss)]
    Ok(inversion as u128)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InversionError {
    NonCoprimes,
}

#[cfg(test)]
mod test_modulo_operations {
    use super::*;

    #[test]
    fn test_correct_modulo_inverse() {
        let mut source = vec![(127, 100), (100, 21), (15, 52)];
        let expected = vec![63, 4, 7];
        let actual: Vec<_> = source
            .drain(..)
            .map(|(inverted, module)| inverse(inverted, module).unwrap())
            .collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_non_coprime_modulo_inverse() {
        let actual = inverse(15, 51);

        assert!(actual.is_err());
        assert_eq!(actual.unwrap_err(), InversionError::NonCoprimes)
    }
}
