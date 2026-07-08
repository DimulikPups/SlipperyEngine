//! Video processing pipeline system.
//!
//! The pipeline is a series of stages that process video frames sequentially.
//! Each stage implements the `Stage` trait and receives a `Context` that
//! tracks the current state of the processing.

pub mod stage;
pub mod context;
pub mod builder;

mod interpolation;
pub mod encoding;

pub use stage::Stage;
pub use context::Context;
pub use builder::Builder;

use log::info;
use stage::Stage as PipelineStageTrait;

pub struct Pipeline {
    stages: Vec<Box<dyn PipelineStageTrait>>,
}

impl Pipeline {
    pub fn new(stages: Vec<Box<dyn PipelineStageTrait>>) -> Self {
        Self { stages }
    }

    pub fn execute(&self, context: &mut Context) {
        for stage in &self.stages {
            info!("Executing stage: {:?}", stage);
            stage.execute(context);
        }
    }

    pub fn len(&self) -> usize {
        self.stages.len()
    }
}
