use clap::{command, Parser};

#[derive(Parser)]
#[command(version, about)]
pub struct Arguments {
    #[arg(short, long, default_value_t = -2.0)]
    pub rotation_speed: f64,
    #[arg(short, long)]
    pub postprocesses: Vec<String>,
}
