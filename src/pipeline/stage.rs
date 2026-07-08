use std::fmt::Debug;
use super::Context;

pub trait Stage: Debug {
    fn execute(&self, context: &mut Context);
}
