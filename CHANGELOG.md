
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).
 
## [0.3.1-rc1] - 2024-02-04
 
Big update and refactor. The overral structure remains the same, but includes a number of fixes included in outstanding PRs. The update have been checked against current test cases and the PoC, but need review before release.
 
### Added
Implemented `AsFd` for `Fanotify` to allow borrowing of the internal file descriptor (e.g. for polling)
 
### Changed
Widened the implementation of `FanotifyPath` to all implementors of `AsRef<OsStr>`, but removed the direct implementation for `String` due to conflict.
Update dependencies, and removed the dependency on lazy_static.
Updated the crate to 2021 edition.
Changed type of PID in the `Event` type as `pid_t` is generally implemented as `int` in libc implementations.
Changed `to_fan_class` to to copy instead of borrow as the type is `Copy`.
Renamed `low_level::fanotify_response` to `low_level::FanotifyResponse` to keep with Rust's naming convention.
Removed crate definitions of some library flags, and replaced with re-exports from `libc`.
Removed setting that forced inclusion of debug symbols in release mode.
 
### Fixed
Fixed the type for calling `fanotify_mark` on aarch64