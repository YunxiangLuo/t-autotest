mod serial;
mod ssh;
mod vnc;

pub use serial::SerialClient;
pub use ssh::{SSHClient, SSHAuthAuth};
pub use vnc::VNCClient;

pub trait FullPowerConsole: ScreenControlConsole + DuplexChannelConsole {}

pub trait ScreenControlConsole {}

pub trait DuplexChannelConsole {}

#[cfg(test)]
mod test {}

// magic string, used for regex extract in ssh or serial output
static MAGIC_STRING: &str = "n8acxy9o47xx7x7xw";

// get display string from raw xt100 output
fn get_parsed_str_from_xt100_bytes(bytes: &[u8]) -> String {
    let mut parser = vt100::Parser::new(24, 80, 0);
    parser.process(bytes);
    parser.screen().contents()
}