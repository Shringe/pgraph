use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// A simple TUI program to help guage the power costs relative to the purchase costs of running a
/// device. For example to see how long it would take for an expensive, power-saving device to
/// break even over a cheap, high-power device.
pub struct Args {
    /// Avoid randomizing the colors of devices
    #[arg(long)]
    pub no_color_devices: bool,
}
