use clap::Parser;
use pixelify::{
    cli::{Cli, DEFAULT_OUT},
    BoxBlur,
};
use std::fs;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let out = args.output.unwrap_or(DEFAULT_OUT.into());

    let buf = fs::read(args.input).unwrap();

    let new_img_buf = BoxBlur::run(&buf, args.scale)?;
    new_img_buf.save(out)?;

    Ok(())
}
