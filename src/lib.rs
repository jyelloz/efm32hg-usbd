//! usb-device implementation for EFM32HG chip.

#![no_std]


mod peripheral;

/// Re-export of the EFM32HG PAC with additional metadata exposing the the
/// Synopsys DWC core memory address.
mod pac {
    pub use efm32hg_pac::*;
    pub trait SynopsysUSB {
        const OTG_BASE: *const ();
    }
    impl SynopsysUSB for USB {
        const OTG_BASE: *const () = (0x400c_4000 + 0x0003_c000) as *const _;
    }
}

pub use peripheral::{
    USB,
    USBCore,
    UsbBusType,
    UsbPeripheral,
};
