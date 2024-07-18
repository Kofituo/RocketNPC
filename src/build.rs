use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("src").join("hello_world.capnp");
    let oc_path = PathBuf::from("src").join("ocs365.capnp");
    capnpc::CompilerCommand::new()
        .file(path)
        .file(oc_path)
        .run()?;
    Ok(())
}
