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
    if let Err(e) = fd.add_mountpoint(
        FAN_OPEN_EXEC_PERM | FAN_CLOSE_WRITE,
        app.value_of("path").unwrap(),
    ) {
        eprintln!("Error on add_mountpoint {}", e);
        std::process::exit(1);
    }

    let mut fds = [PollFd::new(fd.as_raw_fd(), PollFlags::POLLIN)];
    loop {
        let poll_num = poll(&mut fds, -1).unwrap();
        if poll_num > 0 {
            assert!(fds[0].revents().unwrap().contains(PollFlags::POLLIN));
            for event in fd.read_event() {
                println!("{:#?}", event);
                if event.events.contains(&FanEvent::OpenExecPerm) {
                    loop {
                        eprintln!("run?[Y/n]");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        match *input.into_bytes().first().unwrap() as char {
                            'y'|'Y' => {
                                fd.send_response(event.fd, FanotifyResponse::Allow);
                                break;
                            },
                            'n'|'N' => {
                                fd.send_response(event.fd, FanotifyResponse::Deny);
                                break;
                            },
                            _ => continue
                        }
                    }
                }
            }
        } else {
            eprintln!("poll_num <= 0");
            break;
        }
    }
}
