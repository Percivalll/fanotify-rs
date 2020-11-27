#[macro_use]
extern crate clap;
extern crate fanotify;
extern crate nix;

use fanotify::high_level::*;
use nix::poll::{poll, PollFd, PollFlags};

fn main() {
    let app = clap_app!(fanotify_demo =>
        (version:       crate_version!())
        (author:        crate_authors!())
        (about:         crate_description!())
        (@arg path: +required "watch target mount point")
    )
    .get_matches();

    let fd = Fanotify::new_with_nonblocking(FanotifyMode::CONTENT);
    fd.add_mountpoint(FAN_OPEN_EXEC | FAN_CLOSE_WRITE, app.value_of("path").unwrap()).unwrap();

    let mut fds = [PollFd::new(fd.as_raw_fd(), PollFlags::POLLIN)];
    loop {
        let poll_num = poll(&mut fds, -1).unwrap();
        if poll_num > 0 {
            for event in fd.read_event() {
                println!("{:#?}", event);
            }
        } else {
            eprintln!("poll_num <= 0!");
            break;
        }
    }
}
