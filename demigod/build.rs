use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "message/cstrike15_gcmessages.proto",
            "message/cstrike15_usermessages.proto",
            "message/engine_gcmessages.proto",
            "message/netmessages.proto",
            "message/steammessages.proto",
        ],
        &["message/"]
    )?;
    Ok(())
}
