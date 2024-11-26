use clap::{arg, Args, Command, FromArgMatches as _};
use image::GenericImageView;
use pixelify::{cli, BoxBlur};

fn main() {
    let args = cli::Cli::init();
    let out = args.output.unwrap_or(cli::DEFAULT_OUT.into());

    let p = args.input;
    let mut img = image::open(&p).unwrap().as_mut_rgba8().unwrap();

    let out = BoxBlur::run(img);
    img.save(out).unwrap();
}
