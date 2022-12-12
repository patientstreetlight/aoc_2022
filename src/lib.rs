use std::{
    ops::{Index, IndexMut},
    str::Chars,
};

pub struct Grid<T> {
    elems: Vec<T>,
    num_rows: usize,
    num_cols: usize,
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        assert!(r < self.num_rows);
        assert!(c < self.num_cols);
        &self.elems[r * self.num_cols + c]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        assert!(r < self.num_rows);
        assert!(c < self.num_cols);
        &mut self.elems[r * self.num_cols + c]
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(rows: Vec<Vec<T>>) -> Self {
        let mut elems = vec![];
        let num_rows = rows.len();
        assert!(num_rows > 0);
        let num_cols = rows[0].len();
        assert!(num_cols > 0);
        for row in rows {
            assert_eq!(row.len(), num_cols);
            for elem in row {
                elems.push(elem);
            }
        }
        Grid {
            elems,
            num_rows,
            num_cols,
        }
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.elems.chunks(self.num_cols) {
            for elem in row {
                elem.fmt(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct GridEnumeratedElems<'a, T> {
    grid: &'a Grid<T>,
    idx: usize,
}

impl<'a, T> Iterator for GridEnumeratedElems<'a, T> {
    type Item = ((usize, usize), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.grid.elems.len() {
            let r = self.idx / self.grid.num_cols;
            let c = self.idx % self.grid.num_cols;
            let val = &self.grid.elems[self.idx];
            self.idx += 1;
            Some(((r, c), val))
        } else {
            None
        }
    }
}

impl<T> Grid<T> {
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn enumerated_elems(&self) -> GridEnumeratedElems<T> {
        GridEnumeratedElems { grid: self, idx: 0 }
    }

    pub fn from_str<F>(s: &str, mut read_one: F) -> Grid<T>
    where
        F: FnMut((usize, usize), &mut Chars) -> Option<T>,
    {
        let mut num_rows = 0;
        let mut num_cols = None;
        let mut elems = vec![];
        for (r, line) in s.lines().enumerate() {
            let mut cols_in_this_row = 0;
            let mut cs = line.chars();
            while let Some(e) = read_one((r, cols_in_this_row), &mut cs) {
                elems.push(e);
                cols_in_this_row += 1;
            }
            match num_cols {
                None => num_cols = Some(cols_in_this_row),
                Some(cs) => assert!(cs == cols_in_this_row),
            }
            num_rows += 1;
        }
        let num_cols = num_cols.unwrap();
        Grid {
            elems,
            num_rows,
            num_cols,
        }
    }

    pub fn map<F, T2>(&self, map_elem: F) -> Grid<T2>
    where
        F: FnMut(&T) -> T2,
    {
        let elems = self.elems.iter().map(map_elem).collect();
        Grid {
            elems,
            num_rows: self.num_rows,
            num_cols: self.num_cols,
        }
    }

    pub fn neighbors4(&self, src: (usize, usize)) -> Neighbors4<T> {
        Neighbors4 {
            grid: self,
            src,
            idx: 0,
        }
    }

    pub fn neighbors8(&self, src: (usize, usize)) -> Neighbors8<T> {
        Neighbors8 {
            grid: self,
            src,
            idx: 0,
        }
    }

    pub fn row(&self, row: usize) -> Row<T> {
        assert!(row < self.num_rows);
        Row::new(self, row)
    }

    pub fn col(&self, col: usize) -> Col<T> {
        assert!(col < self.num_cols);
        Col::new(self, col)
    }

    pub fn rows(&self) -> Rows<T> {
        Rows::new(self)
    }

    pub fn cols(&self) -> Cols<T> {
        Cols::new(self)
    }
}

pub struct Cols<'a, T> {
    grid: &'a Grid<T>,
    front_col: usize,
    back_col: usize,
}

impl<'a, T> Cols<'a, T> {
    fn new(grid: &'a Grid<T>) -> Cols<'a, T> {
        Cols {
            grid,
            front_col: 0,
            back_col: grid.num_cols,
        }
    }
}

impl<'a, T> Iterator for Cols<'a, T> {
    type Item = Col<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_col == self.back_col {
            return None;
        }
        let col = Col::new(self.grid, self.front_col);
        self.front_col += 1;
        Some(col)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back_col - self.front_col;
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for Cols<'a, T> {
}

impl<'a, T> DoubleEndedIterator for Cols<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front_col == self.back_col {
            return None;
        }
        self.back_col -= 1;
        let col = Col::new(self.grid, self.back_col);
        Some(col)
    }
}

pub struct Rows<'a, T> {
    grid: &'a Grid<T>,
    front_row: usize,
    back_row: usize,
}

impl<'a, T> Rows<'a, T> {
    fn new(grid: &'a Grid<T>) -> Rows<'a, T> {
        Rows {
            grid,
            front_row: 0,
            back_row: grid.num_rows,
        }
    }
}

impl<'a, T> Iterator for Rows<'a, T> {
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_row == self.back_row {
            return None;
        }
        let row = Row::new(self.grid, self.front_row);
        self.front_row += 1;
        Some(row)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back_row - self.front_row;
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for Rows<'a, T> {
}

impl<'a, T> DoubleEndedIterator for Rows<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front_row == self.back_row {
            return None;
        }
        self.back_row -= 1;
        let row = Row::new(self.grid, self.back_row);
        Some(row)
    }
}

pub struct Col<'a, T> {
    grid: &'a Grid<T>,
    col: usize,
    front_row: usize,
    back_row: usize,
}

impl<'a, T> Col<'a, T> {
    fn new(grid: &'a Grid<T>, col: usize) -> Col<'a, T> {
        Col {
            grid,
            col,
            front_row: 0,
            back_row: grid.num_rows,
        }
    }
}

impl<'a, T> Iterator for Col<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_row == self.back_row {
            return None;
        }
        let ret = &self.grid[(self.front_row, self.col)];
        self.front_row += 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back_row - self.front_row;
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for Col<'a, T> {
}

impl<'a, T> DoubleEndedIterator for Col<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front_row == self.back_row {
            return None;
        }
        self.back_row -= 1;
        let ret = &self.grid[(self.back_row, self.col)];
        Some(ret)
    }
}

pub struct Row<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    front_col: usize,
    back_col: usize,
}

impl<'a, T> Row<'a, T> {
    fn new(grid: &'a Grid<T>, row: usize) -> Row<'a, T> {
        Row {
            grid,
            row,
            front_col: 0,
            back_col: grid.num_cols,
        }
    }
}

impl<'a, T> Iterator for Row<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front_col == self.back_col {
            return None;
        }
        let ret = &self.grid[(self.row, self.front_col)];
        self.front_col += 1;
        Some(ret)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.back_col - self.front_col;
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for Row<'a, T> {
}

impl<'a, T> DoubleEndedIterator for Row<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front_col == self.back_col {
            return None;
        }
        self.back_col -= 1;
        let ret = &self.grid[(self.row, self.back_col)];
        Some(ret)
    }
}

pub struct Neighbors8<'a, T> {
    grid: &'a Grid<T>,
    src: (usize, usize),
    idx: u8,
}

impl<'a, T> Iterator for Neighbors8<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (r, c) = self.src;
        loop {
            if self.idx > 8 {
                return None;
            }
            let i = self.idx as isize;
            self.idx += 1;
            let dr = -1 + i / 3;
            let dc = -1 + i % 3;
            if dr == 0 && dc == 0 {
                continue;
            }
            let r = r as isize + dr;
            let c = c as isize + dc;
            if r < 0 || c < 0 {
                continue;
            }
            let r = r as usize;
            let c = c as usize;
            if r >= self.grid.num_rows || c >= self.grid.num_cols {
                continue;
            }
            return Some((r, c));
        }
    }
}

pub struct Neighbors4<'a, T> {
    grid: &'a Grid<T>,
    src: (usize, usize),
    idx: u8,
}

impl<'a, T> Iterator for Neighbors4<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > 3 {
            return None;
        }
        let (r, c) = self.src;
        loop {
            let i = self.idx;
            self.idx += 1;
            match i {
                0 => {
                    if r == 0 {
                        continue;
                    }
                    return Some((r - 1, c));
                }
                1 => {
                    if c == self.grid.num_cols - 1 {
                        continue;
                    }
                    return Some((r, c + 1));
                }
                2 => {
                    if r == self.grid.num_rows - 1 {
                        continue;
                    }
                    return Some((r + 1, c));
                }
                3 => {
                    if c == 0 {
                        continue;
                    }
                    return Some((r, c - 1));
                }
                _ => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "1234\n5678";
        let mut count = 0;
        let mut grid = Grid::from_str(input, |_, cs| {
            cs.next().map(|c| {
                count += 1;
                c.to_digit(10).unwrap()
            })
        });
        assert_eq!(4, grid.num_cols);
        assert_eq!(2, grid.num_rows);
        assert_eq!(8, count);
        assert_eq!(1, grid[(0, 0)]);
        assert_eq!("1234\n5678\n", format!("{grid:?}"));
        grid[(0, 0)] = 20;
        assert_eq!(20, grid[(0, 0)]);
    }

    #[test]
    fn from() {
        let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(2, grid.num_rows);
        assert_eq!(3, grid.num_cols);
        assert_eq!(1, grid[(0, 0)]);
    }

    #[test]
    fn map() {
        let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
        let grid = grid.map(|e| e - 1);
        assert_eq!(0, grid[(0, 0)]);
    }

    #[test]
    fn neighbors() {
        let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        // top left
        assert_eq!(
            vec![(0, 1), (1, 0)],
            grid.neighbors4((0, 0)).collect::<Vec<(usize, usize)>>()
        );
        // bottom left
        assert_eq!(
            vec![(1, 0), (2, 1)],
            grid.neighbors4((2, 0)).collect::<Vec<(usize, usize)>>()
        );
        // top right
        assert_eq!(
            vec![(1, 2), (0, 1)],
            grid.neighbors4((0, 2)).collect::<Vec<(usize, usize)>>()
        );
        // bottom right
        assert_eq!(
            vec![(1, 2), (2, 1)],
            grid.neighbors4((2, 2)).collect::<Vec<(usize, usize)>>()
        );
        // top middle
        assert_eq!(
            vec![(0, 2), (1, 1), (0, 0)],
            grid.neighbors4((0, 1)).collect::<Vec<(usize, usize)>>()
        );
        // left middle
        assert_eq!(
            vec![(0, 0), (1, 1), (2, 0)],
            grid.neighbors4((1, 0)).collect::<Vec<(usize, usize)>>()
        );
        // right middle
        assert_eq!(
            vec![(0, 2), (2, 2), (1, 1)],
            grid.neighbors4((1, 2)).collect::<Vec<(usize, usize)>>()
        );
        // bottom middle
        assert_eq!(
            vec![(1, 1), (2, 2), (2, 0)],
            grid.neighbors4((2, 1)).collect::<Vec<(usize, usize)>>()
        );
        // middle
        assert_eq!(
            vec![(0, 1), (1, 2), (2, 1), (1, 0)],
            grid.neighbors4((1, 1)).collect::<Vec<(usize, usize)>>()
        );
    }

    #[test]
    fn neighbors8() {
        let grid = Grid::from(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
        assert_eq!(
            vec![(0, 1), (1, 0), (1, 1)],
            grid.neighbors8((0, 0)).collect::<Vec<(usize, usize)>>()
        );
    }

    #[test]
    fn enumerated_elems() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let elems: Vec<_> = grid.enumerated_elems().map(|(rc, &e)| (rc, e)).collect();
        assert_eq!(
            vec![((0, 0), 1), ((0, 1), 2), ((1, 0), 3), ((1, 1), 4)],
            elems
        );
    }

    #[test]
    fn row() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let row = grid.row(1);
        assert_eq!(2, row.len());
        let row: Vec<_> = row.copied().collect();
        assert_eq!(vec![3, 4], row);
    }

    #[test]
    fn row_reversed() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let row: Vec<_> = grid.row(1).rev().copied().collect();
        assert_eq!(vec![4, 3], row);
    }

    #[test]
    fn col() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let col = grid.col(1);
        assert_eq!(2, col.len());
        let col: Vec<_> = col.copied().collect();
        assert_eq!(vec![2, 4], col);
    }

    #[test]
    fn col_reversed() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let col: Vec<_> = grid.col(1).rev().copied().collect();
        assert_eq!(vec![4, 2], col);
    }

    #[test]
    fn cols() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let cols = grid.cols();
        assert_eq!(2, cols.len());
        let cols: Vec<Vec<_>> = cols.map(|col| col.copied().collect()).collect();
        assert_eq!(vec![vec![1, 3], vec![2, 4]], cols);
    }

    #[test]
    fn cols_reversed() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let cols: Vec<Vec<_>> = grid.cols().rev().map(|col| col.copied().collect()).collect();
        assert_eq!(vec![vec![2, 4], vec![1, 3]], cols);
    }

    #[test]
    fn rows() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let rows = grid.rows();
        assert_eq!(2, rows.len());
        let rows: Vec<Vec<_>> = rows.map(|row| row.copied().collect()).collect();
        assert_eq!(vec![vec![1, 2], vec![3, 4]], rows);
    }

    #[test]
    fn rows_reversed() {
        let grid = Grid::from(vec![vec![1, 2], vec![3, 4]]);
        let rows: Vec<Vec<_>> = grid.rows().rev().map(|row| row.copied().collect()).collect();
        assert_eq!(vec![vec![3, 4], vec![1, 2]], rows);
    }
}
