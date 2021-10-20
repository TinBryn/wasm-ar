#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Canvas {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Control {
    pub bottom: f32,
    pub top: f32,
    pub left: f32,
    pub right: f32,
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
    pub control: Control,
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
        let min_height_width = canvas.height.min(canvas.width);
        let display_size = 0.9 * min_height_width;

        let half_display_size = display_size * 0.5;
        let half_canvas_width = canvas.width * 0.5;
        let half_canvas_height = canvas.height * 0.5;

        self.canvas = canvas;
        let time_step = time - self.time;
        self.time = time;

        self.control = Control {
            bottom: half_canvas_height - half_display_size,
            top: half_canvas_height + half_display_size,
            left: half_canvas_width - half_display_size,
            right: half_canvas_width + half_display_size,
        };

        self.angles.x += 0.00037 * time_step;
        self.angles.y += 0.00059 * time_step;
        self.angles.y += 0.00083 * time_step;
    }
}
