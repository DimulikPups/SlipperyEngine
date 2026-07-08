#[derive(Debug)]
pub struct Context {
    pub current_path: String,
    pub original_fps: f32,
    pub new_fps: f32,
    pub multiply: i8,
    pub scale: f32,         // Scale factor for interpolation (e.g., 1.0)
    pub union: bool,        // Use union model variant
    pub fp16: bool,         // Use float16 precision
}

impl Context {
    pub fn new(current_path: impl Into<String>) -> Self {
        Self {
            current_path: current_path.into(),
            original_fps: 0.0,
            new_fps: 0.0,
            multiply: 1,
            scale: 1.0,
            union: false,
            fp16: false,
        }
    }

    pub fn with_original_fps(mut self, fps: f32) -> Self {
        self.original_fps = fps;
        self
    }

    pub fn with_new_fps(mut self, fps: f32) -> Self {
        self.new_fps = fps;
        self
    }

    pub fn with_multiply(mut self, factor: i8) -> Self {
        self.multiply = factor;
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_union(mut self, union: bool) -> Self {
        self.union = union;
        self
    }

    pub fn with_fp16(mut self, fp16: bool) -> Self {
        self.fp16 = fp16;
        self
    }
}
