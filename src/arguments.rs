use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about)]
pub struct Arguments {
    #[arg(short, long, default_value_t = -2.0)]
    /// Duck rotation speed [rad/s] (counter-clockwise)
    pub rotation_speed: f64,
    #[arg(short, long)]
    /// Post-processes that should be applied (antialias, outline)
    pub postprocesses: Vec<String>,
}
