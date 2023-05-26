#![feature(path_file_prefix)]

pub use clap::Parser;
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

	/// Width of the space character
	#[clap(long, default_value="4")]
	pub space: u32,

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

		if img.width() % self.width != 0 {
			eprintln!("Image width must be a multiple of {}", self.width);
			exit(1);
		}

		if img.height() % self.height != 0 {
			eprintln!("Image width must be a multiple of {}", self.height);
			exit(1);
		}

		let mut glyph = ' ' as u8;

		let tile_count = (img.width() / self.width) * (img.height() / self.height);

		let input_name = self.input.file_prefix().unwrap_or_else(|| {
			eprintln!("Warning: {}'s `file` field (line 3) will need to be adjusted.", self.output.display());
			OsStr::new("font")
		});

		let mut output = String::new();

		writeln!(
			output,
			"info face=\"{}\" size={} bold=0 italic=0 unicode=1 stretchH=100 smooth=1 aa=0 padding=0,0,0,0 spacing=0,0",
			input_name.to_string_lossy(),
			self.width
		).unwrap();
		writeln!(
			output,
			"common lineHeight={} base=0 scaleW={} scaleH={} pages=1 packed=0",
			self.height,
			img.width(),
			img.height(),
		).unwrap();
		writeln!(
			output,
			"page id=0 file=\"{}\"",
			self.bitmap_location.unwrap_or(self.input).display()
		).unwrap();
		writeln!(output, "chars count={tile_count}").unwrap();

		for ty in (0..img.height()).step_by(self.height as usize) {
			for tx in (0..img.width()).step_by(self.width as usize) {
				let mut size = 0;

				for y in ty..(ty + self.height) {
					let mut row_size = 0;

					for x in tx..(tx + self.width) {
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

		match fs::write(&self.output, output) {
			Err(err) => {
				eprintln!("Failed to write to {}: {}", self.output.display(), err);
				exit(1);
			}
			_ => {}
		}
	}
}