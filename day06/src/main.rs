fn main() {
    let chars = include_str!("../input").char_indices().collect::<Vec<_>>();

    println!("Part 1: {}", find_index(&chars, 4));

    println!("Part 2: {}", find_index(&chars, 14));
}

fn find_index(chars: &[(usize, char)], size: usize) -> usize {
    let mut dupes = chars.windows(size).skip_while(|w| has_duplicate(w)).skip(1);

    dupes.next().unwrap()[size - 1].0
}

fn has_duplicate(chars: &[(usize, char)]) -> bool {
    let mut x: [char; 26] = ['\0'; 26];

    for (_, ch) in chars {
        let idx = *ch as u8 - 97;
        if x[idx as usize] != '\0' {
            return true;
        } else {
            x[idx as usize] = *ch;
        }
    }

    false
}
