use crate::low_level::{
    close_fd, fanotify_init, fanotify_mark, fanotify_read, FanotifyEventMetadata, AT_FDCWD,
    FAN_ALLOW, FAN_CLASS_CONTENT, FAN_CLASS_NOTIF, FAN_CLASS_PRE_CONTENT, FAN_CLOEXEC, FAN_DENY,
    FAN_MARK_ADD, FAN_MARK_FLUSH, FAN_MARK_MOUNT, FAN_MARK_REMOVE, FAN_NONBLOCK, O_CLOEXEC,
    O_RDONLY,
};
use crate::FanotifyPath;
use enum_iterator::{all, Sequence};
use std::fs::read_link;
use std::io::Error;
use std::os::fd::{AsFd, BorrowedFd};

pub use crate::low_level::{
    FAN_ACCESS, FAN_ACCESS_PERM, FAN_ATTRIB, FAN_CLOSE, FAN_CLOSE_NOWRITE, FAN_CLOSE_WRITE,
    FAN_CREATE, FAN_DELETE, FAN_DELETE_SELF, FAN_EVENT_ON_CHILD, FAN_MODIFY, FAN_MOVE,
    FAN_MOVED_FROM, FAN_MOVED_TO, FAN_MOVE_SELF, FAN_ONDIR, FAN_OPEN, FAN_OPEN_EXEC,
    FAN_OPEN_EXEC_PERM, FAN_OPEN_PERM,
};

pub struct Fanotify {
    fd: i32,
}

impl AsFd for Fanotify {
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.fd) }
    }
}

impl<T> From<T> for Fanotify
where
    T: Into<i32>,
{
    fn from(raw: T) -> Fanotify {
        Fanotify { fd: raw.into() }
    }
}

#[derive(Debug, Clone, Copy, Sequence, PartialEq)]
pub enum FanEvent {
    Access = FAN_ACCESS as isize,
    AccessPerm = FAN_ACCESS_PERM as isize,
    Attrib = FAN_ATTRIB as isize,
    Close = FAN_CLOSE as isize,
    CloseNowrite = FAN_CLOSE_NOWRITE as isize,
    CloseWrite = FAN_CLOSE_WRITE as isize,
    Create = FAN_CREATE as isize,
    Delete = FAN_DELETE as isize,
    DeleteSelf = FAN_DELETE_SELF as isize,
    EventOnChild = FAN_EVENT_ON_CHILD as isize,
    Modify = FAN_MODIFY as isize,
    Move = FAN_MOVE as isize,
    MovedFrom = FAN_MOVED_FROM as isize,
    MovedTo = FAN_MOVED_TO as isize,
    MoveSelf = FAN_MOVE_SELF as isize,
    Ondir = FAN_ONDIR as isize,
    Open = FAN_OPEN as isize,
    OpenExec = FAN_OPEN_EXEC as isize,
    OpenExecPerm = FAN_OPEN_EXEC_PERM as isize,
    OpenPerm = FAN_OPEN_PERM as isize,
}

impl From<FanEvent> for u64 {
    fn from(event: FanEvent) -> u64 {
        match event {
            FanEvent::Access => FAN_ACCESS,
            FanEvent::AccessPerm => FAN_ACCESS_PERM,
            FanEvent::Attrib => FAN_ATTRIB,
            FanEvent::Close => FAN_CLOSE,
            FanEvent::CloseNowrite => FAN_CLOSE_NOWRITE,
            FanEvent::CloseWrite => FAN_CLOSE_WRITE,
            FanEvent::Create => FAN_CREATE,
            FanEvent::Delete => FAN_DELETE,
            FanEvent::DeleteSelf => FAN_DELETE_SELF,
            FanEvent::EventOnChild => FAN_EVENT_ON_CHILD,
            FanEvent::Modify => FAN_MODIFY,
            FanEvent::Move => FAN_MOVE,
            FanEvent::MovedFrom => FAN_MOVED_FROM,
            FanEvent::MovedTo => FAN_MOVED_TO,
            FanEvent::MoveSelf => FAN_MOVE_SELF,
            FanEvent::Ondir => FAN_ONDIR,
            FanEvent::Open => FAN_OPEN,
            FanEvent::OpenExec => FAN_OPEN_EXEC,
            FanEvent::OpenExecPerm => FAN_OPEN_EXEC_PERM,
            FanEvent::OpenPerm => FAN_OPEN_PERM,
        }
    }
}

pub fn events_from_mask(mask: u64) -> Vec<FanEvent> {
    all::<FanEvent>()
        .filter(|flag| (mask & (*flag as u64)) != 0)
        .collect::<Vec<FanEvent>>()
}

#[derive(Debug)]
pub enum FanotifyResponse {
    Allow,
    Deny,
}

impl From<FanotifyResponse> for u32 {
    fn from(resp: FanotifyResponse) -> u32 {
        match resp {
            FanotifyResponse::Allow => FAN_ALLOW,
            FanotifyResponse::Deny => FAN_DENY,
        }
    }
}

#[derive(Debug)]
pub struct Event {
    pub fd: i32,
    pub path: String,
    pub events: Vec<FanEvent>,
    pub pid: i32,
}

impl From<FanotifyEventMetadata> for Event {
    fn from(metadata: FanotifyEventMetadata) -> Self {
        let path = read_link(format!("/proc/self/fd/{}", metadata.fd)).unwrap_or_default();
        Event {
            fd: metadata.fd,
            path: path.to_str().unwrap().to_string(),
            events: events_from_mask(metadata.mask),
            pid: metadata.pid,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum FanotifyMode {
    PRECONTENT,
    CONTENT,
    NOTIF,
}

impl FanotifyMode {
    fn to_fan_class(self) -> u32 {
        match self {
            FanotifyMode::PRECONTENT => FAN_CLASS_PRE_CONTENT,
            FanotifyMode::CONTENT => FAN_CLASS_CONTENT,
            FanotifyMode::NOTIF => FAN_CLASS_NOTIF,
        }
    }
}

impl Fanotify {
    pub fn new_with_blocking(mode: FanotifyMode) -> Self {
        Fanotify {
            fd: fanotify_init(FAN_CLOEXEC | mode.to_fan_class(), (O_CLOEXEC | O_RDONLY) as u32).unwrap(),
        }
    }

    pub fn new_with_nonblocking(mode: FanotifyMode) -> Self {
        Fanotify {
            fd: fanotify_init(
                FAN_CLOEXEC | FAN_NONBLOCK | mode.to_fan_class(),
                (O_CLOEXEC | O_RDONLY) as u32,
            )
            .unwrap(),
        }
    }

    pub fn add_path<P: ?Sized + FanotifyPath>(&self, mode: u64, path: &P) -> Result<(), Error> {
        fanotify_mark(self.fd, FAN_MARK_ADD, mode, AT_FDCWD, path)?;
        Ok(())
    }

    pub fn add_mountpoint<P: ?Sized + FanotifyPath>(
        &self,
        mode: u64,
        path: &P,
    ) -> Result<(), Error> {
        fanotify_mark(self.fd, FAN_MARK_ADD | FAN_MARK_MOUNT, mode, AT_FDCWD, path)?;
        Ok(())
    }

    pub fn remove_path<P: ?Sized + FanotifyPath>(&self, mode: u64, path: &P) -> Result<(), Error> {
        fanotify_mark(self.fd, FAN_MARK_REMOVE, mode, AT_FDCWD, path)?;
        Ok(())
    }

    pub fn flush_path<P: ?Sized + FanotifyPath>(&self, mode: u64, path: &P) -> Result<(), Error> {
        fanotify_mark(self.fd, FAN_MARK_FLUSH, mode, AT_FDCWD, path)?;
        Ok(())
    }

    pub fn read_event(&self) -> Vec<Event> {
        let mut result = Vec::new();
        let events = fanotify_read(self.fd);
        for metadata in events {
            let path = read_link(format!("/proc/self/fd/{}", metadata.fd)).unwrap_or_default();
            let path = path.to_str().unwrap();
            result.push(Event {
                fd: metadata.fd,
                path: String::from(path),
                events: events_from_mask(metadata.mask),
                pid: metadata.pid,
            });
            close_fd(metadata.fd);
        }
        result
    }

    pub fn send_response<T: Into<i32>>(&self, fd: T, resp: FanotifyResponse) {
        use crate::low_level::FanotifyResponse as LowLeveResponse;
        use libc::c_void;
        let response = LowLeveResponse {
            fd: fd.into(),
            response: resp.into(),
        };
        unsafe {
            libc::write(
                self.fd,
                core::ptr::addr_of!(response) as *const c_void,
                std::mem::size_of::<LowLeveResponse>(),
            );
        }
    }

    pub fn as_raw_fd(&self) -> i32 {
        self.fd
    }
}
