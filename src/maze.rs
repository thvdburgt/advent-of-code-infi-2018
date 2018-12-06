use tile::Tile;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

pub type Grid2D<T> = Vec<Vec<T>>;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Maze {
    pub rows: Grid2D<Tile>,
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        let width = s.lines().next().ok_or(())?.chars().count();
        for line in s.lines() {
            if line.chars().count() != width {
                return Err(());
            }

            let mut row = Vec::new();

            for c in line.split("").skip(1).take(width) {
                row.push(c.parse::<Tile>()?);
            }
            rows.push(row);
        }

        Ok(Self { rows })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct SolveState {
    cost: usize,
    position: (usize, usize),
}

impl Ord for SolveState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for SolveState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SolveState {}

impl Maze {
    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn goal_position(&self) -> (usize, usize) {
        (self.height() - 1, self.width() - 1)
    }

    pub fn get_fresh_dist_table(&self) -> Grid2D<usize> {
        let mut dist: Grid2D<usize> = self
            .rows
            .iter()
            .map(|row| row.iter().map(|_| std::usize::MAX).collect())
            .collect();
        dist[0][0] = 0;

        dist
    }

    pub fn distance_to_goal(&self, position: (usize, usize)) -> usize {
        let (row, col) = position;
        let (goal_row, goal_col) = self.goal_position();

        let (dy, dx) = (goal_row - row, goal_col - col);

        ((dx * dx + dy * dy) as f64).sqrt() as usize
    }

    pub fn neighbours(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let (row, col) = position;
        let cur_tile = &self.rows[row][col];

        let mut neighbours = Vec::new();

        if row != 0 && cur_tile.exit_north && self.rows[row - 1][col].exit_south {
            neighbours.push((row - 1, col));
        }
        if col + 1 != self.width() && cur_tile.exit_east && self.rows[row][col + 1].exit_west {
            neighbours.push((row, col + 1));
        }
        if row + 1 != self.height() && cur_tile.exit_south && self.rows[row + 1][col].exit_north {
            neighbours.push((row + 1, col));
        }
        if col != 0 && cur_tile.exit_west && self.rows[row][col - 1].exit_east {
            neighbours.push((row, col - 1));
        }

        neighbours
    }

    // returns shortest path
    pub fn solve(&self) -> Option<Vec<(usize, usize)>> {
        let goal = (self.height() - 1, self.width() - 1);

        let mut dist: Grid2D<(usize, Option<(usize, usize)>)> = self
            .rows
            .iter()
            .map(|row| row.iter().map(|_| (std::usize::MAX, None)).collect())
            .collect();

        let mut heap = BinaryHeap::new();

        // We're at (0,0), with a zero cost
        dist[0][0] = (0, None);
        heap.push(SolveState {
            cost: 0,
            position: (0, 0),
        });
        while let Some(SolveState { cost, position }) = heap.pop() {
            // check if we are done
            if position == goal {
                let mut pos = position;

                let mut path = vec![position];
                while let Some(prev) = dist[pos.0][pos.1].1 {
                    pos = prev;
                    path.push(pos);
                }
                path.reverse();
                return Some(path);
            }

            let (row, col) = position;

            // check if we already found a shorter way
            if cost > dist[row][col].0 {
                continue;
            }

            for (new_row, new_col) in self.neighbours(position) {
                let state = SolveState {
                    cost: cost + 1,
                    position: (new_row, new_col),
                };

                if state.cost < dist[new_row][new_col].0 {
                    heap.push(state);
                    dist[new_row][new_col] = (state.cost, Some(position));
                }
            }
        }

        None
    }
}
