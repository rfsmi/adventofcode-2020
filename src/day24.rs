use std::collections::HashMap;
use std::collections::HashSet;

use derive_more::Add;
use derive_more::Sub;
use derive_more::Sum;
use itertools::Itertools;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Add, Sub, Sum)]
struct Hex(isize, isize, isize);

impl Hex {
    /*      i
     *      |
     *    /  \
     *   |    |
     *  / \  / \
     * k         j
     */
    const I: Hex = Hex(1, 0, 0);
    const J: Hex = Hex(0, 1, 0);
    const K: Hex = Hex(0, 0, 1);

    fn neighbours(self) -> impl IntoIterator<Item = Hex> {
        [Self::I, Self::J, Self::K]
            .into_iter()
            .tuple_combinations()
            .flat_map(|(a, b)| [a - b, b - a])
            .map(move |vector| self + vector)
    }
}

impl TryFrom<&str> for Hex {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "e" => Ok(Self::J - Self::K),
            "w" => Ok(Self::K - Self::J),
            "se" => Ok(Self::J - Self::I),
            "nw" => Ok(Self::I - Self::J),
            "sw" => Ok(Self::K - Self::I),
            "ne" => Ok(Self::I - Self::K),
            _ => Err(format!("Invalid direction: {value}")),
        }
    }
}

fn iter_vectors(s: &str) -> Vec<Hex> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 1;
    while j <= s.len() && j - i < 3 {
        if let Ok(vector) = s[i..j].try_into() {
            result.push(vector);
            i = j;
        }
        j += 1;
    }
    assert_eq!(i, s.len(), "Invalid directions: {s}");
    result
}

fn generate(input: &str) -> HashSet<Hex> {
    input
        .lines()
        .map(str::trim)
        .map(iter_vectors)
        .map(|v| v.into_iter().sum())
        .fold(HashSet::<Hex>::new(), |mut hs, hex| {
            if hs.contains(&hex) {
                hs.remove(&hex);
            } else {
                hs.insert(hex);
            }
            hs
        })
}

fn mutate(mut black_tiles: HashSet<Hex>) -> HashSet<Hex> {
    let mut black_counts: HashMap<Hex, usize> = HashMap::new();
    for tile in black_tiles.iter() {
        black_counts.entry(*tile).or_default();
        for neighbour in tile.neighbours() {
            *black_counts.entry(neighbour).or_default() += 1;
        }
    }
    for (tile, count) in black_counts {
        match (black_tiles.contains(&tile), count) {
            (true, 0 | 3..) => {
                black_tiles.remove(&tile);
            }
            (false, 2) => {
                black_tiles.insert(tile);
            }
            _ => (),
        };
    }
    black_tiles
}

pub(crate) fn solve(input: &str) -> usize {
    generate(input).len()
}

pub(crate) fn solve_2(input: &str) -> usize {
    (0..100)
        .fold(generate(input), |tiles, _| mutate(tiles))
        .len()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
    ";

    #[test]
    fn test_directions() {
        fn apply_dirs(dirs: &str) -> Hex {
            iter_vectors(dirs).into_iter().sum()
        }
        assert_eq!(apply_dirs("nwwswee"), Hex::default());
        assert_eq!(apply_dirs("esew"), apply_dirs("se"));
    }

    #[test]
    fn test_small() {
        assert_eq!(solve("esew"), 1);
        assert_eq!(solve("nwwswee"), 1);
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 10);
    }

    #[test]
    fn test_neighbours() {
        let neighbours: HashSet<_> = Hex::try_from("w")
            .unwrap()
            .neighbours()
            .into_iter()
            .collect();
        let all_dirs = "
            we
            ww
            wsw
            wse
            wne
            wnw
        ";
        assert_eq!(neighbours, generate(all_dirs));
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 2208);
    }
}
