#[test]
fn high_level_test() {
    use fanotify::high_level::MarkMode;
    use fanotify::high_level::{Fanotify, FanotifyMode};
    use std::io::{Read, Write};
    let ft =
        Fanotify::new_blocking(FanotifyMode::NOTIF).expect("Error registering fanotify listener");
    ft.add_path(
        MarkMode::Access
            | MarkMode::Close
            | MarkMode::EventOnChild
            | MarkMode::Modify
            | MarkMode::OnDirectory
            | MarkMode::Open,
        "/tmp",
    )
    .unwrap();
    let handler = std::thread::spawn(|| {
        let mut tmp = std::fs::File::create("/tmp/fanotify_test").unwrap();
        tmp.write_all(b"xxx").unwrap();
        let mut tmp = std::fs::File::open("/tmp/fanotify_test").unwrap();
        let mut res = String::new();
        tmp.read_to_string(&mut res).unwrap();
        assert_eq!(res, "xxx".to_string());
    });
    handler.join().unwrap();
}
