use std::io;
/// Set the close-on-exec flag (FD_CLOEXEC) on the new file descriptor.<br/>
/// See the description of the O_CLOEXEC flag in open(2).
pub const FAN_CLOEXEC: u32 = 0x0000_0001;
/// Enable the nonblocking flag (O_NONBLOCK) for the file descriptor.   <br/>
/// Reading from the file descriptor will not block. <br/>
/// Instead, if no data is available, read(2) fails with the error EAGAIN
pub const FAN_NONBLOCK: u32 = 0x0000_0002;
/// This is the default value.  It does not need to be specified.
/// This value only allows the receipt of events notifying that a file has been accessed.   <br/>
/// Permission decisions before the file is accessed are not possible.
pub const FAN_CLASS_NOTIF: u32 = 0x0000_0000;
/// This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.   <br/>
/// It is intended for event listeners that need to access files when they already contain their final content.   <br/>
/// This notification class might be used by malware detection programs, for example.
pub const FAN_CLASS_CONTENT: u32 = 0x0000_0004;
/// This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.   <br/>
/// It is intended for event listeners that need to access files before they contain their final data.<br/>
/// This notification class might be used by hierarchical storage managers, for example.
pub const FAN_CLASS_PRE_CONTENT: u32 = 0x0000_0008;
/// Remove the limit of 16384 events for the event queue.  <br/>
/// Use of this flag requires the CAP_SYS_ADMIN capability.
pub const FAN_UNLIMITED_QUEUE: u32 = 0x0000_0010;
/// Remove the limit of 8192 marks.  <br/>
/// Use of this flag requires the CAP_SYS_ADMIN capability.
pub const FAN_UNLIMITED_MARKS: u32 = 0x0000_0020;
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_LARGEFILE: u32 = 0;
pub const O_CLOEXEC: u32 = 0x80000;
pub const O_APPEND: u32 = 1024;
pub const O_DSYNC: u32 = 4096;
pub const O_NOATIME: u32 = 0o1000000;
pub const O_NONBLOCK: u32 = 2048;
pub const O_SYNC: u32 = 1052672;
/// # Examples
/// ```
/// use fanotify::low_level::fanotify_init::{fanotify_init, FAN_CLASS_NOTIF, O_RDONLY};
/// let fs = fanotify_init(FAN_CLASS_NOTIF, O_RDONLY).unwrap();
/// assert!(fs > 0)
/// ```
pub fn fanotify_init(flags: u32, event_f_flags: u32) -> Result<i32, io::Error> {
    unsafe {
        match libc::fanotify_init(flags, event_f_flags) {
            -1 => {
                return Err(io::Error::last_os_error());
            }
            fd => {
                return Ok(fd);
            }
        };
    }
}
