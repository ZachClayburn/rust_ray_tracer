mod geometry;
mod renderer;

use std::{fs::File, path::PathBuf};

use clap::{IntoApp, Parser};

#[derive(Parser)]
#[clap(author, version)]
struct Cli {
    #[clap(help = "The file for the iamge to be written to")]
    #[clap(parse(try_from_str=valid_file))]
    file_name: PathBuf,

    #[clap(short = 'W')]
    #[clap(default_value_t = 400)]
    #[clap(help = "Width of the rendered image")]
    image_width: u32,

    #[clap(short = 'H')]
    #[clap(default_value_t = 225)]
    #[clap(help = "Height of the rendered image")]
    image_height: u32,

    #[clap(short = 's')]
    #[clap(default_value_t = 100)]
    #[clap(help = "Number of samples taken for each pixel")]
    samples_per_pixel: usize,

    #[clap(short = 'r')]
    #[clap(default_value_t = 50)]
    #[clap(help = "Maximum reflections per ray")]
    max_recursion_depth: usize,

    #[clap(short = 'F', long)]
    #[clap(help = "Force overwrite of existing file")]
    force: bool,
}

fn valid_file(s: &str) -> Result<PathBuf, String> {
    let file = PathBuf::from(s);
    match file.extension() {
        None => Err(format!("{} has no extension!", s)),
        Some(os_str) => match os_str.to_str() {
            Some(ext) if ext.to_lowercase() == "jpg" => Ok(file),
            Some(ext) if ext.to_lowercase() == "png" => Ok(file),
            Some(ext) if ext.to_lowercase() == "ico" => Ok(file),
            Some(ext) if ext.to_lowercase() == "pnm" => Ok(file),
            Some(ext) if ext.to_lowercase() == "bmp" => Ok(file),
            Some(ext) if ext.to_lowercase() == "tiff" => Ok(file),
            Some(ext) => Err(format!("The {} is not supported!", ext)),
            None => Err(format!("Could not convert {:?} to a string!?", os_str)),
        },
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.file_name.exists() && !cli.force {
        let mut cmd = Cli::command();
        cmd.error(
            clap::ErrorKind::ValueValidation,
            format!(
                "{:?} already exists! To force overwrite, use the --force flag",
                cli.file_name
            ),
        )
        .exit();
    }

    if let Err(err) = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&cli.file_name)
    {
        let mut cmd = Cli::command();
        cmd.error(
            clap::ErrorKind::ValueValidation,
            format!("Cannot open {:?} for writing: {}", cli.file_name, err),
        )
        .exit();
    }

    let imgbuf = renderer::Renderer::new()
        .image_width(cli.image_width)
        .image_height(cli.image_height)
        .samples_per_pixle(cli.samples_per_pixel)
        .max_depth(cli.max_recursion_depth)
        .render();
    imgbuf.save(cli.file_name).unwrap();
}
