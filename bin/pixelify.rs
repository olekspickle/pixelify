use pixelify::{
    cli::{Cli, DEFAULT_OUT},
    BoxBlur,
};

fn main() {
    let args = Cli::init();
    let out = args.output.unwrap_or(DEFAULT_OUT.into());

    let p = args.input;
    let mut img = image::open(&p).unwrap();

    if let Some(img_buf) = img.as_mut_rgba8() {
        let new_img_buf = BoxBlur::run(img_buf, args.scale);
        new_img_buf.save(out).unwrap();
    } else {
        println!("Could not convert to RGBA");
    }

    //if let Some(img_buf) = img.as_mut_rgb8() {
    //    let new_img_buf = BoxBlur::run(img_buf, args.scale);
    //    new_img_buf.save(out).unwrap();
    //}
}
