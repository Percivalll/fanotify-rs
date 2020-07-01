use crate::low_level::*;
use std::io;
pub struct Fanotify {
    fanotify_fd: i32,
}
pub enum FanotifyMode {
    PRECONTENT,
    CONTENT,
    NOTIF,
}
pub use crate::low_level::{FAN_ACCESS,FAN_MODIFY,FAN_CLOSE_WRITE,FAN_CLOSE_NOWRITE,FAN_OPEN,FAN_OPEN_PERM,FAN_ACCESS_PERM,FAN_ONDIR,FAN_EVENT_ON_CHILD,FAN_CLOSE};
impl Fanotify {
    pub fn new_with_blocking(mode: FanotifyMode) -> Self {
        match mode {
            FanotifyMode::PRECONTENT => {
                return Fanotify {
                    fanotify_fd: fanotify_init(FAN_CLASS_PRE_CONTENT, O_RDONLY).unwrap(),
                };
            }
            FanotifyMode::CONTENT => {
                return Fanotify {
                    fanotify_fd: fanotify_init(FAN_CLASS_CONTENT, O_RDONLY).unwrap(),
                };
            }
            FanotifyMode::NOTIF => {
                return Fanotify {
                    fanotify_fd: fanotify_init(FAN_CLASS_NOTIF, O_RDONLY).unwrap(),
                };
            }
        }
    }
    pub fn add_path(&self,mode:u64,path:&'static str)->Result<(),io::Error> {
        fanotify_mark(self.fanotify_fd, FAN_MARK_ADD, mode, AT_FDCWD,path)?;
        Ok(())
    }
    pub fn remove_path(&self,mode:u64,path:&'static str)->Result<(),io::Error>{
        fanotify_mark(self.fanotify_fd, FAN_MARK_REMOVE, mode, AT_FDCWD,path)?;
        Ok(())
    }
    pub fn flush_path(&self,mode:u64,path:&'static str)->Result<(),io::Error>{
        fanotify_mark(self.fanotify_fd, FAN_MARK_FLUSH, mode, AT_FDCWD,path)?;
        Ok(())
    }
}
