fn main() {
    // SAFETY: build script initialization before any Slint compiler threads are spawned.
    unsafe {
        std::env::set_var("SLINT_ENABLE_EXPERIMENTAL_FEATURES", "1");
    }
    let config = slint_build::CompilerConfiguration::new().with_library_paths(
        std::collections::HashMap::from([(
            "material".to_string(),
            std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap())
                .join("material-1.0/material.slint"),
        )]),
    );
    slint_build::compile_with_config("ui/main.slint", config)
        .expect("failed to compile slint file");
}
