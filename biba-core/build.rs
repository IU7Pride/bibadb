use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut proto = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    proto.pop();
    proto.push("api");
    proto.push("biba.proto");
    tonic_build::compile_protos(proto)?;
    Ok(())
}
