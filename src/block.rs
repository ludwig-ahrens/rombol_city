use crate::{SIZE_X, SIZE_Y};

pub struct Block {
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
            dim: [foundation.len(), len_row],
            foundation,
            pos: [0, 0],
            rot: 0,
            tried: false,
            n_rot,
        }
    }

    pub fn set_tried(&mut self) {
        self.tried = true;
    }
    pub fn get_tried(&self) -> bool {
        self.tried
    }
    pub fn get_dim(&self) -> [usize; 2] {
        self.dim
    }
    pub fn get_pos(&self) -> [usize; 2] {
        self.pos
    }
    pub fn get_foundation(&self) -> &Vec<Vec<bool>> {
        &self.foundation
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

fn rotate_foundation(foundation: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let ny = foundation.len();
    let nx = foundation[0].len();
    let mut rotated = Vec::new();
    for i in 0..nx {
        let mut row = Vec::new();
        for j in 0..ny {
            row.push(foundation[ny - 1 - j][i])
        }
        rotated.push(row);
    }
    rotated
}

#[cfg(test)]
mod tests {
    use super::*;
    fn check(a: &Vec<Vec<bool>>, b: &Vec<Vec<bool>>) {
        assert_eq!(a.len(), b.len());
        for i in 0..a.len() {
            assert_eq!(a[i], b[i]);
        }
    }
    #[test]
    fn rot_2x2() {
        let f = vec![vec![true, false], vec![false, false]];
        let rotated = rotate_foundation(&f);
        let target = vec![vec![false, true], vec![false, false]];
        check(&rotated, &target);
    }

    #[test]
    fn rot_2x3() {
        let f = vec![vec![true, false], vec![false, false], vec![true, true]];
        let rotated = rotate_foundation(&f);
        let target = vec![vec![true, false, true], vec![true, false, false]];
        check(&rotated, &target);
    }

    #[test]
    fn rot_4x3() {
        let f = vec![
            vec![true, true, true, false],
            vec![false, true, true, true],
            vec![false, true, true, false],
        ];
        let mut rotated = rotate_foundation(&f);
        let target = vec![
            vec![false, false, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![false, true, false],
        ];
        check(&rotated, &target);
        // rotate back to original
        for _ in 0..3 {
            rotated = rotate_foundation(&rotated);
        }
        check(&rotated, &f);
    }
}
