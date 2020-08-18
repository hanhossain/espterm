use anyhow::Result;
use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::io;
use std::io::Write;
use std::time::Duration;

fn main() -> Result<()> {
    let port_name = "/dev/tty.SLAB_USBtoUART";
    let baud_rate = 115200;

    let s = SerialPortSettings {
        baud_rate,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(10),
    };

    let mut port = serialport::open_with_settings(port_name, &s)?;

    let mut serial_buf: Vec<u8> = vec![0; 1000];

    loop {
        match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => io::stdout().write_all(&serial_buf[..t])?,
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
