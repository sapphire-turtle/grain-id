# Changelog

All notable changes to this workspace will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.17.0](https://github.com/sapphire-turtle/grain-id/compare/grain-id-cli-v0.16.0...grain-id-cli-v0.17.0) - 2026-10-25

### Other

- drop package.homepage, which duplicates repository
- deprecate GrainId::as_u64 in favour of to_u64
- [**breaking**] clean up the grain-id crate and fix panics found by fuzzing
- [**breaking**] drop prost, rusqlite, sea-orm and digest integrations
- update repository URLs to sapphire-turtle
- move to a virtual workspace (crates/grain-id, cli)
