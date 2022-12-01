use itertools::Itertools;

fn main() {
    let input = include_str!("../input").lines().group_by(|x| x.is_empty());

    let calories: Vec<_> = input
        .into_iter()
        .map(|(_, c)| c.filter_map(|c| c.parse::<i64>().ok()))
        .map(|c| c.sum::<i64>())
        .collect();

    println!("Part 1: {:?}", calories.iter().max());

    println!(
        "Part 2: {:?}",
        calories.iter().sorted().rev().take(3).sum::<i64>()
    )
}
