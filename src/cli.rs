use clap::{arg, crate_authors, crate_version, Parser};
use std::path::PathBuf;

pub const DEFAULT_OUT: &str = "out.png";
const ABOUT: &str = "Image downsampler, pixelifies all your images!";
const LONG: &str = "Pixelifier uses simple box blur algorhytm to pixelify the image \
                In the future Lanczos will be available as well";

#[derive(Parser, Debug)]
#[command(name = "pixelify")]
#[command(bin_name = "pixelify")]
#[command(version = crate_version!(),  author = crate_authors!("\n"), about = ABOUT, long_about = LONG)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    pub input: PathBuf,
    #[arg(short, long, default_value = DEFAULT_OUT)]
    pub output: Option<PathBuf>,
    #[arg(short, long, default_value = "3")]
    pub scale: u32,
}
