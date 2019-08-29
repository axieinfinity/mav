#[macro_export]
macro_rules! file_stem {
    () => {
        std::path::Path::new(file!())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    };
}
