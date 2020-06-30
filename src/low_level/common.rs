use lazy_static::lazy_static;
use libc::{__s32, __u16, __u32, __u64, __u8};
use std::mem;
#[derive(Debug)]
#[repr(C)]
/// The fanotify generates event struct.
pub struct fanotify_event_metadata {
    pub event_len: __u32,
    pub vers: __u8,
    pub reserved: __u8,
    pub metadata_len: __u16,
    pub mask: __u64,
    pub fd: __s32,
    pub pid: __s32,
}
#[derive(Debug)]
#[repr(C)]
/// It is used to control file access.
pub struct fanotify_response {
    pub fd: __s32,
    pub response: __u32,
}
lazy_static! {
    /// Get current platform sizeof of fanotify_event_metadata.
    pub static ref FAN_EVENT_METADATA_LEN: usize = mem::size_of::<fanotify_event_metadata>();
}
/// This const is used to be compared to vers field of fanotify_event_metadata to verify that the structures returned at run time match the structures defined at compile time.  </br>
/// In case of a mismatch, the application should abandon trying to use the fanotify file descriptor.
pub const FANOTIFY_METADATA_VERSION: u8 = 3;
/// Allow the file operation.
pub const FAN_ALLOW: u32 = 0x01;
/// Deny the file operation.
pub const FAN_DENY: u32 = 0x02;
/// Indicates a queue overflow.
pub const FAN_NOFD: i32 = -1;
/// The event queue exceeded the limit of 16384 entries.  <br/>
/// This limit can be overridden by specifying the FAN_UNLIMITED_QUEUE flag when calling fanotify_init(2).
pub const FAN_Q_OVERFLOW: u64 = 0x0000_4000;