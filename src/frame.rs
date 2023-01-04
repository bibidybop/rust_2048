pub struct Frame {
    pub tiles: [u32; 16],
    pub score: u32,
    pub won: bool,
    pub over: bool,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            tiles: [0; 16],
            score: 0,
            won: false,
            over: false,
        }
    }
}
