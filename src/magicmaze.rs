use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use maze::*;

pub struct MagicMaze {
    base_maze: Maze,
    period: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct SolveState {
    maze: Maze,
    steps: usize,
    position: (usize, usize),
}

impl Ord for SolveState {
    fn cmp(&self, other: &Self) -> Ordering {
        let score_other = other.steps + self.maze.distance_to_goal(other.position);
        let score_self = self.steps + self.maze.distance_to_goal(self.position);

        score_other
            .cmp(&score_self)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for SolveState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

enum ShiftTarget {
    Row(usize),
    Col(usize),
}

impl MagicMaze {
    pub fn new(base_maze: Maze) -> Self {
        let period = std::cmp::min(base_maze.width(), base_maze.height());
        Self { base_maze, period }
    }

    fn neighbours(&self, state: &SolveState) -> Vec<(usize, usize)> {
        let mut neighbours = state.maze.neighbours(state.position);

        let shift_target = self.shift_after_steps(state.steps);
        for neighbour in &mut neighbours {
            let (row, col) = neighbour;

            match shift_target {
                ShiftTarget::Row(shift_row) => {
                    if *row == shift_row {
                        *col = (*col + 1) % state.maze.width()
                    }
                }
                ShiftTarget::Col(shift_col) => {
                    if *col == shift_col {
                        *row = (*row + 1) % state.maze.height()
                    }
                }
            }
        }

        neighbours
    }

    fn shift_row(maze: &mut Maze, row: usize) {
        let row = &mut maze.rows[row];
        row.rotate_right(1);
    }

    fn shift_col(maze: &mut Maze, col: usize) {
        let height = maze.height();
        if height == 0 {
            return;
        }

        let new_top = maze.rows[height - 1][col].clone();
        for row in (1..height).rev() {
            maze.rows[row][col] = maze.rows[row - 1][col].clone();
        }
        maze.rows[0][col] = new_top;
    }

    fn shift_after_steps(&self, step: usize) -> ShiftTarget {
        let step = step % self.period;
        if step % 2 == 0 {
            ShiftTarget::Row(step)
        } else {
            ShiftTarget::Col(step)
        }
    }

    fn get_next_maze(&self, state: &SolveState) -> Maze {
        let mut next_maze = state.maze.clone();
        match self.shift_after_steps(state.steps) {
            ShiftTarget::Row(row) => Self::shift_row(&mut next_maze, row),
            ShiftTarget::Col(col) => Self::shift_col(&mut next_maze, col),
        }

        next_maze
    }

    pub fn solve(&self) -> Option<usize> {
        let mut heap = BinaryHeap::new();
        let mut dists: HashMap<Maze, Grid2D<usize>> = HashMap::new();

        heap.push(SolveState {
            maze: self.base_maze.clone(),
            steps: 0,
            position: (0, 0),
        });

        let goal = (self.base_maze.height() - 1, self.base_maze.width() - 1);

        let mut shortest_path_len = None;
        while let Some(state) = heap.pop() {
            // check if we arived at the goal
            if state.position == goal {
                if let Some(path_len) = shortest_path_len {
                    if state.steps < path_len {
                        shortest_path_len = Some(state.steps);
                    }
                } else {
                    shortest_path_len = Some(state.steps);
                }
                continue;
            }

            // check if we already found a shorter way
            if let Some(path_len) = shortest_path_len {
                // to the goal
                if state.steps > path_len {
                    continue;
                }
            }
            if let Some(dist) = dists.get(&state.maze) {
                // or to this square
                let (row, col) = state.position;
                if state.steps > dist[row][col] {
                    continue;
                }
            }

            // create maze configuration on next step
            let next_maze = self.get_next_maze(&state);

            // add possible paths to heap
            let dists = dists
                .entry(next_maze.clone())
                .or_insert_with(|| next_maze.get_fresh_dist_table());
            for (new_row, new_col) in self.neighbours(&state) {
                let state = SolveState {
                    maze: next_maze.clone(),
                    steps: state.steps + 1,
                    position: (new_row, new_col),
                };

                if state.steps < dists[new_row][new_col] {
                    dists[new_row][new_col] = state.steps;
                    heap.push(state);
                }
            }
        }

        shortest_path_len
    }
}
