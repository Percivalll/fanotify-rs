use fanotify::high_level::*;
use nix::poll::{poll, PollFd, PollFlags};
use std::os::fd::AsFd;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = clap::Command::new("with_poll")
        .arg(clap::Arg::new("path").index(1).required(true))
        .get_matches();

    let fd = Fanotify::new_nonblocking(FanotifyMode::CONTENT)?;
    fd.add_mountpoint(
        MarkMode::OpenExec | MarkMode::CloseWrite,
        app.get_one::<String>("path")
            .expect("We can unwrap here as clap enforces the existence of `path`"),
    )?;

    let fd_handle = fd.as_fd();
    let mut fds = [PollFd::new(fd_handle, PollFlags::POLLIN)];
    loop {
        let poll_num = poll(&mut fds, None::<u8>).unwrap();
        if poll_num > 0 {
            for event in fd.read_event()? {
                println!("{:#?}", event);
                fd.send_response(event.fd, FanotifyResponse::Allow);
            }
        } else {
            eprintln!("poll_num <= 0!");
            break;
        }
    }

    Ok(())
}
