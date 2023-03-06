use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, space0, space1},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult,
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct IngredientList<'a> {
    ingredients: Vec<&'a str>,
    allergins: Vec<&'a str>,
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
            ingredients: i,
            allergins: a,
        })(input)
    }
    many1(ingredient_list)(input).unwrap().1
}

pub(crate) fn solve(input: &str) -> u64 {
    let lists = parse(input);
    todo!()
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
    fn test_parse() {
        let lists = parse(EXAMPLE);
        assert_eq!(
            lists,
            vec![
                IngredientList {
                    ingredients: vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                    allergins: vec!["dairy", "fish"]
                },
                IngredientList {
                    ingredients: vec!["trh", "fvjkl", "sbzzf", "mxmxvkd"],
                    allergins: vec!["dairy"]
                },
                IngredientList {
                    ingredients: vec!["sqjhc", "fvjkl"],
                    allergins: vec!["soy"]
                },
                IngredientList {
                    ingredients: vec!["sqjhc", "mxmxvkd", "sbzzf"],
                    allergins: vec!["fish"]
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
                    ingredients: vec!["mxmxvkd", "kfcds"],
                    allergins: vec![]
                },
                IngredientList {
                    ingredients: vec!["sbzzf", "mxmxvkd"],
                    allergins: vec![]
                },
            ]
        )
    }
}
