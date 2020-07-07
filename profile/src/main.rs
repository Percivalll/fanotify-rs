use fanotify::high_level::*;
use std::thread;
use std::fs::{File,OpenOptions};
use std::io::Write;
use std::time::{Duration, SystemTime};
fn main() {
    let ft = Fanotify::new_with_blocking(FanotifyMode::NOTIF);
    ft.add_path(
        FAN_CLOSE | FAN_EVENT_ON_CHILD | FAN_MODIFY | FAN_ONDIR | FAN_CLOSE_NOWRITE,
        "/tmp",
    )
    .unwrap();

    loop{
        ft.read_event();
    }
}
