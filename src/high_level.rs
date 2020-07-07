use crate::low_level::*;
use lazy_static::lazy_static;
use std::fs::read_link;
use std::io::Error;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
pub struct Fanotify {
    fd: i32,
}
#[derive(Debug)]
pub struct Event {
    pub path: String,
    pub genre: String,
    pub pid: u32,
}
pub enum FanotifyMode {
    PRECONTENT,
    CONTENT,
    NOTIF,
}
pub use crate::low_level::{
    FAN_ACCESS, FAN_ACCESS_PERM, FAN_CLOSE, FAN_CLOSE_NOWRITE, FAN_CLOSE_WRITE, FAN_EVENT_ON_CHILD,
    FAN_MODIFY, FAN_ONDIR, FAN_OPEN, FAN_OPEN_PERM,
};
lazy_static! {
    static ref DELAY: Mutex<u32> = Mutex::new(0);
}
impl Fanotify {
    pub fn new_with_blocking(mode: FanotifyMode) -> Self {
        match mode {
            FanotifyMode::PRECONTENT => {
                return Fanotify {
                    fd: fanotify_init(FAN_CLOEXEC|FAN_CLASS_PRE_CONTENT, O_CLOEXEC|O_RDONLY).unwrap(),
                };
            }
            FanotifyMode::CONTENT => {
                return Fanotify {
                    fd: fanotify_init(FAN_CLOEXEC|FAN_CLASS_PRE_CONTENT, O_CLOEXEC|O_RDONLY).unwrap(),
                };
            }
            FanotifyMode::NOTIF => {
                return Fanotify {
                    fd: fanotify_init(FAN_CLOEXEC|FAN_CLASS_NOTIF, O_CLOEXEC|O_RDONLY).unwrap(),
                };
            }
        }
    }
    pub fn add_path(&self, mode: u64, path: &'static str) -> Result<(), Error> {
        fanotify_mark(self.fd, FAN_MARK_ADD, mode, AT_FDCWD, path)?;
        Ok(())
    }
    pub fn remove_path(&self, mode: u64, path: &'static str) -> Result<(), Error> {
        fanotify_mark(self.fd, FAN_MARK_REMOVE, mode, AT_FDCWD, path)?;
        Ok(())
    }
    pub fn flush_path(&self, mode: u64, path: &'static str) -> Result<(), Error> {
        fanotify_mark(self.fd, FAN_MARK_FLUSH, mode, AT_FDCWD, path)?;
        Ok(())
    }
    pub fn read_event(&self) -> Vec<Event> {
        let mut delay = *DELAY.lock().unwrap();
        thread::sleep_ms(delay.clone());
        let mut result = Vec::new();
        let events = fanotify_read(self.fd);
        for i in events {
            let path = read_link(format!("/proc/self/fd/{}", i.fd)).unwrap_or_default();
            let path = path.to_str().unwrap();
            let mut genre = "";
            if i.mask & FAN_ACCESS != 0 {
                genre = "FAN_ACCESS"
            } else if i.mask & FAN_ACCESS_PERM != 0 {
                genre = "FAN_ACCESS_PERM"
            } else if i.mask & FAN_CLOSE_NOWRITE != 0 {
                genre = "FAN_CLOSE_NOWRITE"
            } else if i.mask & FAN_CLOSE_WRITE != 0 {
                genre = "FAN_CLOSE_WRITE"
            } else if i.mask & FAN_OPEN != 0 {
                genre = "FAN_OPEN"
            } else if i.mask & FAN_OPEN_PERM != 0 {
                genre = "FAN_OPEN_PERM"
            } else if i.mask & FAN_MODIFY != 0 {
                genre = "FAN_MODIFY"
            }
            result.push(Event {
                path: String::from(path),
                genre: String::from(genre),
                pid: i.pid as u32,
            });
            close_fd(i.fd);
        }
        if *DELAY.lock().unwrap() <= 256 && result.len() < 128 {
            *DELAY.lock().unwrap() += 1;
        } else if result.len() > 128 {
            *DELAY.lock().unwrap() -= 1;
        }
        result
    }
}
