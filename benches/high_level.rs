#![feature(test)]
extern crate test;
use test::Bencher;
#[bench]
fn high_level_profile(b: &mut Bencher) {
    use fanotify::high_level::*;
    let ft = Fanotify::new_with_blocking(FanotifyMode::NOTIF);
    ft.add_path(
        FAN_CLOSE | FAN_EVENT_ON_CHILD | FAN_MODIFY | FAN_ONDIR | FAN_CLOSE_NOWRITE,
        "/tmp",
    )
    .unwrap();
    b.iter(move ||{
        let res = ft.read_event();
        println!("{:?}",res.len());
    })
}
