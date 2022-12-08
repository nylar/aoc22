use std::collections::HashMap;

fn main() {
    let mut sizes = HashMap::new();
    sizes.insert("/".to_owned(), 0);
    let mut stack = vec!["/"];

    for line in include_str!("../input").lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        match parts[..] {
            ["$", "cd", ".."] => {
                stack.pop();
            }
            ["$", "cd", "/"] => {
                stack = vec!["/"]; // reset stack
            }
            ["$", "cd", dir] => {
                stack.push(dir);
                sizes.insert(stack.join(""), 0);
            }
            [size, _] if matches!(size.chars().next(), Some('0'..='9')) => {
                let size = size.parse::<usize>().unwrap();

                let mut s = stack.clone();

                // Add to leaf
                if let Some(count) = sizes.get_mut(&s.join("")) {
                    *count += size;
                }

                // Add to parents
                while s.pop().is_some() {
                    if let Some(count) = sizes.get_mut(&s.join("")) {
                        *count += size;
                    }
                }
            }
            _ => {} // ignore everything else
        }
    }

    let total: usize = sizes.values().filter(|s| **s <= 100_000).sum();

    println!("Part 1: {}", total);

    let free_space = sizes
        .values()
        .filter(|s| **s > sizes["/"] - 40_000_000)
        .min();

    println!("Part 2: {}", free_space.unwrap());
}
