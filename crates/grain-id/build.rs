fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "prost")]
    prost_build::Config::new().compile_protos(
        &[
            "grain-id-proto/grain_id/grain_id.proto",
            "grain-id-proto/grain_id/grain_id_prefix.proto",
        ],
        &["grain-id-proto/"],
    )?;
    Ok(())
}
