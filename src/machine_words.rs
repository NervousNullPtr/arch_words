/// Alias for the `i8` type.
pub type SigByte  = i8;
/// Alias for the `u8` type.
pub type Byte     = u8;
/// Alias for the `u16` type.
pub type DByte    = u16;
/// Alias for the `i16` type.
pub type SigDByte = i16;

#[cfg(target_pointer_width = "16")]
/// Alias for the `u8` type.
pub type HWord = u8;
#[cfg(target_pointer_width = "16")]
/// Alias for the `u16` type.
pub type Word  = u16;
#[cfg(target_pointer_width = "16")]
/// Alias for the `u32` type.
pub type DWord = u32;
#[cfg(target_pointer_width = "16")]
/// Alias for the `u64` type.
/// # Warning
/// This type is only defined for 32 bit machines and 16 bit machines.
pub type QWord = u64;
#[cfg(target_pointer_width = "16")]
/// Alias for the `u128` type.
pub type OWord = u128;

#[cfg(target_pointer_width = "16")]
/// Alias for the `i8` type.
pub type SigHWord = i8;
#[cfg(target_pointer_width = "16")]
/// Alias for the `i16` type.
pub type SigWord  = i16;
#[cfg(target_pointer_width = "16")]
/// Alias for the `i32` type.
pub type SigDWord = i32;
#[cfg(target_pointer_width = "16")]
/// Alias for the `i64` type.
/// # Warning
/// This type is only defined for 32 bit machines and 16 bit machines.
pub type SigQWord = i64;
#[cfg(target_pointer_width = "16")]
/// Alias for the `i128` type.
/// # Warning
/// This type is only defined for 16 bit machines.
pub type SigOWord = i128;


#[cfg(target_pointer_width = "32")]
/// Alias for the `u16` type.
pub type HWord = u16;
#[cfg(target_pointer_width = "32")]
/// Alias for the `u32` type.
pub type Word  = u32;
#[cfg(target_pointer_width = "32")]
/// Alias for the `u64` type.
pub type DWord = u64;
#[cfg(target_pointer_width = "32")]
/// Alias for the `u128` type.
/// # Warning
/// This type is only defined for 32 bit machines and 16 bit machines.
pub type QWord = u128;

#[cfg(target_pointer_width = "32")]
/// Alias for the `i16` type.
pub type SigHWord = i16;
#[cfg(target_pointer_width = "32")]
/// Alias for the `i32` type.
pub type SigWord  = i32;
#[cfg(target_pointer_width = "32")]
/// Alias for the `i64` type.
pub type SigDWord = i64;
#[cfg(target_pointer_width = "32")]
/// Alias for the `i128` type.
/// # Warning
/// This type is only defined for 32 bit machines and 16 bit machines.
pub type SigQWord = i128;


#[cfg(target_pointer_width = "64")]
/// Alias for the `u32` type.
pub type HWord = u32;
#[cfg(target_pointer_width = "64")]
/// Alias for the `u64` type.
pub type Word  = u64;
#[cfg(target_pointer_width = "64")]
/// Alias for the `u128` type.
pub type DWord = u128;

#[cfg(target_pointer_width = "64")]
/// Alias for the `i32` type.
pub type SigHWord = i32;
#[cfg(target_pointer_width = "64")]
/// Alias for the `i64` type.
pub type SigWord  = i64;
#[cfg(target_pointer_width = "64")]
/// Alias for the `i128` type.
pub type SigDWord = i128;