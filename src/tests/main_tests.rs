#[cfg(test)]
mod main_tests {
    use crate::parse_args;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn test_parse_args_accepts_valid_arguments() {
        let parsed = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "output.mp4",
            "-vmodel",
            "gfmss",
            "-type",
            "0",
            "-fps",
            "60",
            "-scale",
            "1.0",
            "-multi",
            "2",
            "-fp16",
            "false",
            "-union",
            "false",
        ]))
        .unwrap();

        assert_eq!(parsed.input_file, "input.mp4");
        assert_eq!(parsed.output_file, "output.mp4");
        assert_eq!(parsed.vfi_model, "gfmss");
        assert_eq!(parsed.processing_type, 0);
        assert_eq!(parsed.fps, 60.0);
        assert_eq!(parsed.scale, 1.0);
        assert_eq!(parsed.multi, 2);
        assert_eq!(parsed.vfi_fp16, false);
        assert_eq!(parsed.union, false);
    }

    #[test]
    fn test_parse_args_with_union_model() {
        let parsed = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "output.mp4",
            "-vmodel",
            "gfmss",
            "-type",
            "1",
            "-fps",
            "60",
            "-scale",
            "2.0",
            "-multi",
            "4",
            "-fp16",
            "true",
            "-union",
            "true",
        ]))
        .unwrap();

        assert_eq!(parsed.input_file, "input.mp4");
        assert_eq!(parsed.output_file, "output.mp4");
        assert_eq!(parsed.vfi_model, "gfmss");
        assert_eq!(parsed.processing_type, 1);
        assert_eq!(parsed.fps, 60.0);
        assert_eq!(parsed.scale, 2.0);
        assert_eq!(parsed.multi, 4);
        assert_eq!(parsed.vfi_fp16, true);
        assert_eq!(parsed.union, true);
    }

    #[test]
    fn test_parse_args_rejects_no_arguments() {
        let result = parse_args(&args(&["SmoothOctopusEngine"]));

        assert_eq!(result.unwrap_err(), "No arguments provided.");
    }

    #[test]
    fn test_parse_args_rejects_missing_value() {
        let result = parse_args(&args(&["SmoothOctopusEngine", "-fps"]));

        assert_eq!(result.unwrap_err(), "Missing value for argument: -fps");
    }

    #[test]
    fn test_parse_args_rejects_invalid_argument() {
        let result = parse_args(&args(&["SmoothOctopusEngine", "-unknown", "value"]));

        assert_eq!(result.unwrap_err(), "invalid argument: -unknown");
    }

    #[test]
    fn test_parse_args_rejects_invalid_value_type() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "output.mp4",
            "-vmodel",
            "gfmss",
            "-fps",
            "fast",
        ]));

        assert_eq!(result.unwrap_err(), "failed to parse fps argument");
    }

    #[test]
    fn test_parse_args_rejects_missing_required_input_file() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-vmodel",
            "gfmss",
            "-fps",
            "60",
            "-scale",
            "1.0",
            "-multi",
            "2",
            "-fp16",
            "false",
            "-union",
            "false",
        ]));

        assert_eq!(result.unwrap_err(), "no input file provided.");
    }

    #[test]
    fn test_parse_args_rejects_missing_required_vfitype() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "/path/to/file.mp4",
            "-fps",
            "60",
            "-scale",
            "1.0",
            "-multi",
            "2",
            "-fp16",
            "false",
            "-union",
            "false",
        ]));

        assert_eq!(result.unwrap_err(), "no VFI model type provided.");
    }

    #[test]
    fn test_parse_args_rejects_missing_required_fps() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "/path/to/file.mp4",
            "-vmodel",
            "gfmss",
            "-scale",
            "1.0",
            "-multi",
            "2",
            "-fp16",
            "false",
            "-union",
            "false",
        ]));

        assert_eq!(result.unwrap_err(), "no valid fps provided.");
    }

    #[test]
    fn test_parse_args_uses_default_values() {
        let parsed = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "output.mp4",
            "-vmodel",
            "gfmss",
            "-fps",
            "60",
        ]))
        .unwrap();

        assert_eq!(parsed.scale, 1.0);
        assert_eq!(parsed.multi, 2);
        assert_eq!(parsed.vfi_fp16, false); 
        assert_eq!(parsed.union, false);
    }

    #[test]
    fn test_parse_args_with_float_scale() {
        let parsed = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "output.mp4",
            "-vmodel",
            "gfmss",
            "-fps",
            "60",
            "-scale",
            "1.5",
            "-multi",
            "2",
            "-fp16",
            "false",
            "-union",
            "false",
        ]))
        .unwrap();

        assert_eq!(parsed.scale, 1.5);
    }

    #[test]
    fn test_parse_args_rejects_invalid_scale() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "output.mp4",
            "-vmodel",
            "gfmss",
            "-fps",
            "60",
            "-scale",
            "invalid",
            "-multi",
            "2",
            "-fp16",
            "false",
            "-union",
            "false",
        ]));

        assert_eq!(result.unwrap_err(), "failed to parse scale argument");
    }

    #[test]
    fn test_parse_args_accepts_output_file() {
        let parsed = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-o",
            "output.mp4",
            "-vmodel",
            "gfmss",
            "-fps",
            "60",
        ]))
        .unwrap();

        assert_eq!(parsed.output_file, "output.mp4");
    }

    #[test]
    fn test_parse_args_rejects_missing_output_file() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-vmodel",
            "gfmss",
            "-fps",
            "60",
            "-scale",
            "1.0",
            "-multi",
            "2",
            "-fp16",
            "false",
            "-union",
            "false",
        ]));

        assert_eq!(result.unwrap_err(), "no output folder provided.");
    }

    #[test]
    fn test_parse_args_requires_all_three_core_args() {
        // Missing -i
        let r1 = parse_args(&args(&[
            "SmoothOctopusEngine", "-o", "out.mp4", "-vmodel", "gfmss", "-fps", "60",
        ]));
        assert_eq!(r1.unwrap_err(), "no input file provided.");

        // Missing -o
        let r2 = parse_args(&args(&[
            "SmoothOctopusEngine", "-i", "in.mp4", "-vmodel", "gfmss", "-fps", "60",
        ]));
        assert_eq!(r2.unwrap_err(), "no output folder provided.");

        // Missing "-vmodel"
        let r3 = parse_args(&args(&[
            "SmoothOctopusEngine", "-i", "in.mp4", "-o", "out.mp4", "-fps", "60",
        ]));
        assert_eq!(r3.unwrap_err(), "no VFI model type provided.");

        // Missing -fps
        let r4 = parse_args(&args(&[
            "SmoothOctopusEngine", "-i", "in.mp4", "-o", "out.mp4", "-vmodel", "gfmss",
        ]));
        assert_eq!(r4.unwrap_err(), "no valid fps provided.");
    }

    #[test]
    fn test_parse_args_rejects_negative_fps() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i", "input.mp4",
            "-o", "output.mp4",
            "-vmodel", "gfmss",
            "-fps", "-30",
        ]));
        assert_eq!(result.unwrap_err(), "no valid fps provided.");
    }

    #[test]
    fn test_parse_args_rejects_zero_fps() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i", "input.mp4",
            "-o", "output.mp4",
            "-vmodel", "gfmss",
            "-fps", "0",
        ]));
        assert_eq!(result.unwrap_err(), "no valid fps provided.");
    }

    #[test]
    fn test_parse_args_with_negative_scale() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i", "input.mp4",
            "-o", "output.mp4",
            "-vmodel", "gfmss",
            "-fps", "60",
            "-scale", "-1",
        ]));
        let parsed = result.unwrap();
        assert_eq!(parsed.scale, -1.0);
    }
}
