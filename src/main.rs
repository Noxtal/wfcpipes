// https://miro.medium.com/max/640/1*INn-oJb9fVK2bsiD6J6N7Q.png

use itertools::Itertools;
use rand::seq::{IteratorRandom, SliceRandom};

const TILES: [(char, [u8; 4]); 11] = [
    ('┗', [1, 0, 0, 1]),
    ('┓', [0, 1, 1, 0]),
    ('┏', [0, 0, 1, 1]),
    ('┛', [1, 1, 0, 0]),
    ('━', [0, 1, 0, 1]),
    ('┃', [1, 0, 1, 0]),
    ('┣', [1, 0, 1, 1]),
    ('┫', [1, 1, 1, 0]),
    ('┳', [0, 1, 1, 1]),
    ('┻', [1, 1, 0, 1]),
    ('╋', [1, 1, 1, 1]),
];

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

#[derive(Debug, Clone, Default)]
struct Tile {
    indices: Vec<usize>,
}

impl Tile {
    fn new(size: usize) -> Tile {
        Tile {
            indices: (0..size).collect(),
        }
    }

    fn collapse(&mut self) {
        self.indices = vec![self
            .indices
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()];
    }

    fn entropy(&self) -> usize {
        self.indices.len()
    }

    fn is_collapsed(&self) -> bool {
        self.entropy() == 1
    }
}

fn neighbors(index: usize) -> Vec<(usize, usize)> {
    let mut neighs = Vec::new();
    let (x, y) = (index % WIDTH, index / WIDTH);

    if y < HEIGHT - 1 {
        neighs.push((0, index + WIDTH));
    }
    if x > 0 {
        neighs.push((1, index - 1));
    }
    if y > 0 {
        neighs.push((2, index - WIDTH));
    }
    if x < WIDTH - 1 {
        neighs.push((3, index + 1));
    }

    neighs
}

fn display(grid: &Vec<Tile>) {
    for (i, tile) in grid.into_iter().enumerate() {
        print!(
            "{}",
            if tile.is_collapsed() {
                TILES[tile.indices[0]].0
            } else {
                tile.entropy().to_string().chars().next().unwrap()
            }
        );

        if i % WIDTH == WIDTH - 1 {
            println!();
        }
    }
}

fn main() {
    let mut grid = vec![Tile::new(TILES.len()); WIDTH * HEIGHT];

    loop {
        if wfc(&mut grid) {
            break;
        }
    }

    display(&grid);
}

fn wfc(grid: &mut Vec<Tile>) -> bool {
    let max = grid
        .into_iter()
        .filter(|t| !t.is_collapsed())
        .map(|t| t.entropy())
        .max();

    if max.is_none() {
        return true;
    }

    let max = max.unwrap();

    let (i, tile) = grid
        .iter_mut()
        .enumerate()
        .filter(|t| !t.1.is_collapsed())
        .sorted_by(|a, b| a.1.entropy().cmp(&b.1.entropy()))
        .filter(|t| t.1.entropy() == max)
        .choose(&mut rand::thread_rng())
        .unwrap();

    tile.collapse();

    let mut stack = vec![i];

    while !stack.is_empty() {
        let i = stack.pop().unwrap();

        for j in neighbors(i) {
            if !grid[j.1].is_collapsed() {
                for n in grid[j.1].indices.clone() {
                    for t in grid[i].indices.clone() {
                        if TILES[n].1[(j.0 + 2) % 4] != TILES[t].1[j.0] {
                            grid[j.1].indices.retain(|&x| x != t);

                            if !stack.contains(&j.1) {
                                stack.push(j.1);
                            }
                        }
                    }
                }
            }
        }
    }

    return false;
}
