use std::str::FromStr;

use nom::{
    character::complete::{digit0, multispace0},
    combinator::map_res,
    sequence::{pair, preceded},
    IResult,
};

// Encrypt is used to calculate the following, modulo MOD:
//
//       transform(s, l) ≡ s ^ l
//
// As an optimisation, we note that for l's set bits, b_i:
//                     l = sum(1 << b_i)
//
// Then, transform(s, l) ≡ s ^ sum(1 << b_i)
//                       ≡ prod(s ^ (1 << b_i))
//
// So we calculate each s ^ (1 << b_i) for l, then return
// their product modulo MOD.
struct Encrypt<const MOD: u64> {
    pows: [u64; 64],
}

impl<const MOD: u64> Encrypt<MOD> {
    fn new(subject_number: u64) -> Self {
        let mut pows = [0; 64];
        for exp in 0..64 {
            if exp == 0 {
                pows[exp] = subject_number;
            } else {
                pows[exp] = pows[exp - 1] * pows[exp - 1];
            }
            pows[exp] %= MOD;
        }
        Self { pows }
    }

    fn transform(&self, mut loop_size: u64) -> u64 {
        let mut result = 1;
        while loop_size != 0 {
            let exp = loop_size.trailing_zeros() as usize;
            loop_size &= !(1 << exp);
            result *= self.pows[exp];
            result %= MOD;
        }
        result
    }
}

fn forward(subject_number: u64, loop_size: u64) -> u64 {
    Encrypt::<20201227>::new(subject_number).transform(loop_size)
}

fn backward(subject_number: u64, transformed: u64) -> u64 {
    let cracker = Encrypt::<20201227>::new(subject_number);
    for loop_size in 1.. {
        if cracker.transform(loop_size) == transformed {
            return loop_size;
        }
    }
    panic!("Failed to solve `{subject_number} ^ l ≡ {transformed} (mod 20201227)` for l");
}

fn parse(input: &str) -> (u64, u64) {
    fn num(input: &str) -> IResult<&str, u64> {
        map_res(preceded(multispace0, digit0), u64::from_str)(input)
    }
    pair(num, num)(input).unwrap().1
}

pub(crate) fn solve(input: &str) -> u64 {
    let (a, b) = parse(input);
    let l = backward(7, a);
    forward(b, l)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        5764801
        17807724
    ";

    #[test]
    fn test_parse() {
        assert_eq!(parse(EXAMPLE), (5764801, 17807724));
    }

    #[test]
    fn test_example() {
        assert_eq!(forward(7, 11), 17807724);
        assert_eq!(forward(7, 8), 5764801);
        assert_eq!(forward(17807724, 8), 14897079);
        assert_eq!(forward(5764801, 11), 14897079);
    }

    #[test]
    fn test_crack() {
        assert_eq!(backward(7, 17807724), 11);
        assert_eq!(backward(7, 5764801), 8);
    }
}
