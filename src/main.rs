use rombol_city::rotate_foundation;
const SIZE_X: usize = 7;
const SIZE_Y: usize = 7;
fn main() {
    let mut field = Field::new([SIZE_X, SIZE_Y]);
    field.place_pin([0, 1]);
    let mut blocks = vec![
        Block::new(vec![vec![true, true, true, true, true]]),
        Block::new(vec![vec![true, true, true], vec![false, true, false]]),
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
            vec![true, true, true, true],
            vec![false, false, true, false],
        ]),
        Block::new(vec![
            vec![true, true, true, true],
            vec![false, true, true, false],
        ]),
        Block::new(vec![
            vec![true, true, true, false],
            vec![false, true, true, true],
            vec![false, true, true, false],
        ]),
    ];
    while blocks.len() > 0 {
        let mut b = blocks.pop().unwrap();
        let mut tmp = false; // TODO tidy up
        while !field.block_fits(&b) || tmp {
            tmp = false;
            if !b.increment() {
                b.reset();
                blocks.push(b);
                b = field.pop_block().unwrap();
                tmp = true; // this position has been tried before; increment before placing
                //println!(
                    //"Removed block, {}, {}",
                    //blocks.len() + 1,
                    //field.blocks.len()
                //);
                //field.print();
            }
        }
        field.place_block(b);
        //println!("Placed block, {}, {}", blocks.len(), field.blocks.len());
        //field.print();
    }
    field.print();
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
                    //println!("Blocked at: {}, {}",b.pos[0] + x, b.pos[1] + y);
                    return false;
                }
            }
        }
        true
    }

    pub fn place_block(&mut self, b: Block) {
        assert!(self.block_fits(&b));
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
        println!("");
    }
}

struct Block {
    dim: [usize; 2],
    foundation: Vec<Vec<bool>>,
    pos: [usize; 2],
    rot: usize,
}

impl Block {
    pub fn new(foundation: Vec<Vec<bool>>) -> Self {
        Block {
            dim: [foundation.len(), foundation[0].len()], // TODO check all rows same length
            foundation,
            pos: [0, 0],
            rot: 0,
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
            return true;
        }
        if self.pos[0] + self.dim[0] < SIZE_X {
            self.pos[0] += 1;
            self.pos[1] = 0;
            return true;
        }
        false
    }
    fn increment_rot(&mut self) -> bool {
        if self.rot == 3 {
            return false;
        } else {
            self.pos = [0, 0];
            self.rotate();
            return true;
        }
    }

    pub fn reset(&mut self) {
        self.pos = [0, 0];
        while self.rot != 0 {
            self.rotate()
        }
    }

    fn rotate(&mut self) {
        self.rot = (self.rot + 1) % 4;
        self.foundation = rotate_foundation(&self.foundation);
        self.dim = [self.dim[1], self.dim[0]];
        assert!(self.dim[0] == self.foundation.len());
        assert!(self.dim[1] == self.foundation[0].len());
    }

    //pub fn print(&self) {
        //println!("{:?}", self.pos);
        //println!("{:?}", self.rot);
    //}
}
