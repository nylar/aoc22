use std::collections::HashSet;

fn main() {
    let rucksack_compartments = include_str!("../input")
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(f, s)| (priorities(f), priorities(s)));

    println!(
        "Part 1: {}",
        rucksack_compartments
            .filter_map(|(f, s)| compartment_matches2(&f, &s))
            .map(|(_, priority)| priority as i64)
            .sum::<i64>()
    );

    let rucksack_lines = include_str!("../input").lines().collect::<Vec<_>>();

    println!(
        "Part 2: {}",
        rucksack_lines
            .chunks(3)
            .map(|g| (priorities(g[0]), priorities(g[1]), priorities(g[2])))
            .filter_map(|(f, s, t)| compartment_matches3(&f, &s, &t))
            .map(|(_, priority)| priority as i64)
            .sum::<i64>(),
    );
}

type Pair = (char, u8);

fn compartment_matches2(first: &[Pair], second: &[Pair]) -> Option<Pair> {
    let f: HashSet<_> = first.iter().collect();
    let s: HashSet<_> = second.iter().collect();

    let intersection = &f & &s;
    intersection.into_iter().next().copied()
}

fn compartment_matches3(first: &[Pair], second: &[Pair], third: &[Pair]) -> Option<Pair> {
    let f: HashSet<_> = first.iter().collect();
    let s: HashSet<_> = second.iter().collect();
    let t: HashSet<_> = third.iter().collect();

    f.intersection(&s)
        .copied()
        .find(|i| t.contains(&**i))
        .copied()
}

fn priorities(s: &str) -> Vec<Pair> {
    s.chars().map(item_priority).collect()
}

fn item_priority(item: char) -> Pair {
    match item {
        'a'..='z' => (item, item as u8 - 96),
        'A'..='Z' => (item, item as u8 - 38),
        _ => panic!("{} is not a valid item", item),
    }
}
