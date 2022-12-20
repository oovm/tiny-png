#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn main() -> diagnostic_quick::QResult {
    // Image loading/saving is outside scope of this library
    let width = 10;
    let height = 10;
    let fakebitmap = vec![imagequant::RGBA {r:100, g:200, b:250, a:255}; width * height];

    // Configure the library
    let mut liq = imagequant::new();
    liq.set_speed(1)?;
    liq.set_quality(90, 100)?;

    // Describe the bitmap
    let mut img = liq.new_image(&fakebitmap[..], width, height, 0.0)?;
    // The magic happens in quantize()
    let mut res = liq.quantize(&mut img)?;
    // Enable dithering for subsequent remappings
    res.set_dithering_level(1.0)?;
    // You can reuse the result to generate several images with the same palette
    let (palette, pixels) = res.remapped(&mut img)?;
    println!("Done! Got palette {palette:?} and {} pixels with {}% quality", pixels.len(), res.quantization_quality().unwrap());
    Ok(())
}