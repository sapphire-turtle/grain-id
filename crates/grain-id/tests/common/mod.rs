use grain_id::*;

#[cfg(feature = "std")]
fn assert_string_convertion(value: GrainId) {
    assert_eq!(value, value.to_string().parse::<GrainId>().unwrap());
}

fn assert_integer_conversion(value: GrainId) {
    assert_eq!(value, <GrainId>::try_from(<u64>::from(value)).unwrap());
}

fn assert_le_bytes_conversion(value: GrainId) {
    let bytes = value.to_le_bytes();
    assert_eq!(value, GrainId::from_le_bytes(bytes).unwrap());
    assert_eq!(value, GrainId::from_le_bytes_lossy(bytes));
}
fn assert_be_bytes_conversion(value: GrainId) {
    let bytes = value.to_be_bytes();
    assert_eq!(value, GrainId::from_be_bytes(bytes).unwrap());
    assert_eq!(value, GrainId::from_be_bytes_lossy(bytes));
}
fn assert_be_bytes_compact_conversion(value: GrainId) {
    let bytes = value.to_be_bytes_compact();
    assert_eq!(value, GrainId::from_be_bytes_compact(bytes).unwrap());
    assert_eq!(value, GrainId::from_be_bytes_compact_lossy(bytes));
}
fn assert_le_bytes_compact_conversion(value: GrainId) {
    let bytes = value.to_le_bytes_compact();
    assert_eq!(value, GrainId::from_le_bytes_compact(bytes).unwrap());
    assert_eq!(value, GrainId::from_le_bytes_compact_lossy(bytes));
}

pub fn assert_conversion(value: GrainId) {
    #[cfg(feature = "std")]
    assert_string_convertion(value);
    assert_integer_conversion(value);
    assert_be_bytes_conversion(value);
    assert_be_bytes_compact_conversion(value);
    assert_le_bytes_conversion(value);
    assert_le_bytes_compact_conversion(value);
}
