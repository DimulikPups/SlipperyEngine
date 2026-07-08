use super::{Stage, Context};

#[derive(Debug)]
pub struct Interpolate;

impl Interpolate {
    pub fn new() -> Self {
        Self
    }
}

impl Stage for Interpolate {
    fn execute(&self, context: &mut Context) {
        context.new_fps = context.original_fps * context.multiply as f32;
        // TODO: Implement actual frame interpolation using:
        // - context.scale for scale factor
        // - context.union for union model variant (GFMSS)
        // - context.fp16 for precision mode
    }
}

impl Default for Interpolate {
    fn default() -> Self {
        Self::new()
    }
}
