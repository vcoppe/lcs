pub struct LcsDp<'a> {
    pub n_chars: usize,
    pub a: &'a Vec<usize>,
    pub b: &'a Vec<usize>,
}

impl<'a> LcsDp<'a> {
    pub fn solve(&self) -> Vec<Vec<isize>> {
        let mut table = vec![vec![-1; self.b.len() + 1]; self.a.len() + 1];
        for i in 0..=self.a.len() {
            table[i][self.b.len()] = 0;
        }
        for j in 0..=self.b.len() {
            table[self.a.len()][j] = 0;
        }
        self._solve(&mut table, 0, 0);
        table
    }

    fn _solve(&self, table: &mut Vec<Vec<isize>>, i: usize, j: usize) -> isize {
        if table[i][j] != -1 {
            return table[i][j];
        }
        
        let mut best = 0;
        
        best = best.max(self._solve(table, i + 1, j));
        best = best.max(self._solve(table, i, j + 1));
        best = best.max(
            self._solve(table, i + 1, j + 1)
            + (self.a[i] == self.b[j]) as isize
        );

        table[i][j] = best;

        best
    }
}