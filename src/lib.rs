//! # Wasapi bindings for Rust
//!
//! The aim of this create is to provide easy and safe access to the Wasapi API for audio playback and capture.
//!
//! Most things map closely to something in the Windows API.
//!
//! For details on how to use Wasapi, please see ["the Windows documentation"](https://docs.microsoft.com/en-us/windows/win32/coreaudio/core-audio-interfaces).
//!
//! Both shared and exclusive modes are supported. 
//!
//! Bindings are generated automatically using the [windows](https://crates.io/crates/windows) crate.
//! 
//! The `loopback` example shows how to simultaneously capture and render sound, with separate threads for capture and render.

::windows::include_bindings!();
use Windows::Win32::System::PropertiesSystem::PROPERTYKEY;
mod api;
pub use api::*;

#[macro_use]
extern crate log;

#[allow(non_upper_case_globals)]
pub const PKEY_Device_FriendlyName: PROPERTYKEY = PROPERTYKEY {
    fmtid: windows::Guid::from_values(
        0xA45C254E,
        0xDF1C,
        0x4EFD,
        [0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0],
    ),
    pid: 14,
};

#[allow(non_upper_case_globals)]
pub const PKEY_Device_DeviceDesc: PROPERTYKEY = PROPERTYKEY {
    fmtid: windows::Guid::from_values(
        0xA45C254E,
        0xDF1C,
        0x4EFD,
        [0x80, 0x20, 0x67, 0xD1, 0x46, 0xA8, 0x50, 0xE0],
    ),
    pid: 2,
};


