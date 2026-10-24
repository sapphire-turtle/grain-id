use super::*;

impl GrainId {
    /// Generate a [`GrainId`] from arbitrary bytes using a fixed-output hash function.
    ///
    /// The first (most significant) 35 bits of the hash output are used to construct the ID.
    /// The hash function's output must be at least 5 bytes (40 bits); this is asserted at runtime.
    /// All standard cryptographic hash functions satisfy this requirement.
    ///
    /// # Panics
    ///
    /// Panics if the hash function produces fewer than 5 bytes of output.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grain_id::*;
    /// use sha2::Sha256;
    ///
    /// let id = GrainId::from_digest::<Sha256>(b"hello world");
    /// // Deterministic: same input always yields same ID.
    /// assert_eq!(id, GrainId::from_digest::<Sha256>(b"hello world"));
    /// // Different inputs yield different IDs.
    /// assert_ne!(id, GrainId::from_digest::<Sha256>(b"goodbye world"));
    /// ```
    pub fn from_digest<D: ::digest::Digest>(input: &[u8]) -> Self {
        let output = D::digest(input);
        assert!(
            output.len() >= 5,
            "digest output is {} bytes, but at least 5 bytes are required",
            output.len()
        );
        Self::from_byte_prefix(output[..5].try_into().unwrap())
    }

    /// Generate a [`GrainId`] from arbitrary bytes using an extendable-output function (XOF).
    ///
    /// Exactly 5 bytes are read from the XOF output, and the top 35 bits are used to
    /// construct the ID.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use grain_id::*;
    /// use shake::Shake128;
    ///
    /// let id = GrainId::from_xof::<Shake128>(b"hello world");
    /// // Deterministic: same input always yields same ID.
    /// assert_eq!(id, GrainId::from_xof::<Shake128>(b"hello world"));
    /// // Different inputs yield different IDs.
    /// assert_ne!(id, GrainId::from_xof::<Shake128>(b"goodbye world"));
    /// ```
    pub fn from_xof<D>(input: &[u8]) -> Self
    where
        D: ::digest::ExtendableOutput + ::digest::Update + Default,
    {
        use ::digest::XofReader;
        let mut hasher = D::default();
        hasher.update(input);
        let mut reader = hasher.finalize_xof();
        let mut buf = [0u8; 5];
        reader.read(&mut buf);
        Self::from_byte_prefix(&buf)
    }
}
