#[cfg(test)]
mod python_tests {
    use crate::python::ensure_python_ready;

    static INIT: std::sync::Once = std::sync::Once::new();

    fn init_logger() {
        INIT.call_once(|| {
            let _ = pretty_env_logger::try_init();
        });
    }

    #[tokio::test]
    async fn test_python_setup() {
        init_logger();
        // Note: This test requires PYTHON_VERSION env var to be set
        unsafe { std::env::set_var("PYTHON_VERSION", "3.12.2"); }
        ensure_python_ready().await.unwrap();
    }
}
