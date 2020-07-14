use super::color::RGBColor;
use wasm_bindgen::{JsCast, Clamped};

///
/// Canvas manipulation
///
/// Provides an abstraction for the HTML5 Canvas
///
pub struct Canvas {
    /// The width of the Canvas
    pub width: usize,
    /// The height of the Canvas
    pub height: usize,

    buffer: Vec<u8>,
    context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    /// Construct a new canvas
    ///
    /// This function creates the canvas with the size and pixels
    ///
    /// The canvas must exist in the html document
    /// If it doesn't, it will result in a RuntimeError: "unreachable executed"
    ///
    /// # Arguments
    /// * width - Canvas width
    /// * height - Canvas height
    ///
    /// Return Canvas object
    pub fn new(width: usize, height: usize) -> Canvas {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        Canvas {
            width,
            height,
            buffer: vec![0; width * height * 4],
            context,
        }
    }

    /// Put a pixel in the canvas's buffer
    ///
    /// # Arguments
    /// * x - Pixel x position
    /// * y - Pixel y position
    /// * color - Color of the pixel
    pub fn put_pixel(&mut self, x: usize, y: usize, color: RGBColor) {
        let offset = (y * self.width + x) * 4;

        self.buffer[offset] = color.r;
        self.buffer[offset + 1] = color.g;
        self.buffer[offset + 2] = color.b;
        self.buffer[offset + 3] = 255;
    }

    /// Update the whole canvas
    pub fn update(&mut self) {
        let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut self.buffer), self.width as u32, self.height as u32).unwrap();
        self.context.put_image_data(&data, 0.0, 0.0).unwrap();
    }
}