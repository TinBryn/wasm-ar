use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref APP_STATE: Mutex<Arc<AppState>> = Mutex::new(Arc::new(AppState::new()));
}

pub fn update_dynamic_data(time: f32, canvas: Canvas) {
    let min_height_width = canvas.height.min(canvas.width);
    let display_size = 0.9 * min_height_width;

    let half_display_size = display_size * 0.5;
    let half_canvas_width = canvas.width * 0.5;
    let half_canvas_height = canvas.height * 0.5;

    let mut data = APP_STATE.lock().unwrap();

    *data = Arc::new(AppState {
        canvas,
        time,

        control: Control {
            bottom: half_canvas_height - half_display_size,
            top: half_canvas_height + half_display_size,
            left: half_canvas_width - half_display_size,
            right: half_canvas_width + half_display_size,
        },
    });
}

pub fn get_curr_state() -> Arc<AppState> {
    APP_STATE.lock().unwrap().clone()
}

pub struct Canvas {
    pub width: f32,
    pub height: f32,
}

pub struct Control {
    pub bottom: f32,
    pub top: f32,
    pub left: f32,
    pub right: f32,
}

pub struct AppState {
    pub canvas: Canvas,
    pub control: Control,
    pub time: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            canvas: Canvas {
                width: 0.0,
                height: 0.0,
            },
            control: Control {
                bottom: 0.0,
                top: 0.0,
                left: 0.0,
                right: 0.0,
            },
            time: 0.0,
        }
    }
}
