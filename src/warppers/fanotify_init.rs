///Set the close-on-exec flag (FD_CLOEXEC) on the new file descriptor.<br/>
///See the description of the O_CLOEXEC flag in open(2).
pub const FAN_CLOEXEC: i32 = 0x0000_0001;
///Enable the nonblocking flag (O_NONBLOCK) for the file descriptor.   <br/>
///Reading from the file descriptor will not block. <br/>
///Instead, if no data is available, read(2) fails with the error EAGAIN
pub const FAN_NONBLOCK: i32 = 0x0000_0002;
///This is the default value.  It does not need to be specified.
///This value only allows the receipt of events notifying that a file has been accessed.   <br/>
///Permission decisions before the file is accessed are not possible.
pub const FAN_CLASS_NOTIF: i32 = 0x0000_0000;
///This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.   <br/>
///It is intended for event listeners that need to access files when they already contain their finalcontent.   <br/>
///This notification class might be used by malware detection programs, for example.
pub const FAN_CLASS_CONTENT: i32 = 0x0000_0004;
///This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.   <br/>
///It is intended for event listeners that need to access files before they contain their final data.<br/>
///This notification class might be used by hierarchical storage managers, for example.
pub const FAN_CLASS_PRE_CONTENT: i32 = 0x0000_0008;

pub const FAN_UNLIMITED_QUEUE: i32 = 0x0000_0010;
pub const FAN_UNLIMITED_MARKS: i32 = 0x0000_0020;

pub const FAN_MARK_ADD: i32 = 0x0000_0001;
pub const FAN_MARK_REMOVE: i32 = 0x0000_0002;
pub const FAN_MARK_DONT_FOLLOW: i32 = 0x0000_0004;
pub const FAN_MARK_ONLYDIR: i32 = 0x0000_0008;
pub const FAN_MARK_INODE: i32 = 0x0000_0000;
pub const FAN_MARK_MOUNT: i32 = 0x0000_0010;

pub const FAN_MARK_FILESYSTEM: i32 = 0x0000_0100;
pub const FAN_MARK_IGNORED_MASK: i32 = 0x0000_0020;
pub const FAN_MARK_IGNORED_SURV_MODIFY: i32 = 0x0000_0040;
pub const FAN_MARK_FLUSH: i32 = 0x0000_0080;

pub const FANOTIFY_METADATA_VERSION: u8 = 3;

pub const FAN_ALLOW: u32 = 0x01;
pub const FAN_DENY: u32 = 0x02;

pub const FAN_NOFD: i32 = -1;
pub const FAN_Q_OVERFLOW: u64 = 0x0000_4000;