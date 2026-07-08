#[derive(Debug)]
pub struct Context {
    pub current_path: String,
    pub output_path: String,
    pub original_fps: f32,
    pub new_fps: f32,
    pub multiply: i8,
    pub scale: f32,
    pub vfi_model: String,
    pub union: bool,
    pub fp16: bool,
    pub encoding: bool,
    pub codec: String,
    pub codec_args: Vec<String>,
}

impl Context {
    pub fn new(current_path: impl Into<String>) -> Self {
        Self {
            current_path: current_path.into(),
            output_path: "".to_string(),
            original_fps: 0.0,
            new_fps: 0.0,
            multiply: 1,
            scale: 1.0,
            vfi_model: "".to_string(),
            union: false,
            fp16: false,
            encoding: false,
            codec: "libx264".to_string(),
            codec_args: Vec::new(),
        }
    }

    pub fn with_output_path(mut self, output_p: String) -> Self {
        self.output_path = output_p;
        self
    }

    pub fn with_original_fps(mut self, fps: f32) -> Self {
        self.original_fps = fps;
        self
    }

    pub fn with_vfi_model(mut self, vfi_model: impl Into<String>) -> Self {
        self.vfi_model = vfi_model.into();
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

    pub fn with_encoding(mut self, encoding: bool) -> Self {
        self.encoding = encoding;
        self
    }

    pub fn with_codec(mut self, codec: impl Into<String>) -> Self {
        self.codec = codec.into();
        self
    }

    pub fn with_codec_args(mut self, args: Vec<String>) -> Self {
        self.codec_args = args;
        self
    }
}
