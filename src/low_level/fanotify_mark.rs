use libc;
use std::ffi;
use std::io;
///Create an event when a file or directory (but see BUGS) is accessed (read).
pub const FAN_ACCESS: u64 = 0x0000_0001;
///Create an event when a file is modified (write).
pub const FAN_MODIFY: u64 = 0x0000_0002;
///Create an event when a writable file is closed.
pub const FAN_CLOSE_WRITE: u64 = 0x0000_0008;
///Create an event when a read-only file or directory is closed.
pub const FAN_CLOSE_NOWRITE: u64 = 0x0000_0010;
///Create an event when a file or directory is opened.
pub const FAN_OPEN: u64 = 0x0000_0020;
///Create an event when a permission to open a file or directory is requested. <br/>
///An fanotify file descriptor created with FAN_CLASS_PRE_CONTENT or FAN_CLASS_CONTENT is required.
pub const FAN_OPEN_PERM: u64 = 0x0001_0000;
///Create an event when a permission to read a file or directoryis requested. <br/>
///An fanotify file descriptor created with FAN_CLASS_PRE_CONTENT or FAN_CLASS_CONTENT is required.
pub const FAN_ACCESS_PERM: u64 = 0x0002_0000;
///Create events for directories—for example, when opendir(3),readdir(3) (but see BUGS), and closedir(3) are called. <br/>
///Without this flag, events are created only for files. <br/>
///In the context of directory entry events, such as FAN_CREATE,FAN_DELETE, FAN_MOVED_FROM, and FAN_MOVED_TO, specifying the flag FAN_ONDIR is required in order to create events when subdirectory entries are modified (i.e., mkdir(2)/ rmdir(2)).
pub const FAN_ONDIR: u64 = 0x4000_0000;
///Events for the immediate children of marked directories shall be created.   <br/>
///The flag has no effect when marking mounts and filesystems.   <br/>
///Note that events are not generated for children of the subdirectories of marked directories. <br/>
///More specifically, the directory entry modification events FAN_CREATE, FAN_DELETE, FAN_MOVED_FROM, and FAN_MOVED_TO arenot generated for any entry modifications performed inside subdirectories of marked directories.   <br/>
///Note that the events FAN_DELETE_SELF and FAN_MOVE_SELF are not generated for children of marked directories.   <br/>
///To monitor complete directory trees it is necessary to mark the relevant mount or filesystem. <br/>
pub const FAN_EVENT_ON_CHILD: u64 = 0x0800_0000;
///A file is closed (FAN_CLOSE_WRITE|FAN_CLOSE_NOWRITE). <br/>
pub const FAN_CLOSE: u64 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE;
///The events in mask will be added to the mark mask (or to the ignore mask).  mask must be nonempty or the error EINVAL will occur.
pub const FAN_MARK_ADD: i32 = 0x0000_0001;
///The events in argument mask will be removed from the mark mask (or from the ignore mask).  mask must be nonempty or the error EINVAL will occur.
pub const FAN_MARK_REMOVE: i32 = 0x0000_0002;
///If pathname is a symbolic link, mark the link itself, rather than the file to which it refers. <br/>
///(By default,fanotify_mark() dereferences pathname if it is a symbolic link.)
pub const FAN_MARK_FLUSH: i32 = 0x0000_0080;
///Remove either all marks for filesystems, all marks for mounts,or all marks for directories and files from the fanotify group.  <br/>
///If flags contains FAN_MARK_MOUNT, all marks for mounts are removed from the group.  <br/>
///If flags contains FAN_MARK_FILESYSTEM, all marks for filesystems are removed from the group.  <br/>
///Otherwise, all marks for directories and files are removed.  <br/>
///No flag other than and at most one of the flags FAN_MARK_MOUNT or FAN_MARK_FILESYSTEM can be used in conjunction with FAN_MARK_FLUSH.  mask is ignored.
pub const FAN_MARK_DONT_FOLLOW: i32 = 0x0000_0004;
///If the filesystem object to be marked is not a directory, the error ENOTDIR shall be raised.
pub const FAN_MARK_ONLYDIR: i32 = 0x0000_0008;
///Mark the inode specified by pathname.<br/>
///It is default way to mark.
pub const FAN_MARK_INODE: i32 = 0x0000_0000;
///Mark the mount point specified by pathname.  If pathname is not itself a mount point, the mount point containing pathname will be marked.  <br/>
///All directories, subdirectories, and the contained files of the mount point will be monitored.  <br/>
///The events which require the fanotify_fd file descriptor to have been initialized with the flag FAN_REPORT_FID, such as FAN_CREATE, FAN_ATTRIB, FAN_MOVE, and FAN_DELETE_SELF, cannot be provided as a mask when flags contains FAN_MARK_MOUNT.<br/>
///Attempting to do so will result in the error EINVAL being returned.
pub const FAN_MARK_MOUNT: i32 = 0x0000_0010;
///Mark the filesystem specified by pathname.  <br/>
///The filesystem containing pathname will be marked.  <br/>
///All the contained files and directories of the filesystem from any mount point will be monitored.
pub const FAN_MARK_FILESYSTEM: i32 = 0x0000_0100;
///The events in mask shall be added to or removed from the ignore mask.
pub const FAN_MARK_IGNORED_MASK: i32 = 0x0000_0020;
///The ignore mask shall survive modify events.  <br/>
///If this flag is not set, the ignore mask is cleared when a modify event occurs for the ignored file or directory.
pub const FAN_MARK_IGNORED_SURV_MODIFY: i32 = 0x0000_0040;
pub fn fanotify_mark(fd: i32, flags: u32, mask: u64, dirfd: i32, path: &'static str)->Result<(),io::Error> {
    let path = ffi::CString::new(path).unwrap().as_ptr();
    unsafe {
        match libc::fanotify_mark(fd, flags, mask, dirfd, path){
            0=>{
                return Ok(());
            }
            _=>{
                return Err(io::Error::last_os_error());
            }
        }
    }
}
