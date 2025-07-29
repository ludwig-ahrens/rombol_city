use rombol_city::rotate_foundation;
const SIZE_X: usize = 7;
const SIZE_Y: usize = 7;
fn main() {
    let mut field = Field::new([SIZE_X, SIZE_Y]);
    field.place_pin([0, 1]);
    let mut blocks = vec![
        Block::new(vec![
            vec![true, true, true],
            vec![true, true, false],
            vec![true, false, false],
        ]),
        Block::new(vec![
            vec![true, true, true, false],
            vec![false, true, true, true],
            vec![false, true, true, false],
        ]),
    ];
    while blocks.len() > 0 {
        let mut b = blocks.pop().unwrap();
        let pos_start = b.pos;
        while !field.block_fits(&b) {
            let incremented = b.increment_pos();
            if !incremented {} // TODO
        }
        field.place_block(b);
    }
    field.print_bool();
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

    pub fn place_pin(&mut self, pos: [usize; 2]) {
        for i in 0..2 {
            assert!(pos[i] < self.dim[i])
        }
        assert!(!self.blocked[pos[0]][pos[1]]);
        self.blocked[pos[0]][pos[1]] = true;
    }

    pub fn print_bool(&self) {
        for row in &self.blocked {
            println!("{:?}", row);
        }
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
    pub fn increment_pos(&mut self) -> bool {
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
    pub fn increment_rot(&mut self) -> bool {
        if self.rot == 3 {
            return false;
        } else {
            self.rotate();
            return true;
        }
    }

    pub fn reset(&mut self) {
        self.pos = [0, 0];
    }

    fn rotate(&mut self) {
        self.rot = (self.rot + 1) % 4;
        self.foundation = rotate_foundation(&self.foundation);
        self.dim = [self.dim[1], self.dim[0]];
        assert!(self.dim[0] == self.foundation.len());
        assert!(self.dim[1] == self.foundation[0].len());
    }
}

