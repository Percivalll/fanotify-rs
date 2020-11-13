use crate::low_level::*;
use crate::FanotifyPath;
use enum_iterator::IntoEnumIterator;
use std::fs::read_link;
use std::io::Error;

pub use crate::low_level::{
    FAN_ACCESS, FAN_ACCESS_PERM, FAN_ATTRIB, FAN_CLOSE, FAN_CLOSE_NOWRITE, FAN_CLOSE_WRITE,
    FAN_CREATE, FAN_DELETE, FAN_DELETE_SELF, FAN_EVENT_ON_CHILD, FAN_MODIFY, FAN_MOVE,
    FAN_MOVED_FROM, FAN_MOVED_TO, FAN_MOVE_SELF, FAN_ONDIR, FAN_OPEN, FAN_OPEN_EXEC,
    FAN_OPEN_EXEC_PERM, FAN_OPEN_PERM,
};

pub struct Fanotify {
    fd: i32,
}

impl<T> From<T> for Fanotify
where
    T: Into<i32>,
{
    fn from(raw: T) -> Fanotify {
        Fanotify { fd: raw.into() }
    }
}

#[derive(Debug, Clone, Copy, IntoEnumIterator, PartialEq)]
pub enum FanEvent {
    Access,
    AccessPerm,
    Attrib,
    Close,
    CloseNowrite,
    CloseWrite,
    Create,
    Delete,
    DeleteSelf,
    EventOnChild,
    Modify,
    Move,
    MovedFrom,
    MovedTo,
    MoveSelf,
    Ondir,
    Open,
    OpenExec,
    OpenExecPerm,
    OpenPerm,
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

#[derive(Debug)]
pub enum FanotifyResponse {
    Allow,
    Deny
}

impl From<FanotifyResponse> for u32 {
    fn from(resp: FanotifyResponse) -> u32 {
        match resp {
            FanotifyResponse::Allow => FAN_ALLOW,
            FanotifyResponse::Deny  => FAN_DENY
        }
    }
}

fn events_from_mask(mask: u64) -> Vec<FanEvent> {
    FanEvent::into_enum_iter()
        .filter(|flag| mask & *flag as u64 != 0)
        .collect::<Vec<_>>()
}

#[derive(Debug)]
pub struct Event {
    pub path: String,
    pub events: Vec<FanEvent>,
    pub pid: u32,
}

impl From<fanotify_event_metadata> for Event {
    fn from(metadata: fanotify_event_metadata) -> Self {
        let path = read_link(format!("/proc/self/fd/{}", metadata.fd)).unwrap_or_default();
        Event {
            path: path.to_str().unwrap().to_string(),
            events: events_from_mask(metadata.mask),
            pid: metadata.pid as u32,
        }
    }
}

pub enum FanotifyMode {
    PRECONTENT,
    CONTENT,
    NOTIF,
}

impl FanotifyMode {
    fn to_fan_class(&self) -> u32 {
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
            fd: fanotify_init(FAN_CLOEXEC | mode.to_fan_class(), O_CLOEXEC | O_RDONLY).unwrap(),
        }
    }

    pub fn new_with_nonblocking(mode: FanotifyMode) -> Self {
        Fanotify {
            fd: fanotify_init(
                FAN_CLOEXEC | FAN_NONBLOCK | mode.to_fan_class(),
                O_CLOEXEC | O_RDONLY,
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
        for i in events {
            let path = read_link(format!("/proc/self/fd/{}", i.fd)).unwrap_or_default();
            let path = path.to_str().unwrap();
            result.push(Event {
                path: String::from(path),
                events: events_from_mask(i.mask),
                pid: i.pid as u32,
            });
            close_fd(i.fd);
        }
        result
    }

    pub fn send_response<T: Into<i32>>(&self, fd: T, resp: FanotifyResponse) {
        use libc::c_void;
        let response = fanotify_response {
            fd: fd.into(),
            response: resp.into()
        };
        unsafe {
            libc::write(self.fd,  Box::into_raw(Box::new(response)) as *const c_void, std::mem::size_of::<fanotify_response>());
        }
    }
}
