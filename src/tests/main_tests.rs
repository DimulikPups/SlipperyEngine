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
            "-vfitype",
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
        assert_eq!(parsed.vfitype, "gfmss");
        assert_eq!(parsed.typeofexecution, 0);
        assert_eq!(parsed.fps, 60.0);
        assert_eq!(parsed.scale, 1.0);
        assert_eq!(parsed.multi, 2);
        assert_eq!(parsed.fp16, false);
        assert_eq!(parsed.union, false);
    }

    #[test]
    fn test_parse_args_with_union_model() {
        let parsed = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-vfitype",
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
        assert_eq!(parsed.vfitype, "gfmss");
        assert_eq!(parsed.typeofexecution, 1);
        assert_eq!(parsed.fps, 60.0);
        assert_eq!(parsed.scale, 2.0);
        assert_eq!(parsed.multi, 4);
        assert_eq!(parsed.fp16, true);
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

        assert_eq!(result.unwrap_err(), "Invalid argument: -unknown");
    }

    #[test]
    fn test_parse_args_rejects_invalid_value_type() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-vfitype",
            "gfmss",
            "-fps",
            "fast",
        ]));

        assert_eq!(result.unwrap_err(), "Failed to parse fps argument");
    }

    #[test]
    fn test_parse_args_rejects_missing_required_input_file() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-vfitype",
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

        assert_eq!(result.unwrap_err(), "No input file provided.");
    }

    #[test]
    fn test_parse_args_rejects_missing_required_vfitype() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
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

        assert_eq!(result.unwrap_err(), "No VFI model type provided.");
    }

    #[test]
    fn test_parse_args_rejects_missing_required_fps() {
        let result = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-vfitype",
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

        assert_eq!(result.unwrap_err(), "No valid fps provided.");
    }

    #[test]
    fn test_parse_args_uses_default_values() {
        // Test that missing optional args use defaults
        let parsed = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-vfitype",
            "gfmss",
            "-fps",
            "60",
        ]))
        .unwrap();

        assert_eq!(parsed.scale, 1.0);  // default
        assert_eq!(parsed.multi, 2);    // default
        assert_eq!(parsed.fp16, false); // default
        assert_eq!(parsed.union, false); // default
    }

    #[test]
    fn test_parse_args_with_float_scale() {
        let parsed = parse_args(&args(&[
            "SmoothOctopusEngine",
            "-i",
            "input.mp4",
            "-vfitype",
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
            "-vfitype",
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

        assert_eq!(result.unwrap_err(), "Failed to parse scale argument");
    }
}
