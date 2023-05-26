#![feature(path_file_prefix)]

pub use clap::Parser;
use image::{GenericImageView, Rgba};
use std::cmp::max;
use std::ffi::OsStr;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;

// This program is a heavily modified version of `makefont` from Esprit.

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
	/// Path to input image file
	pub input: PathBuf,

	/// Output .fnt file
	#[clap(short, long)]
	pub output: PathBuf,

	/// Width of each tile
	#[clap(long, default_value="8")]
	pub width: u32,

	/// Height of each tile
	#[clap(long, default_value="8")]
	pub height: u32,

	/// The first character in the font.
	#[clap(long, default_value=" ")]
	pub first: char,

	#[clap(long="bitmap-location")]
	pub bitmap_location: Option<PathBuf>,
}

impl Cli {
	pub fn run(self) {
		let img = image::open(&self.input).unwrap_or_else(|error| {
			eprintln!("Failed to open image {}: {}", self.input.display(), error);
			exit(1);
		});

		let bm_location = self.bitmap_location.as_ref().unwrap_or(&self.input);

		let output = make_fnt(
			img,
			&self.input,
			&self.output.to_string_lossy(),
			self.width,
			self.height,
			self.first,
			bm_location,
		);

		match fs::write(&self.output, output) {
			Err(err) => {
				eprintln!("Failed to write to {}: {}", self.output.display(), err);
				exit(1);
			}
			_ => {}
		}
	}
}

pub fn make_fnt(
	img: impl GenericImageView<Pixel = Rgba<u8>>,
	img_path: &Path,
	output_name: &str,
	width: u32,
	height: u32,
	first: char,
	bitmap_location: &PathBuf,
) -> String {
	if img.width() % width != 0 {
		eprintln!("Image width must be a multiple of {}", width);
		exit(1);
	}

	if img.height() % height != 0 {
		eprintln!("Image width must be a multiple of {}", height);
		exit(1);
	}

	let mut glyph = first as u8;

	let tile_count = (img.width() / width) * (img.height() / height);

	let input_name = img_path.file_prefix().unwrap_or_else(|| {
		eprintln!("Warning: {}'s `file` field (line 3) will need to be adjusted.", output_name);
		OsStr::new("font")
	});

	let mut output = String::new();

	writeln!(
		output,
		"info face=\"{}\" size={} bold=0 italic=0 unicode=1 stretchH=100 smooth=1 aa=0 padding=0,0,0,0 spacing=0,0",
		input_name.to_string_lossy(),
		width
	).unwrap();
	writeln!(
		output,
		"common lineHeight={} base=0 scaleW={} scaleH={} pages=1 packed=0",
		height,
		img.width(),
		img.height(),
	).unwrap();
	writeln!(
		output,
		"page id=0 file=\"{}\"",
		bitmap_location.display()
	).unwrap();
	writeln!(output, "chars count={tile_count}").unwrap();

	for ty in (0..img.height()).step_by(height as usize) {
		for tx in (0..img.width()).step_by(width as usize) {
			let mut size = 0;

			for y in ty..(ty + height) {
				let mut row_size = 0;

				for x in tx..(tx + width) {
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
	output
}
