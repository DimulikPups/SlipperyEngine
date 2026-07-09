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

    // === Tests for fixed logical issues ===

    #[test]
    fn test_stage_order_interpolation_before_encoding() {
        // Verifies fix: interpolation must run BEFORE encoding
        let (pipeline, _context) = Builder::new()
            .with_interpolation()
            .with_encoding(true)
            .build();

        assert_eq!(pipeline.len(), 2, "Pipeline should have 2 stages");

        let stage_names: Vec<String> = pipeline.stages.iter()
            .map(|s| format!("{:?}", s))
            .collect();

        let interp_index = stage_names.iter().position(|n| n.contains("Interpolate"));
        let encode_index = stage_names.iter().position(|n| n.contains("Encode"));

        assert!(interp_index.is_some(), "Interpolate stage should exist");
        assert!(encode_index.is_some(), "Encode stage should exist");
        assert!(
            interp_index < encode_index,
            "Interpolate stage must come BEFORE Encode stage, but order was: {:?}",
            stage_names
        );
    }

    #[test]
    fn test_context_new_accepts_path_and_vfi_model() {
        // Verifies: Context::new takes (current_path, vfi_model)
        let context = Context::new("/some/path")
            .with_vfi_model("rife");
        assert_eq!(context.current_path, "/some/path");
        assert_eq!(context.vfi_model, "rife");
    }

    #[test]
    fn test_context_with_vfi_model() {
        let context = Context::new("/path")
            .with_vfi_model("rife");
        assert_eq!(context.vfi_model, "rife");
    }

    #[test]
    fn test_context_encoding_flag() {
        let context = Context::new("/path")
            .with_encoding(true);
        assert_eq!(context.encoding, true);

        let context_false = Context::new("/path")
            .with_encoding(false);
        assert_eq!(context_false.encoding, false);
    }

    #[test]
    fn test_context_output_path_default_empty() {
        let context = Context::new("/input/path");
        assert_eq!(context.output_path, "", "output_path should default to empty string");
    }

    #[test]
    fn test_context_with_output_path() {
        let context = Context::new("/input/path")
            .with_output_path("/output/video.mp4".to_string());
        assert_eq!(context.output_path, "/output/video.mp4");
    }

    #[test]
    fn test_builder_with_vfi_model() {
        let (_pipeline, context) = Builder::new()
            .with_vfi_model("rife-ncnn")
            .build();
        assert_eq!(context.vfi_model, "rife-ncnn");
    }

    #[test]
    fn test_builder_with_encoding_flag() {
        let (pipeline, context) = Builder::new()
            .with_encoding(true)
            .build();
        assert_eq!(pipeline.len(), 1);
        assert_eq!(context.encoding, true);
    }

    #[test]
    fn test_builder_combined_interpolation_and_encoding() {
        let (pipeline, context) = Builder::new()
            .with_interpolation()
            .with_encoding(true)
            .with_vfi_model("gfmss")
            .with_scale(2.0)
            .with_union(true)
            .with_fp16(true)
            .build();

        assert_eq!(pipeline.len(), 2, "Should have both interpolation and encoding stages");
        assert_eq!(context.vfi_model, "gfmss");
        assert_eq!(context.scale, 2.0);
        assert_eq!(context.union, true);
        assert_eq!(context.fp16, true);
        assert_eq!(context.encoding, true);
    }

    #[test]
    fn test_multiply_parameter_affects_new_fps() {
        // Verify multiply parameter correctly affects new_fps
        let test_cases = [
            (30.0, 2, 60.0),
            (24.0, 4, 96.0),
            (60.0, 8, 480.0),
        ];

        for (original_fps, multiply, expected) in test_cases {
            let (pipeline, context) = Builder::new()
                .with_interpolation()
                .build();
            let context = context
                .with_original_fps(original_fps)
                .with_multiply(multiply);

            let result = engine::execute(&pipeline, context);
            assert_eq!(
                result.new_fps, expected,
                "original_fps={}, multiply={} should give new_fps={}, got {}",
                original_fps, multiply, expected, result.new_fps
            );
        }
    }
}
