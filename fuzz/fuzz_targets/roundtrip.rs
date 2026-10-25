//! Checks that every conversion of an arbitrary `GrainId` round-trips.
#![no_main]

use grain_id::{GrainId, GrainIdPrefix};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (GrainId, u64, [u8; 5], u8, GrainIdPrefix)| {
    let (id, int, bytes, prefix_len, prefix) = input;

    // Integer
    assert_eq!(GrainId::try_from(u64::from(id)), Ok(id));
    assert_eq!(GrainId::from_u64_lossy(u64::from(id)), id);
    match GrainId::from_u64(int) {
        Ok(x) => assert_eq!(u64::from(x), int),
        Err(_) => assert!(int > u64::from(GrainId::MAX)),
    }
    assert_eq!(
        u64::from(GrainId::from_u64_lossy(int)),
        int % (u64::from(GrainId::MAX) + 1)
    );

    // String
    let s = id.to_string();
    assert_eq!(s.parse::<GrainId>(), Ok(id));
    assert_eq!(s.to_uppercase().parse::<GrainId>(), Ok(id));

    // Bytes
    assert_eq!(GrainId::from_byte_prefix(&id.to_byte_prefix()), id);
    assert_eq!(GrainId::from_byte_suffix(&id.to_byte_suffix()), id);
    assert_eq!(
        GrainId::from_byte_prefix(&bytes).to_byte_prefix()[..4],
        bytes[..4]
    );
    assert_eq!(
        GrainId::from_byte_suffix(&bytes).to_byte_suffix()[1..],
        bytes[1..]
    );

    // Ordering matches the integer and the string representation
    let other = GrainId::from_u64_lossy(int);
    assert_eq!(id.cmp(&other), u64::from(id).cmp(&u64::from(other)));
    assert_eq!(id.cmp(&other), s.cmp(&other.to_string()));

    // Arithmetic wraps within the 35-bit range
    assert_eq!(id.increment().wrapping_sub(id), GrainId::from_u64_lossy(1));
    assert_eq!(id.wrapping_sub(id), GrainId::NIL);

    // Every prefix of the string contains the id
    let len = prefix_len % 8;
    let of = GrainIdPrefix::of(id, len).unwrap();
    assert_eq!(s[..usize::from(len)].parse::<GrainIdPrefix>(), Ok(of));
    assert_eq!(of.to_string(), s[..usize::from(len)]);
    assert!(of.contains(id));
    assert!(of.range().contains(&id));

    // `contains` agrees with the numeric range and the string representation
    let in_range = prefix.range().contains(&id);
    assert_eq!(prefix.contains(id), in_range);
    assert_eq!(s.starts_with(&prefix.to_string()), in_range);
    assert_eq!(prefix.to_string().parse::<GrainIdPrefix>(), Ok(prefix));
});
