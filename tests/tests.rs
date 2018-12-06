extern crate advent_of_code_infi_2018;

use advent_of_code_infi_2018::*;

const SAMPLE_MAZE: &str = "\
╔═╗║
╠╗╠║
╬╬╣╬
╚╩╩═\
";

#[test]
fn part_1_example() {
    assert_eq!(solve_part_1(SAMPLE_MAZE), 6);
}

#[test]
fn part_2_example() {
    assert_eq!(solve_part_2(SAMPLE_MAZE), 4);
}
const INPUT_MAZE: &str = "\
╠╦═╗═╚╣═╔═╦╝╠╗╔╝╚╩═╚
║═╠╠╣╝╔╝╝╗╩╚║╣╝╝╚╚╔╔
╚═╦║╚╔║═╠╩╠╗╚╩║╚╗╣╔╝
═╚╩╚╗╗╦╦╦╦╠╗╝╠║╚╝║╔╗
╣╣╔╝╣║╬╗╩═╝╗╩╚╩╚╗╝═╦
══╠╩╣╠╩╠╗║╝║╚══╦╔╗║═
╦╠╗╝╠═╦╩╚╗╚╩╠╬║╚╩╠╦╚
╗║╩║║╬╔╦╩╬═╦╗╗╝╝╗╔╩║
╠╗╗╚╗═╬═╝═║╠╝══╣╠║╦║
╝╦╚╣╩═╚═╩═╦╠║╔║╣═╠╗╣
╠╚═╠╠╦╩╩═╗╣╚╣╣╔╠╣╣╩╦
═╩═╬╣╠║╣╠╚╗═║╗╝═╦║═╝
╠╬║╝║╚╝╝╣╝╦╚╬╩══╦╩╗╣
╣╩╗╝╣╝╗╝╔╚╝╠╚╩╗╬║╝╣╝
║╗╠═╗╗╦╚╠╗══╚╣╠╚╔╦║╠
║╚╠╠╣╝═╠╣║╩╔╠╗╬╣╗╔╬╗
╠╚╚╚║╚╗╝═╣╝═║╚╦╦╗╔╝╗
╗║═║╠╩╗╩═╣╝╔╝╬╠╗╚╣╔╚
╚╦╣╚╝║╦═╩╗═╩╚╗╠═╚╚╣╩
═╗║╚╠╚╝╝╝║╠╝═╬╣═╠║╬╩\
";

#[test]
fn part_1() {
    assert_eq!(solve_part_1(INPUT_MAZE), 42);
}

#[test]
fn part_2() {
    assert_eq!(solve_part_2(INPUT_MAZE), 47);
}
