pub struct Camera {
    pub x: i64,
    pub y: i64,

    pub width: i64,
    pub height: i64,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            x: 0,
            y: 0,

            width: 7,
            height: 7,
        }
    }

    pub fn update(&mut self, x: i64, y: i64) {
        self.x = x - self.width/2; //Interger divison rounds down
        self.y = y - self.width/2;
    }
}
