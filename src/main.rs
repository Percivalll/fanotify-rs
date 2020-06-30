mod warppers;
// pub unsafe fn test() {
//     let fanotify_fd = libc::fanotify_init(libc::FAN_CLASS_NOTIF, 0);
//     println!("{:?}", io::Error::last_os_error());
//     let f = libc::fanotify_mark(
//         fanotify_fd,
//         libc::FAN_MARK_ADD,
//         libc::FAN_OPEN | libc::FAN_CLOSE,
//         libc::AT_FDCWD,
//         ffi::CString::new("/home/zhanglei.sec/test")
//             .unwrap()
//             .as_ptr(),
//     );
//     println!("{:?}", io::Error::last_os_error());
//         loop {
//         let ptr = libc::malloc(24);
//         let len = libc::read(fanotify_fd, ptr, FAN_EVENT_METADATA_LEN());
//         println!("{:?}",len);
//         println!("{:?}",*(ptr as *const fanotify_event_metadata));
//     }
// }
fn main() {
}
