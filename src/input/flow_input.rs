use crate::input::code_gen_input::CodeGenInput;
use std::io::Error;
use std::borrow::Borrow;

/// Locate a typescript interface to generate code.
struct TypescriptCodeGenInput {
    /// The typescript file path relative to the program entry point.
    relative_path: String,

    /// The name of typescript interface which exported in `relative_path`.
    interface_name: String,
}

impl CodeGenInput for TypescriptCodeGenInput {
    fn absolute_path() -> Result<&str, Error> {
        let current_path = std::env::current_dir();

        current_path.map(|path_buf| {
            path_buf.to_str().unwrap_or("")
        })
    }
}

impl std::fmt::Display for TypescriptCodeGenInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let current_path = std::env::current_dir()
            .expect("Cannot get working directory.");

        let current_path = current_path
            .to_str()
            .unwrap_or("");
        let out_str = format!(
            r#"CodeGenInput(
                from: {},
                relative path: {}
                interface name: {}
            )
            "#,
            current_path,
            self.relative_path,
            self.interface_name
        );

        write!(f, "{}", out_str)
    }
}