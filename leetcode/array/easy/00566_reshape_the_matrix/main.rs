impl Solution {
    pub fn matrix_reshape(
        mat: Vec<Vec<i32>>,
        rows: i32,
        cols: i32
    ) -> Vec<Vec<i32>> {
        let cols = cols as usize;
        let rows = rows as usize;
        if mat.len() * mat[0].len() != rows * cols {
            return mat;
        }

        let mut ret = Vec::new();
        for r in 0..rows {
            let mut row = Vec::new();
            for c in 0..cols {
                let element_number = r * cols + c;

                let i = element_number / mat[0].len();
                let g = element_number % mat[0].len();

                row.push(mat[i][g]);
            }
            ret.push(row);
        }

        ret
    }
}