extern crate shaderc;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // println!("cargo:rerun-if-changed=shaders/*");

    std::fs::create_dir_all("./assets/gen/shaders")?;

    for entry in std::fs::read_dir("./shaders")? {
        let entry = entry?;
        let mut compiler = shaderc::Compiler::new().unwrap();
        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            // Support only vertex and fragment shaders currently
            // Support will be added for
            let shader_type =
                in_path
                    .extension()
                    .and_then(|ext| match ext.to_string_lossy().as_ref() {
                        "vert" => Some(shaderc::ShaderKind::Vertex),
                        "frag" => Some(shaderc::ShaderKind::Fragment),
                        _ => None,
                    });

            if let Some(shader_type) = shader_type {
                let source = std::fs::read_to_string(&in_path)?;
                let compiled_file = compiler
                    .compile_into_spirv(
                        &source,
                        shader_type,
                        in_path.file_name().unwrap().to_str().unwrap(),
                        "main",
                        None,
                    )
                    .unwrap();
                // Read the binary data from the compiled file
                let compiled_bytes = compiled_file.as_binary_u8().to_vec();

                let out_path = format!(
                    "assets/gen/shaders/{}.spv",
                    in_path.file_name().unwrap().to_string_lossy()
                );

                std::fs::write(&out_path, &compiled_bytes)?;
            }
        }
    }
    Ok(())
}
