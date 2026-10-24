use std::io::{self, Read};
use std::path::PathBuf;

use grain_id::GrainId;

fn read_input(file: Option<PathBuf>, string: Option<String>) -> Vec<u8> {
    if let Some(s) = string {
        s.into_bytes()
    } else if let Some(path) = file {
        std::fs::read(&path).unwrap_or_else(|e| {
            eprintln!("grain-id: {}: {}", path.display(), e);
            std::process::exit(1);
        })
    } else {
        let mut buf = Vec::new();
        io::stdin().read_to_end(&mut buf).unwrap();
        buf
    }
}

macro_rules! impl_hash_command {
    ($name:ident, $digest:ty, $doc:literal) => {
        #[derive(clap::Args, Debug)]
        #[doc = $doc]
        pub struct $name {
            /// File to hash (reads from stdin if omitted)
            #[arg(value_name = "FILE")]
            file: Option<PathBuf>,

            /// Hash a string directly
            #[arg(
                short = 's',
                long = "string",
                value_name = "STRING",
                conflicts_with = "file"
            )]
            string: Option<String>,
        }

        impl $name {
            pub fn run(self) {
                let bytes = read_input(self.file, self.string);
                println!("{}", GrainId::from_digest::<$digest>(&bytes));
            }
        }
    };
}

impl_hash_command!(Md5Args, md5::Md5, "Generate grain-id from MD5 hash");
impl_hash_command!(Sha1Args, sha1::Sha1, "Generate grain-id from SHA-1 hash");
impl_hash_command!(
    Sha256Args,
    sha2::Sha256,
    "Generate grain-id from SHA-256 hash"
);
