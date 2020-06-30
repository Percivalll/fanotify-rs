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