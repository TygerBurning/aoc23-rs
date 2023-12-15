pub struct Grid<T> {
    contents: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    // Top-left is the origin.
    // x co-ordinate across (i.e. column number)
    // y co-ordinate down (i.e. row number)
    pub fn index(&self, x: usize, y: usize) -> &T {
        &self.contents[y][x]
    }
}

pub fn create_grid(input: &str) -> Grid<char> {
    let mut contents = vec![];
    for line in input.lines() {
        contents.push(line.chars().collect::<Vec<char>>())
    }
    Grid { contents }
}
