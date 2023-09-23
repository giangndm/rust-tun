//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::os::fd::{AsRawFd, FromRawFd};
use async_std::fs::File;
use futures::{AsyncReadExt, AsyncWriteExt};

#[async_std::main]
async fn main() {
    let mut config = tun::Configuration::default();

    config
        .address((10, 0, 0, 2))
        .netmask((255, 255, 255, 0))    
        .mtu(1200)
        .up();

    #[cfg(target_os = "linux")]
    config.platform(|config| {
        config.packet_information(true);
    });

    let dev = tun::create(&config).unwrap();
    let mut async_file = unsafe {
        File::from_raw_fd(dev.as_raw_fd())   
    };
    let mut buf = [0; 4096];

    loop {
        let amount = async_file.read(&mut buf).await.unwrap();
        println!("Read {}", amount);
        match async_file.write(&buf[0..amount]).await {
            Ok(amount) => {
                println!("Write {}", amount);
            }
            Err(e) => {
                println!("Error {:?}", e);
            }
        }
    }
}
