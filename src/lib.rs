pub mod high_level;
pub mod low_level;

pub trait FanotifyPath {
    fn as_os_str(&self) -> &std::ffi::OsStr;
}

impl FanotifyPath for std::path::Path {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        self.as_os_str()
    }
}

impl FanotifyPath for str {
    fn as_os_str(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new(self)
    }
}
