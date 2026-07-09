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
    pub(crate) stages: Vec<Box<dyn PipelineStageTrait>>,
}

impl Pipeline {
    pub fn new(stages: Vec<Box<dyn PipelineStageTrait>>) -> Self {
        Self { stages }
    }

    pub fn execute(&self, context: &mut Context) {
        for stage in &self.stages {
            info!("executing stage: {:?}", stage);
            stage.execute(context);
        }
    }

    pub fn len(&self) -> usize {
        self.stages.len()
    }
}
