use crate::block::Block;

pub struct Field {
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
        let [pos0, pos1] = b.get_pos();
        let foundation = b.get_foundation();
        assert!(pos0 + b.get_dim()[0] <= self.dim[0]);
        assert!(pos1 + b.get_dim()[1] <= self.dim[1]);
        for x in 0..b.get_dim()[0] {
            for y in 0..b.get_dim()[1] {
                if self.blocked[pos0 + x][pos1 + y] && foundation[x][y] {
                    return false;
                }
            }
        }
        true
    }

    pub fn place_block(&mut self, mut b: Block) {
        b.set_tried();
        let [pos0, pos1] = b.get_pos();
        let foundation = b.get_foundation();
        for x in 0..b.get_dim()[0] {
            for y in 0..b.get_dim()[1] {
                if foundation[x][y] {
                    assert!(!self.blocked[pos0 + x][pos1 + y]);
                    self.blocked[pos0 + x][pos1 + y] = true;
                }
            }
        }
        self.blocks.push(b);
    }
    pub fn pop_block(&mut self) -> Option<Block> {
        match self.blocks.pop() {
            None => None,
            Some(b) => {
                let [pos0, pos1] = b.get_pos();
                let foundation = b.get_foundation();
                for x in 0..b.get_dim()[0] {
                    for y in 0..b.get_dim()[1] {
                        if foundation[x][y] {
                            assert!(self.blocked[pos0 + x][pos1 + y]);
                            self.blocked[pos0 + x][pos1 + y] = false;
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
            let [pos0, pos1] = b.get_pos();
            let foundation = b.get_foundation();
            for x in 0..b.get_dim()[0] {
                for y in 0..b.get_dim()[1] {
                    if foundation[x][y] {
                        table[pos0 + x][pos1 + y] = String::from(i.to_string());
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
