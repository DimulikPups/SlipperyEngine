use crate::pipeline::{Pipeline, Context};

pub fn execute(pipeline: &Pipeline, mut context: Context) -> Context {
    pipeline.execute(&mut context);
    context
}
