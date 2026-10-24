macro_rules! doc_to_bytes {
    ($endian:literal) => {
        concat!("Returns the memory representation of the internal integer of `GrainId` as a byte array in ", $endian, " byte order.")
    };
    ($endian:literal, compact) => {
        concat!("Returns the memory representation of the internal integer of `GrainId` as a compact byte array in ", $endian, " byte order.")
    }
}

macro_rules! doc_from_bytes {
    ($endian:literal, $lossy:path) => {
        concat!(
            "Attempt to creata new `GrainId` from its representaion as a byte array in ",
            $endian,
            ".\n\n",
            "# Error\n",
            "Returns error if the value is larger than [`GrainId::MAX`].\n",
            "See [`from_u64`](GrainId::from_u64) for more details.\n",
            "If you don't need to detect out-of-range values, use [`",
            stringify!($lossy),
            "`]."
        )
    };
    ($endian:literal, $lossy:literal, compact) => {
        concat!(
            "Attempt to create new `GrainId` from its compact representaion as a byte array in ",
            $endian,
            ".\n",
            "\n",
            "# Error\n",
            "Returns error if the value is larger than [`GrainId::MAX`].\n",
            "See [`from_u64`](GrainId::from_u64) for more details.",
            "If you don't need to detect out-of-range values, use ",
            $lossy,
            "."
        )
    };
}

macro_rules! doc_from_bytes_lossy {
    ($endian:literal) => {
        concat!(
            "Create new `GrainId` from its representaion as a byte array in ",
            $endian,
            "with truncation of the upper 29 bit.\n",
            "\n",
            "See [`from_u64_lossy`](GrainId::from_u64_lossy) for more details."
        )
    };
    ($endian:literal, compact) => {
        concat!(
            "Create new `GrainId` from its compact representaion as a byte array in ",
            $endian,
            "with truncation of the upper 5 bits.\n",
            "\n",
            "See [`from_u64_lossy`](GrainId::from_u64_lossy) for more details."
        )
    };
}

pub(crate) use doc_from_bytes;
pub(crate) use doc_from_bytes_lossy;
pub(crate) use doc_to_bytes;
