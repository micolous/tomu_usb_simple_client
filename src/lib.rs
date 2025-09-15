//! Rust client for [Tomu's][tomu] [`usb_simple` app][usb-simple].
//!
//! ## Example
//!
//! ```no_run
//! use tomu_usb_simple_client::*;
//!
//! # #[tokio::main]
//! # async fn main() -> Result {
//! let mut tomu = TomuUsbSimple::open().await?;
//! tomu.led(Colour::Red).await?;
//! # Ok(())
//! # }
//! ```
//!
//! [tomu]: https://tomu.im/
//! [usb-simple]: https://github.com/im-tomu/tomu-samples/tree/master/usb_simple
#[cfg(not(target_os = "windows"))]
use nusb::Device;
#[cfg(target_os = "windows")]
use nusb::Interface;
use nusb::{
    transfer::{ControlOut, ControlType, Recipient, TransferError},
    Error as NUsbError,
};
use std::time::Duration;
use thiserror::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Device not found")]
    DeviceNotFound,
    #[error("USB error: {0}")]
    NUsbError(#[from] NUsbError),
    #[error("USB transfer error: {0}")]
    TransferError(#[from] TransferError),
}

pub struct TomuUsbSimple {
    #[cfg(target_os = "windows")]
    interface: Interface,

    #[cfg(not(target_os = "windows"))]
    device: Device,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Colour {
    Off = 0,
    Green = 1,
    Red = 2,
    Both = 3,
}

impl TomuUsbSimple {
    pub async fn open() -> Result<Self> {
        let device = nusb::list_devices()
            .await?
            .find(|d| d.vendor_id() == 0x1209 && d.product_id() == 0x70b1)
            .ok_or(Error::DeviceNotFound)?
            .open()
            .await?;

        #[cfg(not(target_os = "windows"))]
        {
            device.set_configuration(1).await?;
            Ok(Self { device })
        }

        #[cfg(target_os = "windows")]
        {
            let interface = device.claim_interface(0).await?;
            Ok(Self { interface })
        }
    }

    const TIMEOUT: Duration = Duration::from_millis(500);

    pub async fn led(&mut self, colour: Colour) -> Result {
        let data = ControlOut {
            control_type: ControlType::Vendor,
            recipient: Recipient::Device,
            request: 0,
            value: colour as u16,
            index: 0,
            data: b"",
        };

        #[cfg(not(target_os = "windows"))]
        {
            Ok(self.device.control_out(data, Self::TIMEOUT).await?)
        }

        #[cfg(target_os = "windows")]
        {
            Ok(self.interface.control_out(data, Self::TIMEOUT).await?)
        }
    }
}
