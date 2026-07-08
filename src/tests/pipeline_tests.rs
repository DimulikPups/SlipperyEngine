#[cfg(test)]
mod pipeline_tests {
    use crate::engine;
    use crate::pipeline::{Builder, Context};

    #[test]
    fn test_builder_creates_interpolation_pipeline() {
        let (pipeline, _context) = Builder::new().with_interpolation().build();
        assert_eq!(pipeline.len(), 1);
    }

    #[test]
    fn test_engine_executes_pipeline() {
        let (pipeline, context) = Builder::new().with_interpolation().build();
        let context = context
            .with_original_fps(30.0)
            .with_multiply(8);

        let expected_fps = context.original_fps * context.multiply as f32;
        let result = engine::execute(&pipeline, context);

        assert_eq!(result.current_path, "");
        assert_eq!(result.new_fps, expected_fps);
    }

    #[test]
    fn test_builder_passes_interpolation_params() {
        let (pipeline, context) = Builder::new()
            .with_interpolation()
            .with_scale(2.0)
            .with_union(true)
            .with_fp16(true)
            .build();

        assert_eq!(pipeline.len(), 1);
        assert_eq!(context.scale, 2.0);
        assert_eq!(context.union, true);
        assert_eq!(context.fp16, true);
    }
}
