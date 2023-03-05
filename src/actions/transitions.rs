pub struct Transitions {
    pub backlog: usize,
    pub readytodo: usize,
    pub progress: usize,
    pub done: usize,
    pub review: usize,
    pub homol: usize,
}

impl Transitions {
    pub fn new() -> Self {
        Self {
            backlog: 11,
            readytodo: 21,
            progress: 31,
            done: 41,
            review: 51,
            homol: 81,
        }
    }
}
