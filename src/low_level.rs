use lazy_static::lazy_static;
use libc;
use libc::{__s32, __u16, __u32, __u64, __u8};
use std::ffi;
use std::io;
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
/// This const is used to be compared to vers field of fanotify_event_metadata to verify that the structures returned at run time match the structures defined at compile time.
///
///
/// In case of a mismatch, the application should abandon trying to use the fanotify file descriptor.
pub const FANOTIFY_METADATA_VERSION: u8 = 3;
/// Allow the file operation.
pub const FAN_ALLOW: u32 = 0x01;
/// Deny the file operation.
pub const FAN_DENY: u32 = 0x02;
/// Indicates a queue overflow.
pub const FAN_NOFD: i32 = -1;
/// The event queue exceeded the limit of 16384 entries.
///
///
/// This limit can be overridden by specifying the FAN_UNLIMITED_QUEUE flag when calling fanotify_init(2).
pub const FAN_Q_OVERFLOW: u64 = 0x0000_4000;
/// Set the close-on-exec flag (FD_CLOEXEC) on the new file descriptor.
///
////
/// See the description of the O_CLOEXEC flag in open(2).
pub const FAN_CLOEXEC: u32 = 0x0000_0001;
/// Enable the nonblocking flag (O_NONBLOCK) for the file descriptor.
///
///
/// Reading from the file descriptor will not block. <br/>
/// Instead, if no data is available, read(2) fails with the error EAGAIN
pub const FAN_NONBLOCK: u32 = 0x0000_0002;
/// This is the default value.  It does not need to be specified.
///
///
/// This value only allows the receipt of events notifying that a file has been accessed.   <br/>
/// Permission decisions before the file is accessed are not possible.
pub const FAN_CLASS_NOTIF: u32 = 0x0000_0000;
/// This value allows the receipt of events notifying that a file has been accessed and events for permission decisions if a file may be accessed.  
///
///
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
/// This value allows only read access.
pub const O_RDONLY: u32 = 0;
/// This value allows only write access.
pub const O_WRONLY: u32 = 1;
/// This value allows read and write access.
pub const O_RDWR: u32 = 2;
pub const O_LARGEFILE: u32 = 0;
pub const O_CLOEXEC: u32 = 0x80000;
pub const O_APPEND: u32 = 1024;
pub const O_DSYNC: u32 = 4096;
pub const O_NOATIME: u32 = 0o1000000;
pub const O_NONBLOCK: u32 = 2048;
pub const O_SYNC: u32 = 1052672;
/// Create an event when a file or directory is accessed (read).
pub const FAN_ACCESS: u64 = 0x0000_0001;
/// Create an event when a file is modified (write).
pub const FAN_MODIFY: u64 = 0x0000_0002;
/// Create an event when a writable file is closed.
pub const FAN_CLOSE_WRITE: u64 = 0x0000_0008;
/// Create an event when a read-only file or directory is closed.
pub const FAN_CLOSE_NOWRITE: u64 = 0x0000_0010;
/// Create an event when a file or directory is opened.
pub const FAN_OPEN: u64 = 0x0000_0020;
/// Create an event when a permission to open a file or directory is requested. <br/>
/// An fanotify file descriptor created with FAN_CLASS_PRE_CONTENT or FAN_CLASS_CONTENT is required.
pub const FAN_OPEN_PERM: u64 = 0x0001_0000;
/// Create an event when a permission to read a file or directoryis requested. <br/>
/// An fanotify file descriptor created with FAN_CLASS_PRE_CONTENT or FAN_CLASS_CONTENT is required.
pub const FAN_ACCESS_PERM: u64 = 0x0002_0000;
/// Create events for directories—for example, when opendir(3),readdir(3) (but see BUGS), and closedir(3) are called. <br/>
/// Without this flag, events are created only for files. <br/>
/// In the context of directory entry events, such as FAN_CREATE,FAN_DELETE, FAN_MOVED_FROM, and FAN_MOVED_TO, specifying the flag FAN_ONDIR is required in order to create events when subdirectory entries are modified (i.e., mkdir(2)/ rmdir(2)).
pub const FAN_ONDIR: u64 = 0x4000_0000;
/// Events for the immediate children of marked directories shall be created.
///
///
/// The flag has no effect when marking mounts and filesystems.   <br/>
/// Note that events are not generated for children of the subdirectories of marked directories. <br/>
/// More specifically, the directory entry modification events FAN_CREATE, FAN_DELETE, FAN_MOVED_FROM, and FAN_MOVED_TO arenot generated for any entry modifications performed inside subdirectories of marked directories.   <br/>
/// Note that the events FAN_DELETE_SELF and FAN_MOVE_SELF are not generated for children of marked directories.   <br/>
/// To monitor complete directory trees it is necessary to mark the relevant mount or filesystem. <br/>
pub const FAN_EVENT_ON_CHILD: u64 = 0x0800_0000;
/// A file is closed (FAN_CLOSE_WRITE|FAN_CLOSE_NOWRITE). <br/>
pub const FAN_CLOSE: u64 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE;
/// The events in mask will be added to the mark mask (or to the ignore mask).  mask must be nonempty or the error EINVAL will occur.
pub const FAN_MARK_ADD: u32 = 0x0000_0001;
/// The events in argument mask will be removed from the mark mask (or from the ignore mask).  mask must be nonempty or the error EINVAL will occur.
pub const FAN_MARK_REMOVE: u32 = 0x0000_0002;
/// Remove either all marks for filesystems, all marks for mounts,or all marks for directories and files from the fanotify group.  <br/>
/// If flags contains FAN_MARK_MOUNT, all marks for mounts are removed from the group.  <br/>
/// If flags contains FAN_MARK_FILESYSTEM, all marks for filesystems are removed from the group.  <br/>
/// Otherwise, all marks for directories and files are removed.  <br/>
/// No flag other than and at most one of the flags FAN_MARK_MOUNT or FAN_MARK_FILESYSTEM can be used in conjunction with FAN_MARK_FLUSH.  mask is ignored.
pub const FAN_MARK_FLUSH: u32 = 0x0000_0080;
/// If pathname is a symbolic link, mark the link itself, rather than the file to which it refers. <br/>
/// (By default,fanotify_mark() dereferences pathname if it is a symbolic link.)
pub const FAN_MARK_DONT_FOLLOW: u32 = 0x0000_0004;
/// If the filesystem object to be marked is not a directory, the error ENOTDIR shall be raised.
pub const FAN_MARK_ONLYDIR: u32 = 0x0000_0008;
/// Mark the inode specified by pathname.<br/>
/// It is default way to mark.
pub const FAN_MARK_INODE: u32 = 0x0000_0000;
/// Mark the mount point specified by pathname.  If pathname is not itself a mount point, the mount point containing pathname will be marked.  <br/>
/// All directories, subdirectories, and the contained files of the mount point will be monitored.  <br/>
/// The events which require the fanotify_fd file descriptor to have been initialized with the flag FAN_REPORT_FID, such as FAN_CREATE, FAN_ATTRIB, FAN_MOVE, and FAN_DELETE_SELF, cannot be provided as a mask when flags contains FAN_MARK_MOUNT.<br/>
/// Attempting to do so will result in the error EINVAL being returned.
pub const FAN_MARK_MOUNT: u32 = 0x0000_0010;
/// Mark the filesystem specified by pathname.  <br/>
/// The filesystem containing pathname will be marked.  <br/>
/// All the contained files and directories of the filesystem from any mount point will be monitored.
pub const FAN_MARK_FILESYSTEM: u32 = 0x0000_0100;
/// The events in mask shall be added to or removed from the ignore mask.
pub const FAN_MARK_IGNORED_MASK: u32 = 0x0000_0020;
/// The ignore mask shall survive modify events.  <br/>
/// If this flag is not set, the ignore mask is cleared when a modify event occurs for the ignored file or directory.
pub const FAN_MARK_IGNORED_SURV_MODIFY: u32 = 0x0000_0040;
pub const AT_FDCWD: i32 = -100;
pub const AT_SYMLINK_NOFOLLOW: i32 = 0x100;
pub const AT_REMOVEDIR: i32 = 0x200;
pub const AT_SYMLINK_FOLLOW: i32 = 0x400;
pub const AT_NO_AUTOMOUNT: i32 = 0x800;
pub const AT_EMPTY_PATH: i32 = 0x1000;
/// Initializes a new fanotify group and returns a file descriptor for the event queue associated with the group.<br/>
///
/// The file descriptor is used in calls to fanotify_mark(2) to specify the files, directories, mounts or filesystems for which fanotify events shall be created.  
/// These events are received by reading from the file descriptor.  <br/>
/// Some events are only informative, indicating that a file has been accessed. 
/// Other events can be used to determine whether another application is permitted to access a file or directory. 
/// Permission to access filesystem objects is granted by writing to the file descriptor.
/// Multiple programs may be using the fanotify interface at the same time to monitor the same files.<br/>
/// In the current implementation, the number of fanotify groups per user is limited to 128.  This limit cannot be overridden.
/// Calling fanotify_init() requires the CAP_SYS_ADMIN capability.  
/// This constraint might be relaxed in future versions of the API. <br/>
/// Therefore, certain additional capability checks have been implemented as indicated below.<br/>
/// The `flags` argument contains a multi-bit field defining the notification class of the listening application and further single bit fields specifying the behavior of the file descriptor.<br/>
/// If multiple listeners for permission events exist, the notification class is used to establish the sequence in which the listeners receive the events.<br/>
/// 
/// Only one of the following notification classes may be specified in `flags`:<br/>
/// * FAN_CLASS_PRE_CONTENT
/// * FAN_CLASS_CONTENT
/// * FAN_CLASS_NOTIF
/// 
/// Listeners with different notification classes will receive events in the order `FAN_CLASS_PRE_CONTENT`, `FAN_CLASS_CONTENT`, `FAN_CLASS_NOTIF`.
/// The order of notification for listeners in the same notification class is undefined.<br/>
/// The following bits can additionally be set in flags:<br/>
/// * FAN_CLOEXEC
/// * FAN_NONBLOCK
/// * FAN_UNLIMITED_QUEUE
/// * FAN_UNLIMITED_MARKS
/// * FAN_REPORT_TID (since Linux 4.20)
/// * FAN_REPORT_FID (since Linux 5.1)
/// 
/// The event_f_flags argument defines the file status flags that will be set on the open file descriptions that are created for fanotify events.  <br/>
/// For details of these flags, see the description of the flags values in open(2).  event_f_flags includes a multi-bit field for the access mode.  <br/>
/// This field can take the following values:
/// * O_RDONLY       
/// * O_WRONLY
/// * O_RDWR 
///     
/// Additional bits can be set in event_f_flags.  The most useful values are:
/// * O_LARGEFILE
/// * O_CLOEXEC (since Linux 3.18)
///
/// The following are also allowable: `O_APPEND`, `O_DSYNC`, `O_NOATIME`,`O_NONBLOCK`, and `O_SYNC`.  Specifying any other flag in `event_f_flags` yields the error `EINVAL`.
/// # Examples
/// ```
/// use fanotify::low_level::*;
/// let fd = fanotify_init(FAN_CLASS_NOTIF, O_RDONLY).unwrap();
/// assert!(fd > 0)
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
/// Adds, removes, or modifies an fanotify mark on a filesystem object.  
// The caller must have read permission on the filesystem object that is to be marked.
///
/// The fanotify_fd argument is a file descriptor returned by `fanotify_init()`.
/// 
/// `flags` is a bit mask describing the modification to perform.  It must include exactly one of the following values:
/// * FAN_MARK_ADD
/// * FAN_MARK_REMOVE
/// * FAN_MARK_FLUSH
/// 
/// If none of the values above is specified, or more than one is specified, the call fails with the error `EINVAL`.
///
/// In addition, zero or more of the following values may be ORed into `flags`:
/// * FAN_MARK_DONT_FOLLOW
/// * FAN_MARK_ONLYDIR
/// * FAN_MARK_MOUNT
/// * FAN_MARK_FILESYSTEM(since Linux 4.20)
/// * FAN_MARK_IGNORED_MASK
/// * FAN_MARK_IGNORED_SURV_MODIFY
///
///
/// `mask` defines which events shall be listened for (or which shall be ignored).  It is a bit mask composed of the following values:
/// * FAN_ACCESS
/// * FAN_MODIFY
/// * FAN_CLOSE_WRITE
/// * FAN_CLOSE_NOWRITE
/// * FAN_OPEN
/// * FAN_OPEN_EXEC (since Linux 5.0)
/// * FAN_ATTRIB (since Linux 5.1)
/// * FAN_CREATE (since Linux 5.1)
/// * FAN_DELETE (since Linux 5.1)
/// * FAN_DELETE_SELF (since Linux 5.1)
/// * FAN_MOVED_FROM (since Linux 5.1)
/// * FAN_MOVED_TO (since Linux 5.1)
/// * FAN_MOVE_SELF (since Linux 5.1)
/// * FAN_OPEN_PERM
/// * FAN_OPEN_EXEC_PERM (since Linux 5.0)
/// * FAN_ACCESS_PERM
/// * FAN_ONDIR
/// * FAN_EVENT_ON_CHILD
/// 
/// The following composed values are defined:
/// * FAN_CLOSE
/// * FAN_MOVE(since Linux 5.1)
///
///
/// The filesystem object to be marked is determined by the file descriptor `dirfd` and the pathname specified in pathname:
/// * If pathname is `NULL`, `dirfd` defines the filesystem object to be marked.
/// * If pathname is `NULL`, and `dirfd` takes the special value `AT_FDCWD`,the current working directory is to be marked.
/// * If pathname is absolute, it defines the filesystem object to be marked, and `dirfd` is ignored.
/// * If pathname is relative, and `dirfd` does not have the value `AT_FDCWD`, then the filesystem object to be marked is determined by interpreting pathname relative the directory referred to by `dirfd.`
/// * If pathname is relative, and `dirfd` has the value `AT_FDCWD`, then the filesystem object to be marked is determined by interpreting pathname relative the current working directory.
/// # Examples
/// ```
/// use fanotify::low_level::*;
/// let fd = fanotify_init(FAN_CLASS_NOTIF, O_RDONLY).unwrap();
/// fanotify_mark(fd, FAN_MARK_ADD, FAN_OPEN | FAN_CLOSE, AT_FDCWD, "./").unwrap();
/// ```
pub fn fanotify_mark(
    fanotify_fd: i32,
    flags: u32,
    mask: u64,
    dirfd: i32,
    path: &'static str,
) -> Result<(), io::Error> {
    unsafe {
        match libc::fanotify_mark(
            fanotify_fd,
            flags,
            mask,
            dirfd,
            ffi::CString::new(path).unwrap().as_ptr(),
        ) {
            0 => {
                return Ok(());
            }
            _ => {
                return Err(io::Error::last_os_error());
            }
        }
    }
}
