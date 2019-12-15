use crate::util;
use std::collections::BTreeMap;
use std::cmp::Ordering;

type Ingredient = (usize, String);
type OutputMap = BTreeMap<String, Ingredient>;
type IngredientMap = BTreeMap<Ingredient, Vec<Ingredient>>;
type ResourceMap = BTreeMap<String, usize>;

fn parse_ingredient(ingredient: &str) -> Ingredient {
    let vec: Vec<&str> = ingredient.split(" ").collect();
    (vec[0].parse::<usize>().expect("Not a number"), String::from(vec[1]))
}

fn parse_line(line: &String, output_map: &mut OutputMap, ingredient_map: &mut IngredientMap) {
    let reactions: Vec<&str> = line.split(" => ").collect();
    let ingredients = reactions[0].split(", ");

    let ingredients_list: Vec<Ingredient> = ingredients
        .map(|ingredient| parse_ingredient(ingredient))
        .collect();
    let output = parse_ingredient(reactions[1]);
    output_map.insert(output.1.clone(), output.clone());
    ingredient_map.insert(output, ingredients_list);
}

fn read_input() -> (OutputMap, IngredientMap) {
    let lines = util::read_lines(util::read_file("input/day14.txt"));
    let mut output_map: OutputMap = BTreeMap::new();
    let mut ingredient_map: IngredientMap = BTreeMap::new();
    lines
        .iter()
        .for_each(|line| parse_line(line, &mut output_map, &mut ingredient_map));

    (output_map, ingredient_map)
}

fn calc_required(
    ingredient: &Ingredient,
    required: usize,
    output_map: &OutputMap,
    ingredient_map: &IngredientMap,
    resource_amounts: &mut ResourceMap,
) -> (usize, usize) {
    let leftover = resource_amounts.get(&ingredient.1);
    let mut req = required;
    if leftover.is_some() {
        let leftover = *leftover.unwrap();
        if leftover >= required {
            return (0, 0);
        } else {
            req -= leftover;
        }
    }

    let multiple = req / ingredient.0 + match req % ingredient.0 {
        0 => 0,
        _ => 1,
    };
    let ore = String::from("ORE");

    let v = ingredient_map
        .get(ingredient)
        .expect("Ingredients not found")
        .iter()
        .fold(0, |total, cur_ingredient| -> usize {
            let name = cur_ingredient.1.clone();
            if name == ore {
                return multiple * cur_ingredient.0;
            }

            let output_ingredient = output_map
                .get(&cur_ingredient.1)
                .expect("Expected to find ingredient");

            let calc = calc_required(&output_ingredient, cur_ingredient.0 * multiple, output_map, ingredient_map, resource_amounts);

            if resource_amounts.get(&name).is_none() {
                resource_amounts.insert(name.clone(), 0);
            } else {
                let a = resource_amounts.get(&name).unwrap() + 0;
                let b = cur_ingredient.0 * multiple;
                resource_amounts.insert(name.clone(), a - b);
            }

            total + calc.0
        });

    let amount = match resource_amounts.get(&ingredient.1) {
        Some(a) => a + ingredient.0 * multiple,
        None => ingredient.0 * multiple,
    };
    resource_amounts.insert(ingredient.1.clone(), amount);
    (v, amount)
}

fn bin_search() -> usize {
    let (output_map, ingredient_map) = read_input();
    let one_trill = 1000000000000;

    let mut low = 0;
    let mut high = 2 << 40;
    let mut mid = (high + low) / 2;
    while high - low > 1 {
        let mut resource_map: ResourceMap = BTreeMap::new();
        let required = calc_required(&(1, String::from("FUEL")), mid, &output_map, &ingredient_map, &mut resource_map);
        match required.0.cmp(&one_trill) {
            Ordering::Less => low = mid,
            Ordering::Equal => break,
            Ordering::Greater => high = mid,
        };
        mid = (low + high) / 2;
    }

    mid
}

pub fn part1() {
    let (_a, _b) = read_input();
    let mut resouce_map: ResourceMap = BTreeMap::new();
    let answer = calc_required(&(1, String::from("FUEL")), 1, &_a, &_b, &mut resouce_map);
    println!("part 1: {}", answer.0);
}

pub fn part2() {
    println!("part2: {}", bin_search());
}
