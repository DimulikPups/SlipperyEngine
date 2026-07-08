use crate::pipeline::encoding::Encode;
use super::{Pipeline, Stage, Context};
use super::interpolation::Interpolate;

pub struct Builder {
    is_encoding: bool,
    is_interpolating: bool,
    context_builder: Context,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            is_encoding: false,
            is_interpolating: false,
            context_builder: Context::new("", ""),
        }
    }

    pub fn with_interpolation(mut self) -> Self {
        self.is_interpolating = true;
        self
    }

    pub fn with_vfi_model(mut self, vfi_model: impl Into<String>) -> Self {
        self.context_builder = self.context_builder.with_vfi_model(vfi_model);
        self
    }

    pub fn with_scale(mut self, scale: f32) -> Self {
        self.context_builder = self.context_builder.with_scale(scale);
        self
    }

    pub fn with_union(mut self, union: bool) -> Self {
        self.context_builder = self.context_builder.with_union(union);
        self
    }

    pub fn with_fp16(mut self, fp16: bool) -> Self {
        self.context_builder = self.context_builder.with_fp16(fp16);
        self
    }

    pub fn with_encoding(mut self, encode: bool) -> Self {
        self.context_builder = self.context_builder.with_encoding(encode);
        self
    }

    pub fn build(self) -> (Pipeline, Context) {
        let mut stages: Vec<Box<dyn Stage>> = Vec::new();

        if self.is_encoding {
            stages.push(Box::new(Encode::new()))
        }

        if self.is_interpolating { 
            stages.push(Box::new(Interpolate::new())); 
        }

        let context = self.context_builder;
        (Pipeline::new(stages), context)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
