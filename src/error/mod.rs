use core::result;

mod status;
pub use self::status::Status;

/// Return type of many UEFI functions.
pub type Result<T> = result::Result<T, Status>;
