use std::collections::{HashMap, HashSet};

use array2d::Array2D;
use take_until::TakeUntilExt;

fn main() {
    let input = include_str!("../input")
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<_>>();

    let grid = Grid(Array2D::from_rows(&input));

    println!("Part 1: {}", grid.visible_trees());

    println!("Part 2: {}", grid.most_scenic());
}

struct Grid(Array2D<u32>);

impl Grid {
    fn visible_trees(&self) -> usize {
        let mut vt = HashSet::new();

        let visible = (self.0.num_columns() * 2 + self.0.num_rows() * 2) - 4;

        let rows = self.0.as_rows();

        for (i, row) in rows.iter().enumerate() {
            if i == 0 || i == rows.len() - 1 {
                continue;
            }
            for (j, v) in row.iter().enumerate() {
                if j == 0 || j == row.len() - 1 {
                    continue;
                }
                let (left, right) = row.split_at(j);
                let right = &right[1..];

                let l = left.iter().all(|x| v > x);
                let r = right.iter().all(|x| v > x);

                if l || r {
                    vt.insert(i * row.len() + j);
                }
            }
        }

        let cols = self.0.as_columns();

        for (i, col) in cols.iter().enumerate() {
            if i == 0 || i == col.len() - 1 {
                continue;
            }
            for (j, v) in col.iter().enumerate() {
                if j == 0 || j == col.len() - 1 {
                    continue;
                }
                let (top, bottom) = col.split_at(j);
                let bottom = &bottom[1..];

                let t = top.iter().all(|y| v > y);
                let b = bottom.iter().all(|y| v > y);

                if t || b {
                    vt.insert(j * col.len() + i);
                }
            }
        }

        visible + vt.len()
    }

    fn most_scenic(&self) -> usize {
        let mut trees = HashMap::new();

        let rows = self.0.as_rows();

        for (i, row) in rows.iter().enumerate() {
            if i == 0 || i == rows.len() - 1 {
                continue;
            }
            for (j, v) in row.iter().enumerate() {
                if j == 0 || j == row.len() - 1 {
                    continue;
                }
                let (left, right) = row.split_at(j);
                let right = &right[1..];

                let l = left.iter().rev().take_until(|&x| x >= v).count();
                let r = right.iter().take_until(|&x| x >= v).count();

                trees.insert(i * row.len() + j, l * r);
            }
        }

        let cols = self.0.as_columns();

        for (i, col) in cols.iter().enumerate() {
            if i == 0 || i == col.len() - 1 {
                continue;
            }
            for (j, v) in col.iter().enumerate() {
                if j == 0 || j == col.len() - 1 {
                    continue;
                }
                let (top, bottom) = col.split_at(j);
                let bottom = &bottom[1..];

                let t = top.iter().rev().take_until(|&y| y >= v).count();
                let b = bottom.iter().take_until(|&y| y >= v).count();

                if let Some(tree) = trees.get_mut(&(j * col.len() + i)) {
                    *tree *= t * b;
                }
            }
        }

        *trees.values().max().unwrap()
    }
}
