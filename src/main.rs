use crate::block::Block;
use crate::field::Field;

pub mod block;
pub mod field;

const SIZE_X: usize = 7;
const SIZE_Y: usize = 7;

fn main() {
    let mut blocks = init_blocks();
    let mut field = Field::new([SIZE_X, SIZE_Y]);
    field.place_pin([2, 2]);
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
