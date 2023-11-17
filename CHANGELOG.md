# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/giangndm/rust-tun/compare/v0.1.0...v0.1.1) - 2023-11-17

### Other
- release ([#1](https://github.com/giangndm/rust-tun/pull/1))

## [0.1.0](https://github.com/giangndm/rust-tun/releases/tag/v0.1.0) - 2023-11-17

### Added
- add TunPacket::into_bytes()
- AsyncDevice support write_vectored asynchronously
- Allow setting non-blocking mode without enabling async
- *nix tun device supports vectored read/write
- add Android support
- add iOS support
- add multiqueue support
- add L2 devices support
- add async/tokio support

### Fixed
- use set_alias to ensure the netmask is set on macOS
- implement IntoRawFd for Queue properly
- clippy warnings
- into_raw_fd takes the first Queue's ownership
- Android supports AsyncDevice
- properly handle endianness for big-endian targets
- reserve enough space for packet decoding
- ignore malformed packets while decoding
- use correct error type on macOS
- import ioctl macros on macOS as well

### Other
- prepare for release
- dont meshup with tokio. using raw-fd instead for all other async runtime
- bump version
- bump version
- Use readv/writev instead of recvmsg/sendmsg
- PacketProtocol also supports "android"
- "async" feature deps enables for "android"
- use stable Rust in flakes
- bump version
- add nix flake
- bump version
- chore; update README
- upgrade tokio to v1.x
- update feature name in README
- bump version
- bump ioctl-sys to 0.6
- Revert "fix: ignore malformed packets while decoding"
- add iOS usage
- run `cargo fmt`
- rename {Device,Queue}Async to Async{Device,Queue}
- bump version
- update dependencies
- update to edition 2018
- bump version
- only try to enable the interface when requested
- bump version
- Implement IntoRawFd and AsRawFd for Device on MacOS
- Implement IntoRawFd for Device on Linux
- Implement IntoRawFd for Fd
- bump version
- fix import
- bump version
- add some notes about platform support
- add tests
- support macos tun api
- minor style cleanup
- fix typo
- bump version
- add configuration helper
- bump version
- drop untagged_union feature
- upgrade error-chain
- bump version
- add one
- fix license headers
- add some documentation
- fix basic example
- move configuration application to the main trait
- reuse Fd implementation for mio::event::Evented
- optionally implement mio stuff
- remove useless type parameter
- make the field public
- refactor the way interface creation is handled
- make internal modules public
- bump version
- implement IntoAddress for references too
- bump version
- add mtu()
- bump version
- refactor builder, takes a mutable reference now
- add Fd splitting
- refactor Fd handling
- add optional support for mio
- do not disable protocol headers
- bump version
- refactor everything
- add simple example
- add device enabling
- fix description
- Initial commit
