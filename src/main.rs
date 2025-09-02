use crate::block::Block;
use crate::field::Field;
use std::io;

pub mod block;
pub mod field;

const SIZE_X: usize = 7;
const SIZE_Y: usize = 7;

fn main() {
    // setup
    let mut blocks = init_blocks();
    let mut field = Field::new([SIZE_X, SIZE_Y]);
    // pin position from user
    println!("Enter horizontal pin position");
    let x = user_input(SIZE_X);
    println!("Enter vertical pin position");
    let y = user_input(SIZE_X);
    field.place_pin([y, x]);
    // find solutions with back tracking
    let n = find_solutions(&mut field, &mut blocks);
    println!("{} solutions found", n);
}

fn find_solutions(field: &mut Field, blocks: &mut Vec<Block>) -> usize {
    let mut n_solution: usize = 0;
    loop {
        let mut b = match blocks.pop() {
            Some(block) => block,
            None => field.pop_block().unwrap(), // if at a solution, look for the next
        };
        // check if block can be placed, if not move it to the next position
        while !field.block_fits(&b) || b.get_tried() {
            // if block tried in all positions, put it back and take back a previously placed block
            if !b.increment() {
                b.reset();
                blocks.push(b);
                let block_option = field.pop_block();
                // if the first block cannot be incremented, everything has been tried
                if block_option.is_none() {
                    return n_solution;
                }
                b = block_option.unwrap();
            }
        }
        field.place_block(b);
        if blocks.len() == 0 {
            n_solution += 1;
            field.print();
            println!("");
        }
    }
}

fn user_input(n_max: usize) -> usize {
    let mut number: usize = n_max;
    while number >= n_max {
        println!("Enter a value between 0 and {}", n_max - 1);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(n) => {
                number = n;
            }
            Err(e) => {
                println!("Not a valid input: {}", e);
            }
        }
    }
    number
}

fn init_blocks() -> Vec<Block> {
    // list of all blocks in the game
    // large blocks are at the end, so they are placed first
    vec![
        Block::new(vec![vec![true, true]]),
        Block::new(vec![vec![true, true, true, true, true]]),
        Block::new(vec![vec![true, true, true], vec![false, true, false]]),
        Block::new(vec![
            vec![true, true, true, true],
            vec![false, false, true, false],
        ]),
        Block::new(vec![
            vec![true, true, true, true],
            vec![false, true, true, false],
        ]),
        Block::new(vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, false],
        ]),
        Block::new(vec![
            vec![false, true, true],
            vec![true, true, false],
            vec![true, false, false],
        ]),
        Block::new(vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![false, true, false],
        ]),
        Block::new(vec![
            vec![true, true, true, false],
            vec![false, true, true, true],
            vec![false, true, true, false],
        ]),
    ]
}
