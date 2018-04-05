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
}
