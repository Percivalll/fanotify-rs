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

// SAFETY: the `fanotify_*` functions are thread safe, and file descriptors are safe for
// sharing betweent threads if they are used in threadsafe functions
unsafe impl Send for Fanotify {}
unsafe impl Sync for Fanotify {}

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

impl Event {
    /// Tries to duplicate the event, including duplicating the file descriptor
    /// so new instances can keep it open after the original calls `Drop::drop`.
    pub fn try_clone(&self) -> Result<Self, std::io::Error> {
        let new_fd = unsafe { libc::dup(self.fd) };
        if new_fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(Self {
            fd: new_fd,
            path: self.path.clone(),
            events: self.events.clone(),
            pid: self.pid,
        })
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        close_fd(self.fd);
    }
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
    pub fn new_blocking(mode: FanotifyMode) -> Result<Self, Error> {
        Ok(Fanotify {
            fd: fanotify_init(
                FAN_CLOEXEC | mode.to_fan_class(),
                (O_CLOEXEC | O_RDONLY) as u32,
            )?,
        })
    }

    pub fn new_nonblocking(mode: FanotifyMode) -> Result<Self, Error> {
        Ok(Fanotify {
            fd: fanotify_init(
                FAN_CLOEXEC | FAN_NONBLOCK | mode.to_fan_class(),
                (O_CLOEXEC | O_RDONLY) as u32,
            )?,
        })
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

    pub fn read_event(&self) -> std::io::Result<Vec<Event>> {
        let mut result = Vec::new();
        let events = fanotify_read(self.fd)?;
        for metadata in events {
            let path = read_link(format!("/proc/self/fd/{}", metadata.fd)).unwrap_or_default();
            let path = path.to_str().unwrap();
            result.push(Event {
                fd: metadata.fd,
                path: String::from(path),
                events: events_from_mask(metadata.mask),
                pid: metadata.pid,
            });
        }
        Ok(result)
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

    pub fn close(self) {
        close_fd(self.fd)
    }
}

impl Drop for Fanotify {
    fn drop(&mut self) {
        close_fd(self.fd);
    }
}

impl Clone for Fanotify {
    fn clone(&self) -> Self {
        Self {
            fd: unsafe { libc::dup(self.fd) },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FanotifyBuilder {
    class: FanotifyMode,
    flags: u32,
    event_flags: u32,
}

impl FanotifyBuilder {
    pub fn new() -> Self {
        Self {
            class: FanotifyMode::NOTIF,
            flags: FAN_CLOEXEC,
            event_flags: O_CLOEXEC as u32,
        }
    }

    pub fn with_class(self, class: FanotifyMode) -> Self {
        Self { class, ..self }
    }

    pub fn with_flags(self, flags: u32) -> Self {
        Self {
            flags: FAN_CLOEXEC | flags,
            ..self
        }
    }

    pub fn with_event_flags(self, event_flags: u32) -> Self {
        Self {
            event_flags,
            ..self
        }
    }

    pub fn register(&self) -> Result<Fanotify, Error> {
        Ok(Fanotify {
            fd: fanotify_init(self.flags | self.class.to_fan_class(), self.event_flags)?,
        })
    }
}
