fn main() {
    let input = include_str!("../input");

    let fully_contained = input
        .lines()
        .filter_map(line_to_ranges)
        .filter(|(first, second)| first.fully_contained(second) || second.fully_contained(first))
        .count();

    println!("Part 1: {}", fully_contained);

    let any_overlaps = input
        .lines()
        .filter_map(line_to_ranges)
        .filter(|(first, second)| first.any_overlaps(second))
        .count();

    println!("Part 2: {}", any_overlaps);
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    fn fully_contained(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn any_overlaps(&self, other: &Range) -> bool {
        let r: Vec<_> = (self.start..=self.end).collect();

        // n ln n
        for i in other.start..=other.end {
            if r.binary_search(&i).is_ok() {
                return true;
            }
        }
        false
    }
}

fn line_to_ranges(line: &str) -> Option<(Range, Range)> {
    let (first, second) = line.split_once(',')?;

    Some((part_to_range(first)?, part_to_range(second)?))
}

fn part_to_range(part: &str) -> Option<Range> {
    let (start, end) = part.split_once('-')?;
    let (start, end) = (start.parse::<usize>().ok()?, end.parse::<usize>().ok()?);

    Some(Range::new(start, end))
}
