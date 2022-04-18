use bytesize::ByteSize;
use eta::{Eta, TimeAcc};
use iced::{
    pure::{button, column, row, text, Element, Sandbox},
    Alignment, Settings,
};
use oxipng::{optimize_from_memory, Options};

use crate::errors::TinyError;

pub use self::errors::Result;

mod errors;

pub fn main() -> iced::Result {
    TinyPNG::run(Settings { default_font: Some(include_bytes!("SourceHanSansSC.otf")), ..Settings::default() })
}

pub struct TinyPNG {
    value: i32,
    language: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ClearAll,
    DryRun,
    DirectRun,
}

impl Sandbox for TinyPNG {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0, language: "".to_string() }
    }

    fn title(&self) -> String {
        String::from("Tiny PNG")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::DryRun => {
                self.value += 1;
            }
            Message::DirectRun => {
                self.value -= 1;
            }
            Message::ClearAll => self.value = 0,
        }
    }

    fn view(&self) -> Element<Message> {
        column()
            .padding(20)
            .align_items(Alignment::Center)
            .push(text(self.value.to_string()).size(50))
            .push(self.view_buttons())
            .into()
    }
}

impl TinyPNG {
    pub fn view_buttons(&self) -> Element<Message> {
        row() //
            .push(button("Button_ClearALL").on_press(Message::ClearAll))
            .push(button("Button_DryRun").on_press(Message::DryRun))
            .push(button("Button_DirectRun").on_press(Message::DirectRun))
            .into()
    }
}

pub struct ImageBuffer {
    output: Vec<u8>,
    before: ByteSize,
    after: ByteSize,
    reduce: f64,
}

pub fn optimize_png(png: &[u8]) -> Result<ImageBuffer> {
    let mut opts = Options { ..Options::default() };
    let image = optimize_from_memory(png, &opts)?;
    let before = ByteSize::b(png.len() as u64);
    let after = ByteSize::b(image.len() as u64);
    let reduce = calc_reduce(png, &image);
    let output = if is_fully_optimized(png.len(), image.len(), &opts) { return Err(TinyError::ImageOptimized) } else { image };
    Ok(ImageBuffer { output, before, after, reduce })
}

pub fn is_fully_optimized(original_size: usize, optimized_size: usize, opts: &Options) -> bool {
    original_size <= optimized_size && opts.interlace.is_none()
}

pub fn calc_reduce(before: &[u8], after: &[u8]) -> f64 {
    let before = before.len() as f64;
    let after = after.len() as f64;
    (before - after) / -before
}

fn calculate_square(number: usize) -> usize {
    number * number
}

#[test]
fn test() {
    let count = 100;
    let numbers = Vec::from_iter(0..count);
    let mut eta = Eta::new(count, TimeAcc::MILLI);

    for number in numbers {
        calculate_square(number);
        eta.step();
        if (number % 10) == 0 {
            println!("{}", eta);
        }
    }
}
