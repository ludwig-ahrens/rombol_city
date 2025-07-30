pub fn rotate_foundation(foundation: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
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
        // rotate back to original
        check(&rotated, &target);
        for i in 0..3 {
            rotated = rotate_foundation(&rotated);
        }
        check(&rotated, &f);
    }
}
