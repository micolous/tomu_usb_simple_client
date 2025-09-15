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
//!
//! // Turn on the red LED.
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

/// Result type wrapper.
pub type Result<T = ()> = std::result::Result<T, Error>;

/// Tomu errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Device not found.
    #[error("Device not found")]
    DeviceNotFound,

    /// [`NUsbError`][]
    #[error("USB error: {0}")]
    NUsbError(#[from] NUsbError),

    /// [`TransferError`][]
    #[error("USB transfer error: {0}")]
    TransferError(#[from] TransferError),
}

/// Tomu `usb_simple` device connection.
pub struct TomuUsbSimple {
    #[cfg(target_os = "windows")]
    interface: Interface,

    #[cfg(not(target_os = "windows"))]
    device: Device,
}

/// Tomu LED colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum Colour {
    /// Turn off both LEDs.
    Off = 0,

    /// Turn on the green LED only.
    Green = 1,

    /// Turn on the red LED only.
    Red = 2,

    /// Turn on both the green and red LEDs.
    Both = 3,
}

impl TomuUsbSimple {
    /// Open a connection to the Tomu `usb_simple` app.
    ///
    /// See [the crate-level docs][self] for an example.
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

    /// Set the colour of the LEDs.
    ///
    /// See [the crate-level docs][self] for an example.
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
