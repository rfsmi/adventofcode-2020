use std::{
    cell::RefCell,
    fmt::Display,
    iter::successors,
    rc::{Rc, Weak},
};

use itertools::Itertools;

#[derive(Debug)]
struct Cup {
    dest: Weak<RefCell<Cup>>,
    next: Weak<RefCell<Cup>>,
    value: u32,
}

impl PartialEq for Cup {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

fn next_cups(cup: Rc<RefCell<Cup>>) -> impl Iterator<Item = Rc<RefCell<Cup>>> {
    successors(Some(cup), |cup| cup.borrow().next.upgrade())
}

fn dest_cups(cup: Rc<RefCell<Cup>>) -> impl Iterator<Item = Rc<RefCell<Cup>>> {
    successors(Some(cup), |cup| cup.borrow().dest.upgrade())
}

#[derive(Debug)]
struct Cups {
    size: usize,
    first: Rc<RefCell<Cup>>,
    current: Rc<RefCell<Cup>>,
    cups: Vec<Rc<RefCell<Cup>>>,
}

impl Cups {
    fn new(cups: Vec<u32>) -> Self {
        let cups = cups
            .into_iter()
            .map(|value| {
                Rc::new(RefCell::new(Cup {
                    dest: Weak::new(),
                    next: Weak::new(),
                    value,
                }))
            })
            .collect_vec();
        for (prev, next) in cups.iter().circular_tuple_windows() {
            prev.borrow_mut().next = Rc::downgrade(next);
        }
        let mut sorted_cups = cups.clone();
        sorted_cups.sort_by_key(|cup| cup.borrow().value);
        for (prev, next) in sorted_cups.iter().circular_tuple_windows() {
            next.borrow_mut().dest = Rc::downgrade(prev);
        }
        Self {
            size: cups.len(),
            first: Rc::clone(&sorted_cups[0]),
            current: Rc::clone(&cups[0]),
            cups,
        }
    }

    fn value(&self) -> String {
        let mut result = String::new();
        for cup in next_cups(self.first.clone()).skip(1).take(self.size - 1) {
            result += &cup.borrow().value.to_string();
        }
        result
    }

    fn product(&self) -> u64 {
        next_cups(self.first.clone())
            .skip(1)
            .take(2)
            .map(|cup| cup.borrow().value as u64)
            .product()
    }

    fn iterate(&mut self) {
        let cups = self.pick_up(3);
        let destination = self.get_destination(cups.clone());
        self.put_down(destination, cups);
        let new_current = self.current.borrow().next.upgrade().unwrap();
        self.current = new_current;
    }

    fn get_destination(&self, holding_cup: Rc<RefCell<Cup>>) -> Rc<RefCell<Cup>> {
        let avoid_cups = next_cups(holding_cup).collect_vec();
        for dest in dest_cups(self.current.clone()).skip(1) {
            if avoid_cups.contains(&dest) {
                continue;
            }
            return dest;
        }
        unreachable!()
    }

    fn pick_up(&mut self, count: usize) -> Rc<RefCell<Cup>> {
        let first = next_cups(self.current.clone()).skip(1).next().unwrap();
        let last = next_cups(first.clone()).skip(count - 1).next().unwrap();
        self.current.borrow_mut().next = last.borrow().next.clone();
        last.borrow_mut().next = Weak::new(); // Break the cycle
        first
    }

    fn put_down(&mut self, dest: Rc<RefCell<Cup>>, cup: Rc<RefCell<Cup>>) {
        next_cups(cup.clone()).last().unwrap().borrow_mut().next = dest.borrow().next.clone();
        dest.borrow_mut().next = Rc::downgrade(&cup);
    }
}

impl Display for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        for cup in next_cups(self.cups[0].clone()).take(self.size) {
            if cup == self.current {
                parts.push(format!("({})", cup.borrow().value));
            } else {
                parts.push(format!("{}", cup.borrow().value));
            }
        }
        write!(f, "{}", parts.join(" "))
    }
}

impl TryFrom<(&str, Option<usize>)> for Cups {
    type Error = ();
    fn try_from((value, extend): (&str, Option<usize>)) -> Result<Self, Self::Error> {
        let mut cups = Vec::new();
        for c in value.chars() {
            let digit = c.to_digit(10).ok_or(())?;
            cups.push(digit);
        }
        if let Some(extend) = extend {
            cups.reserve(extend - cups.len());
            for _ in cups.len()..extend {
                cups.push(1 + cups.len() as u32);
            }
        }
        Ok(Cups::new(cups))
    }
}

fn compute(input: &str, extend: Option<usize>, iterations: usize) -> Cups {
    let mut cups: Cups = (input, extend).try_into().unwrap();
    for _ in 0..iterations {
        cups.iterate();
    }
    cups
}

pub(crate) fn solve(input: &str) -> String {
    compute(input, None, 100).value()
}

pub(crate) fn solve_2(input: &str) -> u64 {
    compute(input, Some(1_000_000), 10_000_000).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate() {
        let mut cups: Cups = ("389125467", None).try_into().unwrap();
        cups.iterate();
        assert_eq!(cups.to_string(), "3 (2) 8 9 1 5 4 6 7");
    }

    #[test]
    fn test_example() {
        assert_eq!(compute("389125467", None, 10).value(), "92658374");
        assert_eq!(compute("389125467", None, 100).value(), "67384529");
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2("389125467"), 149245887792);
    }

    #[test]
    fn test_extend() {
        assert_eq!(
            compute("12345", None, 10).value(),
            compute("1", Some(5), 10).value()
        );
    }
}
