fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/auth.proto")?;
    tonic_build::compile_protos("proto/admin.proto")?;
    Ok(())
}
