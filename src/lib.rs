#![no_std]

//! Alternative type definitions for integer types.
//! # Prefixes
//! * `Sig` - A signed type.
//! * `H` - "Half".
//! * `D` - "Double".
//! * `Q` - "Quad / Quadruple".
//! * `O` - "Octa".
/// Machine-specific type definitions.
/// Here, a "word" is the pointer size of a machine, e. g. the data it can deal with using one operation.
pub mod machine_words;
/// Conventional unified type definitions, unconditional.
/// Here, a "word" is a fixed 16-bits (or 2 Bytes) long.
pub mod unified_words;