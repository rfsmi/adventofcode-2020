use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, space0, space1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult,
};

#[derive(PartialEq, Eq, Debug)]
struct IngredientList<'a> {
    ingredients: HashSet<&'a str>,
    allergins: HashSet<&'a str>,
}

fn compute(lists: Vec<IngredientList<'_>>) -> u32 {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for list in &lists {
        for &allergin in &list.allergins {
            map.entry(allergin)
                .and_modify(|i| {
                    *i = list.ingredients.intersection(&i).copied().collect();
                })
                .or_insert(list.ingredients.clone());
        }
    }
    let all_ingredients: HashSet<&str> =
        lists.iter().flat_map(|l| &l.ingredients).copied().collect();
    let risky_ingredients: HashSet<&str> = map.values().flatten().copied().collect();
    let safe_ingredients: HashSet<&str> = all_ingredients
        .difference(&risky_ingredients)
        .copied()
        .collect();
    let mut result = 0;
    for &ingredient in &safe_ingredients {
        for list in &lists {
            if list.ingredients.contains(&ingredient) {
                result += 1;
            }
        }
    }
    result
}

fn compute_2(lists: Vec<IngredientList<'_>>) -> Vec<&str> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    for list in &lists {
        for &allergin in &list.allergins {
            map.entry(allergin)
                .and_modify(|i| {
                    *i = list.ingredients.intersection(&i).copied().collect();
                })
                .or_insert(list.ingredients.clone());
        }
    }
    let mut queue: VecDeque<&str> = map
        .values()
        .filter(|v| v.len() == 1)
        .flatten()
        .copied()
        .collect();
    while let Some(ingredient) = queue.pop_front() {
        for ingredients in map.values_mut() {
            if ingredients.len() == 1 {
                continue;
            }
            ingredients.remove(ingredient);
            if ingredients.len() == 1 {
                queue.extend(ingredients.iter());
            }
        }
    }
    map.iter()
        .sorted_by_key(|(a, _)| *a)
        .flat_map(|(_, v)| v)
        .copied()
        .collect()
}

fn parse(input: &str) -> Vec<IngredientList> {
    fn allergins(input: &str) -> IResult<&str, Vec<&str>> {
        map(
            opt(delimited(
                tag("(contains"),
                separated_list1(tag(","), preceded(space0, alpha1)),
                tag(")"),
            )),
            Option::unwrap_or_default,
        )(input)
    }
    fn ingredients(input: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(space1, alpha1)(input)
    }
    fn ingredient_list(input: &str) -> IResult<&str, IngredientList> {
        let ingredients = preceded(multispace0, ingredients);
        let allergins = preceded(space0, allergins);
        map(pair(ingredients, allergins), |(i, a)| IngredientList {
            ingredients: i.into_iter().collect(),
            allergins: a.into_iter().collect(),
        })(input)
    }
    many1(ingredient_list)(input).unwrap().1
}

pub(crate) fn solve(input: &str) -> u32 {
    let lists = parse(input);
    compute(lists)
}

pub(crate) fn solve_2(input: &str) -> String {
    let lists = parse(input);
    compute_2(lists).join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)
    ";

    #[test]
    fn test_solve() {
        assert_eq!(solve(EXAMPLE), 5);
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(solve_2(EXAMPLE), "mxmxvkd,sqjhc,fvjkl");
    }

    #[test]
    fn test_parse() {
        let lists = parse(EXAMPLE);
        assert_eq!(
            lists,
            vec![
                IngredientList {
                    ingredients: ["mxmxvkd", "kfcds", "sqjhc", "nhms"].into(),
                    allergins: ["dairy", "fish"].into()
                },
                IngredientList {
                    ingredients: ["trh", "fvjkl", "sbzzf", "mxmxvkd"].into(),
                    allergins: ["dairy"].into()
                },
                IngredientList {
                    ingredients: ["sqjhc", "fvjkl"].into(),
                    allergins: ["soy"].into()
                },
                IngredientList {
                    ingredients: ["sqjhc", "mxmxvkd", "sbzzf"].into(),
                    allergins: ["fish"].into()
                },
            ]
        )
    }

    #[test]
    fn test_parse_no_allergins() {
        const EXAMPLE: &str = "
            mxmxvkd kfcds 
            sbzzf mxmxvkd
        ";
        let lists = parse(EXAMPLE);
        assert_eq!(
            lists,
            vec![
                IngredientList {
                    ingredients: ["mxmxvkd", "kfcds"].into(),
                    allergins: [].into()
                },
                IngredientList {
                    ingredients: ["sbzzf", "mxmxvkd"].into(),
                    allergins: [].into()
                },
            ]
        )
    }
}
