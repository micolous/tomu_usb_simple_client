use std::io::{stdin, stdout, Write};

use tomu_usb_simple_client::{Colour, Error, TomuUsbSimple};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut dev = TomuUsbSimple::open().await?;
    let colours = [Colour::Green, Colour::Red, Colour::Both, Colour::Off];
    let mut stdout = stdout();

    loop {
        for colour in colours {
            write!(
                stdout,
                "Light is {colour:?}; press Enter for the next colour, or Ctrl-C to quit: "
            )
            .unwrap();
            stdout.flush().unwrap();
            dev.led(colour).await?;
            stdin().read_line(&mut String::new()).unwrap();
        }
    }
}
