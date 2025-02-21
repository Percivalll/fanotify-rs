use std::os::fd::AsFd;

use fanotify::high_level::*;
use nix::poll::{poll, PollFd, PollFlags};

fn main() {
    let app = clap::Command::new("with_poll")
        .arg(clap::Arg::new("path").index(1).required(true))
        .get_matches();

    let fd = Fanotify::new_nonblocking(FanotifyMode::CONTENT);
    match fd {
        Ok(fdb) => {
            fdb.add_mountpoint(
                FAN_OPEN_EXEC | FAN_CLOSE_WRITE,
                app.get_one::<String>("path")
                    .expect("We can unwrap here as clap enforces the existence of `path`"),
            )
            .unwrap();

            let fd_handle = fdb.as_fd();
            let mut fds = [PollFd::new(&fd_handle, PollFlags::POLLIN)];
            loop {
                let poll_num = poll(&mut fds, -1).unwrap();
                if poll_num > 0 {
                    for event in fdb.read_event() {
                        println!("{:#?}", event);
                        fdb.send_response(event.fd, FanotifyResponse::Allow);
                    }
                } else {
                    eprintln!("poll_num <= 0!");
                    break;
                }
            }
        }
        Err(e) => {
            println!("encountered an error: {e}")
        }
    }
}
