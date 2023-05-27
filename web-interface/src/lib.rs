use image::io::Reader;
use std::io::Cursor;
use std::path::PathBuf;
use evbmfont::make_fnt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn convert_image(
	img_bytes: Vec<u8>,
	image_path: String,
	width: u32,
	height: u32,
	first_char: char,
) -> String {
	let reader = match Reader::new(Cursor::new(img_bytes)).with_guessed_format() {
		Ok(reader) => reader,
		Err(err) => return format!("{err}"),
	};

	let img = match reader.decode() {
		Ok(img) => img,
		Err(err) => return format!("{err}"),
	};

	let image_path = &PathBuf::from(image_path);

	make_fnt(
		img,
		image_path,
		"fnt file",
		width,
		height,
		first_char,
		image_path,
	)
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}