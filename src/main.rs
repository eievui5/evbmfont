#![feature(path_file_prefix)]

use clap::Parser;
use image::GenericImageView;
use std::cmp::max;
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

// This program is a heavily modified version of `makefont` from Esprit.

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	/// Path to input image file
	input: PathBuf,

	/// Output .fnt file
	#[clap(short, long)]
	output: PathBuf,

	/// Width of each tile
	#[clap(long, default_value="8")]
	width: u32,

	/// Height of each tile
	#[clap(long, default_value="8")]
	height: u32,

	/// Width of the space character
	#[clap(long, default_value="4")]
	space: u32,

	/// The first character in the font.
	#[clap(long, default_value=" ")]
	first: char,

	#[clap(long="bitmap-location")]
	bitmap_location: Option<PathBuf>,
}

fn main() {
	let cli = Cli::parse();

	let img = image::open(&cli.input).unwrap_or_else(|error| {
		eprintln!("Failed to open image {}: {}", cli.input.display(), error);
		exit(1);
	});

	if img.width() % cli.width != 0 {
		eprintln!("Image width must be a multiple of {}", cli.width);
		exit(1);
	}

	if img.height() % cli.height != 0 {
		eprintln!("Image width must be a multiple of {}", cli.height);
		exit(1);
	}

	let mut glyph = ' ' as u8;

	let tile_count = (img.width() / cli.width) * (img.height() / cli.height);

	let input_name = cli.input.file_prefix().unwrap_or_else(|| {
		eprintln!("Warning: {}'s `file` field (line 3) will need to be adjusted.", cli.output.display());
		OsStr::new("font")
	});

	let mut output = String::new();

	writeln!(
		output,
		"info face=\"{}\" size={} bold=0 italic=0 unicode=1 stretchH=100 smooth=1 aa=0 padding=0,0,0,0 spacing=0,0",
		input_name.to_string_lossy(),
		cli.width
	).unwrap();
	writeln!(
		output,
		"common lineHeight={} base=0 scaleW={} scaleH={} pages=1 packed=0",
		cli.height,
		img.width(),
		img.height(),
	).unwrap();
	writeln!(
		output,
		"page id=0 file=\"{}\"",
		cli.bitmap_location.unwrap_or(cli.input).display()
	).unwrap();
	writeln!(output, "chars count={tile_count}").unwrap();

	for ty in (0..img.height()).step_by(cli.height as usize) {
		for tx in (0..img.width()).step_by(cli.width as usize) {
			let mut size = 0;

			for y in ty..(ty + cli.height) {
				let mut row_size = 0;

				for x in tx..(tx + cli.width) {
					let pixel = img.get_pixel(x, y);

					if pixel.0[3] != 0 {
						row_size = x - tx;
					}
				}

				size = max(size, row_size);
			}
			size += 2;
			writeln!(
				output,
				"char id={glyph} x={tx} y={ty} width={size} height=8 xoffset=0 yoffset=0 xadvance={size} page=0 chnl=0"
			).unwrap();

			glyph += 1;
		}
	}

	writeln!(output, "kernings count=0").unwrap();

	match fs::write(&cli.output, output) {
		Err(err) => {
			eprintln!("Failed to write to {}: {}", cli.output.display(), err);
			exit(1);
		}
		_ => {}
	}
}
