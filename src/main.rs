use rombol_city::rotate_foundation;

const SIZE_X: usize = 7;
const SIZE_Y: usize = 7;

fn main() {
    let mut field = Field::new([SIZE_X, SIZE_Y]);
    field.place_pin([1, 2]);
    let mut blocks = vec![
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
    ];
    let mut n_solution: usize = 0;
    loop {
        let mut b = match blocks.pop() {
            Some(block) => block,
            None => field.pop_block().unwrap(), // if at a solution, look for the next
        };
        while !field.block_fits(&b) || b.tried {
            if !b.increment() {
                b.reset();
                blocks.push(b);
                let block_option = field.pop_block();
                b = block_option.unwrap();
            }
        }
        field.place_block(b);
        if blocks.len() == 0 {
            n_solution += 1;
            println!("{}", n_solution);
            field.print();
            println!("");
        }
    }
}

struct Field {
    dim: [usize; 2],
    blocked: Vec<Vec<bool>>,
    blocks: Vec<Block>,
}

impl Field {
    pub fn new(dim: [usize; 2]) -> Self {
        Field {
            dim: [dim[0], dim[1]],
            blocked: vec![vec![false; dim[1]]; dim[0]],
            blocks: vec![],
        }
    }

    pub fn block_fits(&self, b: &Block) -> bool {
        for i in 0..2 {
            assert!(b.pos[i] + b.dim[i] <= self.dim[i]);
        }
        for x in 0..b.dim[0] {
            for y in 0..b.dim[1] {
                if self.blocked[b.pos[0] + x][b.pos[1] + y] && b.foundation[x][y] {
                    return false;
                }
            }
        }
        true
    }

    pub fn place_block(&mut self, mut b: Block) {
        assert!(self.block_fits(&b));
        b.tried = true;
        for x in 0..b.dim[0] {
            for y in 0..b.dim[1] {
                if b.foundation[x][y] {
                    self.blocked[b.pos[0] + x][b.pos[1] + y] = true;
                }
            }
        }
        self.blocks.push(b);
    }
    pub fn pop_block(&mut self) -> Option<Block> {
        match self.blocks.pop() {
            None => None,
            Some(b) => {
                for x in 0..b.dim[0] {
                    for y in 0..b.dim[1] {
                        if b.foundation[x][y] {
                            self.blocked[b.pos[0] + x][b.pos[1] + y] = false;
                        }
                    }
                }
                return Some(b);
            }
        }
    }

    pub fn place_pin(&mut self, pos: [usize; 2]) {
        for i in 0..2 {
            assert!(pos[i] < self.dim[i])
        }
        assert!(!self.blocked[pos[0]][pos[1]]);
        self.blocked[pos[0]][pos[1]] = true;
    }

    pub fn print(&self) {
        let mut table = vec![vec![String::from("/"); self.dim[1]]; self.dim[0]];
        let mut i = 0;
        for b in &self.blocks {
            for x in 0..b.dim[0] {
                for y in 0..b.dim[1] {
                    if b.foundation[x][y] {
                        table[b.pos[0] + x][b.pos[1] + y] = String::from(i.to_string());
                    }
                }
            }
            i += 1;
        }
        for row in &table {
            println!("{:?}", row);
        }
    }
}

struct Block {
    dim: [usize; 2],
    foundation: Vec<Vec<bool>>,
    pos: [usize; 2],
    rot: usize,
    tried: bool,
    n_rot: usize,
}

impl Block {
    pub fn new(foundation: Vec<Vec<bool>>) -> Self {
        let len_row = foundation[0].len();
        for row in &foundation {
            assert_eq!(len_row, row.len())
        }
        let n_rot = if foundation.len() > 1 { 2 } else { 0 } + if len_row > 1 { 2 } else { 0 };
        Block {
            dim: [foundation.len(), len_row], // TODO check all rows same length
            foundation,
            pos: [0, 0],
            rot: 0,
            tried: false,
            n_rot,
        }
    }
    pub fn increment(&mut self) -> bool {
        if self.increment_pos() {
            true
        } else {
            return self.increment_rot();
        }
    }

    fn increment_pos(&mut self) -> bool {
        if self.pos[1] + self.dim[1] < SIZE_Y {
            self.pos[1] += 1;
            self.tried = false;
            return true;
        }
        if self.pos[0] + self.dim[0] < SIZE_X {
            self.pos[0] += 1;
            self.pos[1] = 0;
            self.tried = false;
            return true;
        }
        false
    }
    fn increment_rot(&mut self) -> bool {
        if self.rot == self.n_rot - 1 {
            return false;
        } else {
            self.pos = [0, 0];
            self.rotate();
            self.tried = false;
            return true;
        }
    }

    pub fn reset(&mut self) {
        self.tried = false;
        self.pos = [0, 0];
        while self.rot != 0 {
            self.rotate()
        }
    }

    fn rotate(&mut self) {
        self.rot = (self.rot + 1) % self.n_rot;
        self.foundation = rotate_foundation(&self.foundation);
        self.dim = [self.dim[1], self.dim[0]];
        assert!(self.dim[0] == self.foundation.len());
        assert!(self.dim[1] == self.foundation[0].len());
    }
}
