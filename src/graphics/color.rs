use std::ops::Add;

/// Color is a simple color class composed of 4 components: Red, Green, Blue, Alpha
#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct RGBColor {
    /// The red composant of the color
    pub r: u8,
    /// The green composant of the color
    pub g: u8,
    /// The blue composant of the color
    pub b: u8,
    // The alpha composant of the color
    pub a: u8,
}

/// White predefined color
pub static WHITE: RGBColor = RGBColor { r: 255, g: 255, b: 255, a: 255 };

impl RGBColor {
    /// Construct a color from its 3 RGB components
    ///
    /// # Arguments
    /// * r - Red component
    /// * g - Green component
    /// * b - Blue component
    ///
    /// Return Color object constructed from the components
    pub fn new(r: u8, g: u8, b: u8) -> RGBColor {
        RGBColor {
            r,
            g,
            b,
            a: 255,
        }
    }

    pub fn blend(&mut self, rhs: &RGBColor) {
        self.r = self.r / 2 + rhs.r / 2;
        self.g = self.g / 2 + rhs.g / 2;
        self.b = self.b / 2 + rhs.b / 2;
        self.a = rhs.a;
    }

    pub fn from_hsl(hsl: &HSLColor) -> RGBColor {
        let c = (1.0 - (hsl.l * 2.0 - 1.0).abs()) * hsl.s;
        let x = c * (1.0 - ((hsl.h / 60.0) % 2.0 - 1.0).abs());
        let m = hsl.l - c / 2.0;

        let (r, g, b) = match hsl.h {
            h if h >= 0.0 && h < 60.0 => (c, x, 0.0),
            h if h >= 60.0 && h < 120.0 => (x, c, 0.0),
            h if h >= 120.0 && h < 180.0 => (0.0, c, x),
            h if h >= 180.0 && h < 240.0 => (0.0, x, c),
            h if h >= 240.0 && h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x)
        };

        RGBColor {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
            a: 255,
        }
    }
}

/// Color represented in HSL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HSLColor {
    /// The hue of the color, in degrees.
    pub h: f32,
    /// The colorfulness of the color. 0.0 gives gray scale colors and 1.0 will
    /// give absolutely clear colors.
    pub s: f32,
    /// The lightness of the color. 0.0 will be black, 0.5 will give
    /// a clear color, and 1.0 will give white.
    pub l: f32,
}

impl HSLColor {
    pub fn new(h: f32, s: f32, l: f32) -> HSLColor {
        HSLColor {
            h,
            s,
            l,
        }
    }

    pub fn from_rgb(rgb: &RGBColor) -> HSLColor {
        let r = rgb.r as f32 / 255.0;
        let g = rgb.g as f32 / 255.0;
        let b = rgb.b as f32 / 255.0;

        let min = r.min(g.min(b));
        let max = r.max(g.max(b));

        let mut h = 0.0;
        let mut s = 0.0;
        let l = (min + max) / 2.0;

        if min != max {
            s = if l < 0.5 {
                (max - min) / (max + min)
            } else {
                (max - min) / (2.0 - max - min)
            };

            h = if r == max {
                (g - b) / (max - min)
            } else if g == max {
                2.0 + (b - r) / (max - min)
            } else {
                4.0 + (r - g) / (max - min)
            } * 60.0;
        }

        HSLColor {
            h,
            s,
            l,
        }
    }
}

impl Default for HSLColor {
    fn default() -> HSLColor {
        HSLColor {
            h: 0.0,
            s: 0.0,
            l: 0.0,
        }
    }
}

impl Add for HSLColor {
    type Output = HSLColor;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            h: (self.h + rhs.h) % 361.0,
            s: self.s + rhs.s,
            l: self.l + rhs.l,
        }
    }
}