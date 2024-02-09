use std::fs::File;
use std::time::SystemTime;
use std::thread;
use fanotify::high_level::*;

fn monitor() {
    let fty = Fanotify::new_with_blocking(FanotifyMode::NOTIF);
    let _ = fty.add_path(FAN_CLOSE_WRITE | FAN_EVENT_ON_CHILD | FAN_ONDIR, "/tmp");
    loop {
        let _ = fty.read_event();
    }
}

fn main() {
    let _thread_handle = thread::spawn(|| monitor());
    let start_time = SystemTime::now();
    for i in 0..1000000 {
        let _ = File::create(format!("/tmp/{}", i));
    }
    let duration = start_time
        .elapsed().unwrap();
    println!("QPS:{:?}", 1000000 / duration.as_secs());
}