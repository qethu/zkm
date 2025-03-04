//! Ported from Entrypoint for SP1 zkVM.

mod halt;
mod io;
mod keccak;
mod memory;
mod sha256;
mod sys;

pub use halt::*;
pub use io::*;
pub use keccak::*;
pub use memory::*;
pub use sha256::*;
pub use sys::*;

/// These codes MUST match the codes in `core/src/runtime/syscall.rs`. There is a derived test
/// that checks that the enum is consistent with the syscalls.
///
/// Halts the program.
pub const HALT: u32 = 4246u32;

/// Writes to a file descriptor. Currently only used for `STDOUT/STDERR`.
pub const WRITE: u32 = 4004u32;

/// Executes `HINT_LEN`.
pub const HINT_LEN: u32 = 0x00_00_00_F0;

/// Executes `HINT_READ`.
pub const HINT_READ: u32 = 0x00_00_00_F1;

/// Executes `HINT_READ`.
pub const VERIFY: u32 = 0x00_00_00_F2;

/// Executes `KECCAK_PERMUTE`.
pub const KECCAK_PERMUTE: u32 = 0x00_01_01_09;

/// Executes `SHA_EXTEND`.
pub const SHA_EXTEND: u32 = 0x00_30_01_05;

/// Executes `SHA_COMPRESS`.
pub const SHA_COMPRESS: u32 = 0x00_01_01_06;
