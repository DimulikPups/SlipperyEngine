#[cfg(test)]
mod fs_tests {
    use crate::fs;

    #[test]
    fn test_create_directory() {
        let path = "src/tests/filesystem/path";
        let result = fs::create_dir(path, "test_folder");
        assert!(result.is_ok());
    }
}
