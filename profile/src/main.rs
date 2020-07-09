use std::fs::File;
use std::time::{SystemTime,Duration};
use std::thread;
use fanotify::high_level::*;
fn monitor(){
    let fty = Fanotify::new_with_blocking(FanotifyMode::NOTIF);
    fty.add_path(FAN_CLOSE_WRITE | FAN_EVENT_ON_CHILD| FAN_ONDIR,"/tmp");
    loop{
        for i in fty.read_event(){
            println!("{:?}",i);
        }
    }
}
fn main() {
    let heldler=thread::spawn(||monitor());
    let sys_time = SystemTime::now();
    for i in 0..1000000 {
        File::create("/tmp/".to_string() + &i.to_string());
    }
    let difference = sys_time
        .elapsed().unwrap();
    println!("QPS:{:?}", 1000000/difference.as_secs());
}
