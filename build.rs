use std::fs;

use anyhow::Result;
fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder
        .build_client(true)
        .build_server(true)
        .out_dir("src/pb")
        .compile_protos(&["protos/raft.proto"], &["protos"])?;
    Ok(())
}
