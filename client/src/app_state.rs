#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Canvas {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Angles {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Default, Clone)]
pub struct AppState {
    pub canvas: Canvas,
    pub angles: Angles,
    pub time: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn updated(&self, time: f32, canvas: Canvas) -> AppState {
        let mut result = self.clone();
        result.update(time, canvas);
        result
    }

    pub fn update(&mut self, time: f32, canvas: Canvas) {
        self.resize(canvas);
        self.time_step(time);
    }

    pub fn resized(&self, canvas: Canvas) -> AppState {
        let mut result = self.clone();
        result.resize(canvas);
        result
    }

    #[allow(unused)]
    pub fn time_stepped(&self, time: f32) -> AppState {
        let mut result = self.clone();
        result.time_step(time);
        result
    }

    pub fn resize(&mut self, canvas: Canvas) {
        self.canvas = canvas;
    }

    pub fn time_step(&mut self, time: f32) {
        let time_step = time - self.time;
        self.time = time;

        self.angles.x += 0.00037 * time_step;
        self.angles.y += 0.00059 * time_step;
        self.angles.y += 0.00083 * time_step;
    }
}
