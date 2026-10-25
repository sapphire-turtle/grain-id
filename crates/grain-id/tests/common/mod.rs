use grain_id::*;

pub fn assert_conversion(value: GrainId) {
    assert_eq!(value, value.to_string().parse::<GrainId>().unwrap());
    assert_eq!(
        value,
        value.to_string().to_uppercase().parse::<GrainId>().unwrap()
    );
    assert_eq!(value, GrainId::try_from(u64::from(value)).unwrap());
    assert_eq!(value, GrainId::from_byte_prefix(&value.to_byte_prefix()));
    assert_eq!(value, GrainId::from_byte_suffix(&value.to_byte_suffix()));
}
