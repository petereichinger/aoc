pub struct Monkey {
    pub id: usize,
    pub items: Vec<i128>,
    pub operation: Box<dyn Fn(i128) -> i128>,
    pub test: i128,
    pub targets: (usize, usize),
    pub inspect_count: usize,
}
