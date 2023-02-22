use std::{
    collections::{HashSet, VecDeque},
    iter::zip,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map, map_res},
    error::ParseError,
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
    IResult,
};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Deck(VecDeque<u32>);

impl Deck {
    fn score(&self) -> u64 {
        let mut result = 0;
        for (i, &card) in self.0.iter().rev().enumerate() {
            result += (i as u64 + 1) * card as u64;
        }
        result
    }

    fn truncate(&self, len: usize) -> Self {
        Self(self.0.iter().copied().take(len).collect())
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Game {
    decks: Vec<Deck>,
    cards: Vec<u32>,
}

impl Game {
    fn new(decks: impl IntoIterator<Item = impl Into<Deck>>) -> Self {
        let mut decks = decks.into_iter().map_into().collect_vec();
        Self {
            cards: decks
                .iter_mut()
                .map(|deck| deck.0.pop_front().unwrap())
                .collect(),
            decks,
        }
    }
}

fn recursive_winner(game: &Game) -> usize {
    let players = zip(&game.cards, &game.decks);
    if players
        .clone()
        .all(|(&card, deck)| card as usize <= deck.0.len())
    {
        let decks = players.map(|(&card, deck)| deck.truncate(card as usize));
        play(Game::new(decks), recursive_winner).0
    } else {
        game.cards
            .iter()
            .enumerate()
            .max_by_key(|(_, &card)| card)
            .unwrap()
            .0
    }
}

fn normal_winner(game: &Game) -> usize {
    game.cards
        .iter()
        .enumerate()
        .max_by_key(|(_, c)| *c)
        .unwrap()
        .0
}

fn play(mut game: Game, winning_player: fn(&Game) -> usize) -> (usize, Deck) {
    let mut seen = HashSet::new();
    loop {
        if !seen.insert(game.clone()) {
            return (0, game.decks[0].clone());
        }
        let winner = winning_player(&game);
        let mut cards = game.cards;
        let mut decks = game.decks;
        cards.swap(winner, 0);
        decks[winner].0.extend(cards);
        if decks.iter().any(|deck| deck.0.is_empty()) {
            return decks
                .into_iter()
                .find_position(|deck| !deck.0.is_empty())
                .unwrap();
        }
        game = Game::new(decks);
    }
}

fn parse(input: &str) -> Game {
    fn ws<'a, O, E: ParseError<&'a str>>(
        inner: impl FnMut(&'a str) -> IResult<&'a str, O, E>,
    ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
        delimited(multispace0, inner, multispace0)
    }
    fn deck(input: &str) -> IResult<&str, Deck> {
        preceded(
            ws(tuple((tag("Player "), digit1, tag(":")))),
            map(many1(ws(map_res(digit1, u32::from_str))), |cards| {
                Deck(cards.into())
            }),
        )(input)
    }
    let (_, game) = map(many0(deck), Game::new)(input).unwrap();
    game
}

pub(crate) fn solve(input: &str) -> u64 {
    let (_, deck) = play(parse(input), normal_winner);
    deck.score()
}

pub(crate) fn solve_2(input: &str) -> u64 {
    let (_, deck) = play(parse(input), recursive_winner);
    deck.score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
    Player 1:
        9
        2
        6
        3
        1
    
    Player 2:
        5
        8
        4
        7
        10
    ";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(EXAMPLE),
            Game::new([Deck([9, 2, 6, 3, 1].into()), Deck([5, 8, 4, 7, 10].into())])
        )
    }

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 306)
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), 291)
    }
}
