use super::point::Point;

pub struct Grid<T> {
    cells: Vec<Vec<T>>,
    num_rows: i32,
    num_cols: i32,
}

impl <T: std::cmp::PartialEq>Grid<T> {
    pub fn new_from_cells(cells: Vec<Vec<T>>) -> Grid<T> {
        let (num_rows, num_cols) = (cells.len() as i32, cells[0].len() as i32);
        Grid{cells, num_rows, num_cols}
    }

    pub fn get(&self, p: &Point)        -> &T { &self.cells[p.i as usize][p.j as usize] }
    pub fn set(&mut self, p: &Point, val: T)  { self.cells[p.i as usize][p.j as usize] = val; }
    pub fn contains(&self, p: &Point) -> bool { 0 <= p.i && p.i < self.num_rows && 0 <= p.j && p.j < self.num_cols }

    pub fn find(&self, target_val: &T) -> Vec<Point> {
        self.cells.iter().enumerate()
            .flat_map(|(i, row)| row.iter().enumerate()
                .filter(|(_, val)| *val == target_val)
                .map(move |(j, _)| Point{i: i as i32, j: j as i32})
            )
            .collect()
    }

    pub fn find_single(&self, target_val: &T) -> Point {
        let indices = self.find(target_val);
        if indices.len() != 1 {
            panic!("Expected to see find exactly one cell with target value. Found {:?}", {indices})
        }
        return indices[0];
    }

    #[allow(dead_code)] // Exists for tests
    pub fn get_cells(&self) -> &Vec<Vec<T>> { &self.cells }

}