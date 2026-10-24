fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .extern_path(".grain_id", "::grain_id::proto")
        .compile_protos(
            &["proto/grain_id_example.proto"],
            &["proto", "../../crates/grain-id/grain-id-proto/"],
        )?;

    Ok(())
}
