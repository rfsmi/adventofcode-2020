use std::str::FromStr;

use nom::{
    character::complete::{digit0, multispace0},
    combinator::map_res,
    sequence::{delimited, pair},
    IResult,
};

// Cracker is used to calculate the following, modulo MOD:
//
//        transform(s, l) = s ^ l
//
// Consider that, for l's set bit indicies, b_i:
//                      l = sum(1 << b_i)
//
// Now,             s ^ l = s ^ sum(1 << b_i)
//                        = prod(s ^ (1 << b_i))
//
// So we calculate each (1 << b_i) for l, then return their
// product mod MOD.
struct Cracker<const MOD: u64> {
    pows: [u64; 64],
}

impl<const MOD: u64> Cracker<MOD> {
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

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    Cracker::<20201227>::new(subject_number).transform(loop_size)
}

fn get_loop_size(public_key: u64) -> u64 {
    let cracker = Cracker::<20201227>::new(7);
    for loop_size in 1.. {
        if cracker.transform(loop_size) == public_key {
            return loop_size;
        }
    }
    panic!("Failed to find loop size for {public_key}")
}

fn parse(input: &str) -> (u64, u64) {
    fn num(input: &str) -> IResult<&str, u64> {
        map_res(delimited(multispace0, digit0, multispace0), u64::from_str)(input)
    }
    pair(num, num)(input).unwrap().1
}

pub(crate) fn solve(input: &str) -> u64 {
    let (a, b) = parse(input);
    let a_loop_size = get_loop_size(a);
    let encryption_key = transform(b, a_loop_size);
    encryption_key
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
        assert_eq!(transform(7, 11), 17807724);
        assert_eq!(transform(7, 8), 5764801);
        assert_eq!(transform(17807724, 8), 14897079);
        assert_eq!(transform(5764801, 11), 14897079);
    }

    #[test]
    fn test_crack() {
        assert_eq!(get_loop_size(17807724), 11);
        assert_eq!(get_loop_size(5764801), 8);
    }
}
