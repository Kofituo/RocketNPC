use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("src").join("hello_world.capnp");
    capnpc::CompilerCommand::new()
        .file(path)
        .run()?;
    Ok(())
}
