extern crate glsl_to_spirv;

use std::error::Error;

const SRC_LOC: &'static str = "src/shaders";
const DEST_LOC: &'static str = "assets/shaders";

fn main() -> Result<(), Box<Error>> {
    use glsl_to_spirv::ShaderType;

    // Tell the build script to only run again if we change our source shaders
    println!("cargo:rerun-if-changed={}", SRC_LOC);

    // Create destination directory
    std::fs::create_dir_all(DEST_LOC)?;

    // Iterate through shaders and compile them
    for entry in std::fs::read_dir(SRC_LOC)? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();
            if in_path.to_str().unwrap().contains("glium") {
                continue;
            }

            // Pick between vertex and fragment shader
            let shader_type = in_path.extension().and_then(|ext| {
                match ext.to_string_lossy().as_ref() {
                    "vert" => Some(ShaderType::Vertex),
                    "frag" => Some(ShaderType::Fragment),
                    _ => None,
                }
            });

            if let Some(shader_type) = shader_type {
                use std::io::Read;

                let source = std::fs::read_to_string(&in_path)?;
                let mut compiled_file = glsl_to_spirv::compile(&source, shader_type)?;

                let mut compiled_bytes = Vec::new();
                compiled_file.read_to_end(&mut compiled_bytes)?;

                let out_path = format!(
                    "{}/{}.spv",
                    DEST_LOC,
                    in_path.file_name().unwrap().to_string_lossy()
                );

                std::fs::write(&out_path, &compiled_bytes)?;
            }
        }
    }

    Ok(())
}
