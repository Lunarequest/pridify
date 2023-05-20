mod utils;

use std::convert::TryInto;

use image::{
    imageops::overlay, load_from_memory, DynamicImage, GenericImage, GenericImageView, ImageBuffer,
};
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::prelude::*;

const RAINBOW: &[u8] = include_bytes!("images/rainbow.png");

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn girls(pfp_bytes: &[u8], flag: String) -> Result<Uint8Array, JsError> {
    #[cfg(debug_assertions)]
    utils::set_panic_hook();
    let flag_bytes = match flag.as_str() {
        "rainbow" => RAINBOW,
        _ => {
            return Err(JsError::new("unsupported flag"));
        }
    };
    let pfp = load_from_memory(pfp_bytes)?;
    let flag = load_from_memory(flag_bytes)?;
    let (width_pfp, hieght_pfp) = pfp.dimensions();
    let (width_flag, hieght_flag) = flag.dimensions();

    let x = (width_flag - width_pfp) / 2;
    let y = (hieght_flag - hieght_pfp) / 2;

    let buf = ImageBuffer::new(hieght_flag, hieght_flag);

    let mut result = DynamicImage::ImageRgb8(buf);
    result.copy_from(&flag, 0, 0)?;

    overlay(&mut result, &pfp, i64::from(x), i64::from(y));

    let bytes: Vec<u8> = result.into_bytes();
    let inter = u32::from_be_bytes((&bytes[..4]).try_into().unwrap());
    let buf = ArrayBuffer::new(inter);
    let array = Uint8Array::new(&buf);

    Ok(array)
}
