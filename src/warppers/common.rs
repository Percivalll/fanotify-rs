use lazy_static::lazy_static;
use libc;
use std::mem;
#[derive(Debug)]
#[repr(C)]
pub struct fanotify_event_metadata {
    pub event_len: libc::__u32,
    pub vers: libc::__u8,
    pub reserved: libc::__u8,
    pub metadata_len: libc::__u16,
    pub mask: libc::__u64,
    pub fd: libc::__s32,
    pub pid: libc::__s32,
}
lazy_static! {
    pub static ref FAN_EVENT_METADATA_LEN: usize = mem::size_of::<fanotify_event_metadata>();
}