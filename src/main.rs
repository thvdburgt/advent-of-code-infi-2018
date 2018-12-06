extern crate advent_of_code_infi_2018 as advent;

fn main() -> std::io::Result<()> {
    // determine input filepath
    let input_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "input".to_string());

    // read input
    let input = std::fs::read_to_string(input_path)?;
    let input = input.trim();

    // solve the puzzles
    println!("======== Part 1 ========");
    advent::solve_part_1(input);

    println!();
    println!("======== Part 2 ========");
    advent::solve_part_2(input);

    Ok(())
}
