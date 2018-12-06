mod magicmaze;
mod maze;
mod tile;

use magicmaze::MagicMaze;
use maze::Maze;

pub fn solve_part_1(input: &str) -> usize {
    let maze = input.parse::<Maze>().unwrap();
    let path = maze.solve().unwrap();

    let maze: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| if path.contains(&(row, col)) { c } else { ' ' })
                .collect()
        }).collect();

    for row in maze {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    let steps_needed = path.len() - 1;
    println!("nr. of steps needed: {}", steps_needed);

    steps_needed
}

pub fn solve_part_2(input: &str) -> usize {
    let maze = input.parse::<Maze>().unwrap();
    let maze = MagicMaze::new(maze);

    let steps_needed = maze.solve().unwrap();
    println!("nr. of steps needed: {}", steps_needed);

    steps_needed
}
