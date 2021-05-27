#![allow(clippy::many_single_char_names)]

#[cfg(feature = "rust-rgb")]
use rgb::{RGB, RGBA};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{parse, ParseError};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// The color
pub struct Color {
    /// Red
    pub r: f64,
    /// Green
    pub g: f64,
    /// Blue
    pub b: f64,
    /// Alpha
    pub a: f64,
}

impl Color {
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b, a: 1. }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color { r, g, b, a }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    pub fn from_rgb_u8(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r as f64 / 255.,
            g: g as f64 / 255.,
            b: b as f64 / 255.,
            a: 1.,
        }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r as f64 / 255.,
            g: g as f64 / 255.,
            b: b as f64 / 255.,
            a: a as f64 / 255.,
        }
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    pub fn from_linear_rgb(r: f64, g: f64, b: f64) -> Color {
        Color::from_linear_rgba(r, g, b, 1.)
    }

    #[deprecated]
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    pub fn from_lrgb(r: f64, g: f64, b: f64) -> Color {
        Color::from_linear_rgba(r, g, b, 1.)
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_linear_rgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        fn from_linear(x: f64) -> f64 {
            if x >= 0.0031308 {
                return 1.055 * x.powf(1. / 2.4) - 0.055;
            }
            12.92 * x
        }
        Color::from_rgba(from_linear(r), from_linear(g), from_linear(b), a)
    }

    #[deprecated]
    /// Arguments:
    ///
    /// * `r`: Red value [0..1]
    /// * `g`: Green value [0..1]
    /// * `b`: Blue value [0..1]
    /// * `a`: Alpha value [0..1]
    pub fn from_lrgba(r: f64, g: f64, b: f64, a: f64) -> Color {
        Color::from_linear_rgba(r, g, b, a)
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    pub fn from_linear_rgb_u8(r: u8, g: u8, b: u8) -> Color {
        Color::from_linear_rgba(r as f64 / 255., g as f64 / 255., b as f64 / 255., 1.)
    }

    #[deprecated]
    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    pub fn from_lrgb_u8(r: u8, g: u8, b: u8) -> Color {
        Color::from_linear_rgba(r as f64 / 255., g as f64 / 255., b as f64 / 255., 1.)
    }

    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_linear_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::from_linear_rgba(
            r as f64 / 255.,
            g as f64 / 255.,
            b as f64 / 255.,
            a as f64 / 255.,
        )
    }

    #[deprecated]
    /// Arguments:
    ///
    /// * `r`: Red value [0..255]
    /// * `g`: Green value [0..255]
    /// * `b`: Blue value [0..255]
    /// * `a`: Alpha value [0..255]
    pub fn from_lrgba_u8(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::from_linear_rgba(
            r as f64 / 255.,
            g as f64 / 255.,
            b as f64 / 255.,
            a as f64 / 255.,
        )
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    pub fn from_hsv(h: f64, s: f64, v: f64) -> Color {
        Color::from_hsva(h, s, v, 1.)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hsva(h: f64, s: f64, v: f64, a: f64) -> Color {
        let (r, g, b) = hsv_to_rgb(normalize_angle(h), clamp0_1(s), clamp0_1(v));
        Color::from_rgba(clamp0_1(r), clamp0_1(g), clamp0_1(b), clamp0_1(a))
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    pub fn from_hsl(h: f64, s: f64, l: f64) -> Color {
        Color::from_hsla(h, s, l, 1.)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hsla(h: f64, s: f64, l: f64, a: f64) -> Color {
        let (r, g, b) = hsl_to_rgb(normalize_angle(h), clamp0_1(s), clamp0_1(l));
        Color::from_rgba(clamp0_1(r), clamp0_1(g), clamp0_1(b), clamp0_1(a))
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    pub fn from_hwb(h: f64, w: f64, b: f64) -> Color {
        Color::from_hwba(h, w, b, 1.)
    }

    /// Arguments:
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn from_hwba(h: f64, w: f64, b: f64, a: f64) -> Color {
        let (r, g, b) = hwb_to_rgb(normalize_angle(h), clamp0_1(w), clamp0_1(b));
        Color::from_rgba(clamp0_1(r), clamp0_1(g), clamp0_1(b), a)
    }

    /// Arguments:
    ///
    /// * `l`: Perceived lightness
    /// * `a`: How green/red the color is
    /// * `b`: How blue/yellow the color is
    pub fn from_oklab(l: f64, a: f64, b: f64) -> Color {
        Color::from_oklaba(l, a, b, 1.)
    }

    /// Arguments:
    ///
    /// * `l`: Perceived lightness
    /// * `a`: How green/red the color is
    /// * `b`: How blue/yellow the color is
    /// * `alpha`: Alpha [0..1]
    pub fn from_oklaba(l: f64, a: f64, b: f64, alpha: f64) -> Color {
        let l_ = (l + 0.3963377774 * a + 0.2158037573 * b).powi(3);
        let m_ = (l - 0.1055613458 * a - 0.0638541728 * b).powi(3);
        let s_ = (l - 0.0894841775 * a - 1.2914855480 * b).powi(3);

        let r = 4.0767245293 * l_ - 3.3072168827 * m_ + 0.2307590544 * s_;
        let g = -1.2681437731 * l_ + 2.6093323231 * m_ - 0.3411344290 * s_;
        let b = -0.0041119885 * l_ - 0.7034763098 * m_ + 1.7068625689 * s_;

        Color::from_linear_rgba(r, g, b, alpha)
    }

    /// Create color from CSS color string.
    ///
    /// # Examples
    /// ```
    /// use csscolorparser::Color;
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///
    /// let c = Color::from_html("rgb(255,0,0)")?;
    ///
    /// assert_eq!(c.rgba(), (1., 0., 0., 1.));
    /// assert_eq!(c.rgba_u8(), (255, 0, 0, 255));
    /// assert_eq!(c.to_hex_string(), "#ff0000");
    /// assert_eq!(c.to_rgb_string(), "rgb(255,0,0)");
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_html<S: AsRef<str>>(s: S) -> Result<Color, ParseError> {
        parse(s)
    }

    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn rgba(&self) -> (f64, f64, f64, f64) {
        (self.r, self.g, self.b, self.a)
    }

    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..255]
    pub fn rgba_u8(&self) -> (u8, u8, u8, u8) {
        (
            (self.r * 255.).round() as u8,
            (self.g * 255.).round() as u8,
            (self.b * 255.).round() as u8,
            (self.a * 255.).round() as u8,
        )
    }

    #[deprecated]
    /// Get the red value [0..1].
    pub fn red(&self) -> f64 {
        self.r
    }

    #[deprecated]
    /// Get the green value [0..1].
    pub fn green(&self) -> f64 {
        self.g
    }

    #[deprecated]
    /// Get the blue value [0..1].
    pub fn blue(&self) -> f64 {
        self.b
    }

    #[deprecated]
    /// Get the alpha value [0..1].
    pub fn alpha(&self) -> f64 {
        self.a
    }

    /// Returns: `(h, s, v, a)`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `v`: Value [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsva(&self) -> (f64, f64, f64, f64) {
        let (h, s, v) = rgb_to_hsv(self.r, self.g, self.b);
        (h, s, v, self.a)
    }

    /// Returns: `(h, s, l, a)`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `s`: Saturation [0..1]
    /// * `l`: Lightness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hsla(&self) -> (f64, f64, f64, f64) {
        let (h, s, l) = rgb_to_hsl(self.r, self.g, self.b);
        (h, s, l, self.a)
    }

    /// Returns: `(h, w, b, a)`
    ///
    /// * `h`: Hue angle [0..360]
    /// * `w`: Whiteness [0..1]
    /// * `b`: Blackness [0..1]
    /// * `a`: Alpha [0..1]
    pub fn to_hwba(&self) -> (f64, f64, f64, f64) {
        let (h, w, b) = rgb_to_hwb(self.r, self.g, self.b);
        (h, w, b, self.a)
    }

    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn to_linear_rgba(&self) -> (f64, f64, f64, f64) {
        fn to_linear(x: f64) -> f64 {
            if x >= 0.04045 {
                return ((x + 0.055) / 1.055).powf(2.4);
            }
            x / 12.92
        }
        (
            to_linear(self.r),
            to_linear(self.g),
            to_linear(self.b),
            self.a,
        )
    }

    #[deprecated]
    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..1]
    pub fn to_lrgba(&self) -> (f64, f64, f64, f64) {
        self.to_linear_rgba()
    }

    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..255]
    pub fn to_linear_rgba_u8(&self) -> (u8, u8, u8, u8) {
        let (r, g, b, a) = self.to_linear_rgba();
        (
            (r * 255.).round() as u8,
            (g * 255.).round() as u8,
            (b * 255.).round() as u8,
            (a * 255.).round() as u8,
        )
    }

    #[deprecated]
    /// Returns: `(r, g, b, a)`
    ///
    /// * Red, green, blue and alpha in the range [0..255]
    pub fn to_lrgba_u8(&self) -> (u8, u8, u8, u8) {
        self.to_linear_rgba_u8()
    }

    /// Returns: `(l, a, b, alpha)`
    pub fn to_oklaba(&self) -> (f64, f64, f64, f64) {
        let (r, g, b, _) = self.to_linear_rgba();
        let l_ = (0.4121656120 * r + 0.5362752080 * g + 0.0514575653 * b).cbrt();
        let m_ = (0.2118591070 * r + 0.6807189584 * g + 0.1074065790 * b).cbrt();
        let s_ = (0.0883097947 * r + 0.2818474174 * g + 0.6302613616 * b).cbrt();
        let l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
        let a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
        let b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;
        (l, a, b, self.a)
    }

    /// Get the RGB hexadecimal color string.
    pub fn to_hex_string(&self) -> String {
        let (r, g, b, a) = self.rgba_u8();

        if a < 255 {
            return format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a);
        }

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Get the CSS `rgb()` format string.
    pub fn to_rgb_string(&self) -> String {
        let (r, g, b, _) = self.rgba_u8();

        if self.a < 1. {
            return format!("rgba({},{},{},{})", r, g, b, self.a);
        }

        format!("rgb({},{},{})", r, g, b)
    }

    /// Blend this color with the other one, in the RGB color-space. `t` in the range [0..1].
    pub fn interpolate_rgb(&self, other: &Color, t: f64) -> Color {
        Color {
            r: self.r + t * (other.r - self.r),
            g: self.g + t * (other.g - self.g),
            b: self.b + t * (other.b - self.b),
            a: self.a + t * (other.a - self.a),
        }
    }

    /// Blend this color with the other one, in the linear RGB color-space. `t` in the range [0..1].
    pub fn interpolate_linear_rgb(&self, other: &Color, t: f64) -> Color {
        let (r1, g1, b1, a1) = self.to_linear_rgba();
        let (r2, g2, b2, a2) = other.to_linear_rgba();
        Color::from_linear_rgba(
            r1 + t * (r2 - r1),
            g1 + t * (g2 - g1),
            b1 + t * (b2 - b1),
            a1 + t * (a2 - a1),
        )
    }

    #[deprecated]
    /// Blend this color with the other one, in the linear RGB color-space. `t` in the range [0..1].
    pub fn interpolate_lrgb(&self, other: &Color, t: f64) -> Color {
        self.interpolate_linear_rgb(&other, t)
    }

    /// Blend this color with the other one, in the HSV color-space. `t` in the range [0..1].
    pub fn interpolate_hsv(&self, other: &Color, t: f64) -> Color {
        let (h1, s1, v1, a1) = self.to_hsva();
        let (h2, s2, v2, a2) = other.to_hsva();
        Color::from_hsva(
            interp_angle(h1, h2, t),
            s1 + t * (s2 - s1),
            v1 + t * (v2 - v1),
            a1 + t * (a2 - a1),
        )
    }

    /// Blend this color with the other one, in the [Oklab](https://bottosson.github.io/posts/oklab/) color-space. `t` in the range [0..1].
    pub fn interpolate_oklab(&self, other: &Color, t: f64) -> Color {
        let (l1, a1, b1, alpha1) = self.to_oklaba();
        let (l2, a2, b2, alpha2) = other.to_oklaba();
        Color::from_oklaba(
            l1 + t * (l2 - l1),
            a1 + t * (a2 - a1),
            b1 + t * (b2 - b1),
            alpha1 + t * (alpha2 - alpha1),
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        }
    }
}

#[cfg(feature = "cint")]
mod impl_cint {
    use super::*;
    use cint::{Alpha, ColorInterop, EncodedSrgb};

    impl ColorInterop for Color {
        type CintTy = Alpha<EncodedSrgb<f64>>;
    }

    impl From<Color> for EncodedSrgb<f64> {
        fn from(c: Color) -> Self {
            let (r, g, b, _) = c.rgba();
            EncodedSrgb { r, g, b }
        }
    }

    impl From<EncodedSrgb<f64>> for Color {
        fn from(c: EncodedSrgb<f64>) -> Self {
            let EncodedSrgb { r, g, b } = c;
            Color::from_rgb(r, g, b)
        }
    }

    impl From<Color> for EncodedSrgb<f32> {
        fn from(c: Color) -> Self {
            let (r, g, b, _) = c.rgba();
            let (r, g, b) = (r as f32, g as f32, b as f32);
            EncodedSrgb { r, g, b }
        }
    }

    impl From<EncodedSrgb<f32>> for Color {
        fn from(c: EncodedSrgb<f32>) -> Self {
            let EncodedSrgb { r, g, b } = c;
            let (r, g, b) = (r as f64, g as f64, b as f64);
            Color::from_rgb(r, g, b)
        }
    }

    impl From<Color> for Alpha<EncodedSrgb<f64>> {
        fn from(c: Color) -> Self {
            let (r, g, b, alpha) = c.rgba();
            Alpha {
                color: EncodedSrgb { r, g, b },
                alpha,
            }
        }
    }

    impl From<Alpha<EncodedSrgb<f64>>> for Color {
        fn from(c: Alpha<EncodedSrgb<f64>>) -> Self {
            let Alpha {
                color: EncodedSrgb { r, g, b },
                alpha,
            } = c;
            Color::from_rgba(r, g, b, alpha)
        }
    }

    impl From<Color> for Alpha<EncodedSrgb<f32>> {
        fn from(c: Color) -> Self {
            let (r, g, b, alpha) = c.rgba();
            let (r, g, b, alpha) = (r as f32, g as f32, b as f32, alpha as f32);
            Alpha {
                color: EncodedSrgb { r, g, b },
                alpha,
            }
        }
    }

    impl From<Alpha<EncodedSrgb<f32>>> for Color {
        fn from(c: Alpha<EncodedSrgb<f32>>) -> Self {
            let Alpha {
                color: EncodedSrgb { r, g, b },
                alpha,
            } = c;
            let (r, g, b, alpha) = (r as f64, g as f64, b as f64, alpha as f64);
            Color::from_rgba(r, g, b, alpha)
        }
    }

    impl From<Color> for EncodedSrgb<u8> {
        fn from(c: Color) -> Self {
            let (r, g, b, _) = c.rgba_u8();
            EncodedSrgb { r, g, b }
        }
    }

    impl From<EncodedSrgb<u8>> for Color {
        fn from(c: EncodedSrgb<u8>) -> Self {
            let EncodedSrgb { r, g, b } = c;
            Color::from_rgb_u8(r, g, b)
        }
    }

    impl From<Color> for Alpha<EncodedSrgb<u8>> {
        fn from(c: Color) -> Self {
            let (r, g, b, alpha) = c.rgba_u8();
            Alpha {
                color: EncodedSrgb { r, g, b },
                alpha,
            }
        }
    }

    impl From<Alpha<EncodedSrgb<u8>>> for Color {
        fn from(c: Alpha<EncodedSrgb<u8>>) -> Self {
            let Alpha {
                color: EncodedSrgb { r, g, b },
                alpha,
            } = c;
            Color::from_rgba_u8(r, g, b, alpha)
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b, a) = self.rgba();
        write!(f, "RGBA({},{},{},{})", r, g, b, a)
    }
}

impl FromStr for Color {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl TryFrom<&str> for Color {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        parse(s)
    }
}

impl From<(f64, f64, f64, f64)> for Color {
    fn from((r, g, b, a): (f64, f64, f64, f64)) -> Self {
        Color { r, g, b, a }
    }
}

impl From<(f64, f64, f64)> for Color {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        Color { r, g, b, a: 1. }
    }
}

impl From<[f64; 4]> for Color {
    fn from([r, g, b, a]: [f64; 4]) -> Self {
        Color { r, g, b, a }
    }
}

impl From<[f64; 3]> for Color {
    fn from([r, g, b]: [f64; 3]) -> Self {
        Color { r, g, b, a: 1. }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Color::from_rgba_u8(r, g, b, a)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color::from_rgb_u8(r, g, b)
    }
}

impl From<[u8; 4]> for Color {
    fn from([r, g, b, a]: [u8; 4]) -> Self {
        Color::from_rgba_u8(r, g, b, a)
    }
}

impl From<[u8; 3]> for Color {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Color::from_rgb_u8(r, g, b)
    }
}

/// Convert rust-rgb's `RGB<f64>` type into `Color`.
#[cfg(feature = "rust-rgb")]
impl From<RGB<f64>> for Color {
    fn from(item: RGB<f64>) -> Self {
        Color::from_rgb(item.r, item.g, item.b)
    }
}

/// Convert rust-rgb's `RGBA<f64>` type into `Color`.
#[cfg(feature = "rust-rgb")]
impl From<RGBA<f64>> for Color {
    fn from(item: RGBA<f64>) -> Self {
        Color::from_rgba(item.r, item.g, item.b, item.a)
    }
}

/// Implement Serde serialization into HEX string
#[cfg(feature = "serde")]
impl Serialize for Color {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_hex_string())
    }
}

/// Implement Serde deserialization from string
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let string = String::deserialize(deserializer)?;
        Self::from_str(&string).map_err(serde::de::Error::custom)
    }
}

fn hue_to_rgb(n1: f64, n2: f64, h: f64) -> f64 {
    let h = modulo(h, 6.);

    if h < 1. {
        return n1 + ((n2 - n1) * h);
    }

    if h < 3. {
        return n2;
    }

    if h < 4. {
        return n1 + ((n2 - n1) * (4. - h));
    }

    n1
}

// h = 0..360
// s, l = 0..1
// r, g, b = 0..1
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    if s == 0. {
        return (l, l, l);
    }

    let n2 = if l < 0.5 {
        l * (1. + s)
    } else {
        l + s - (l * s)
    };

    let n1 = 2. * l - n2;
    let h = h / 60.;
    let r = hue_to_rgb(n1, n2, h + 2.);
    let g = hue_to_rgb(n1, n2, h);
    let b = hue_to_rgb(n1, n2, h - 2.);
    (r, g, b)
}

fn hwb_to_rgb(hue: f64, white: f64, black: f64) -> (f64, f64, f64) {
    if white + black >= 1. {
        let l = white / (white + black);
        return (l, l, l);
    }

    let (r, g, b) = hsl_to_rgb(hue, 1., 0.5);
    let r = r * (1. - white - black) + white;
    let g = g * (1. - white - black) + white;
    let b = b * (1. - white - black) + white;
    (r, g, b)
}

#[allow(clippy::float_cmp)]
fn hsv_to_hsl(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let l = (2. - s) * v / 2.;

    let s = if l != 0. {
        if l == 1. {
            0.
        } else if l < 0.5 {
            s * v / (l * 2.)
        } else {
            s * v / (2. - l * 2.)
        }
    } else {
        s
    };

    (h, s, l)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let (h, s, l) = hsv_to_hsl(h, s, v);
    hsl_to_rgb(h, s, l)
}

#[allow(clippy::float_cmp)]
fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let v = r.max(g.max(b));
    let d = v - r.min(g.min(b));

    if d == 0. {
        return (0., 0., v);
    }

    let s = d / v;
    let dr = (v - r) / d;
    let dg = (v - g) / d;
    let db = (v - b) / d;

    let h = if r == v {
        db - dg
    } else if g == v {
        2. + dr - db
    } else {
        4. + dg - dr
    };

    let h = (h * 60.) % 360.;
    (normalize_angle(h), s, v)
}

#[allow(clippy::float_cmp)]
fn rgb_to_hsl(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let min = r.min(g.min(b));
    let max = r.max(g.max(b));
    let l = (max + min) / 2.;

    if min == max {
        return (0., 0., l);
    }

    let d = max - min;

    let s = if l < 0.5 {
        d / (max + min)
    } else {
        d / (2. - max - min)
    };

    let dr = (max - r) / d;
    let dg = (max - g) / d;
    let db = (max - b) / d;

    let h = if r == max {
        db - dg
    } else if g == max {
        2. + dr - db
    } else {
        4. + dg - dr
    };

    let h = (h * 60.) % 360.;
    (normalize_angle(h), s, l)
}

fn rgb_to_hwb(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let (hue, _, _) = rgb_to_hsl(r, g, b);
    let white = r.min(g.min(b));
    let black = 1. - r.max(g.max(b));
    (hue, white, black)
}

#[inline]
fn normalize_angle(t: f64) -> f64 {
    let mut t = t % 360.;
    if t < 0. {
        t += 360.;
    }
    t
}

#[inline]
fn interp_angle(a0: f64, a1: f64, t: f64) -> f64 {
    let delta = (((a1 - a0) % 360.) + 540.) % 360. - 180.;
    (a0 + t * delta + 360.) % 360.
}

#[inline]
fn clamp0_1(t: f64) -> f64 {
    t.clamp(0., 1.)
}

#[inline]
fn modulo(x: f64, n: f64) -> f64 {
    (x % n + n) % n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_angle() {
        let data = vec![
            (0., 0.),
            (360., 0.),
            (400., 40.),
            (1155., 75.),
            (-360., 0.),
            (-90., 270.),
            (-765., 315.),
        ];
        for (x, expected) in data {
            let c = normalize_angle(x);
            assert_eq!(expected, c);
        }
    }

    #[test]
    fn test_interp_angle() {
        let data = vec![
            ((0., 360., 0.5), 0.),
            ((360., 90., 0.), 0.),
            ((360., 90., 0.5), 45.),
            ((360., 90., 1.), 90.),
        ];
        for ((a, b, t), expected) in data {
            let v = interp_angle(a, b, t);
            assert_eq!(expected, v);
        }
    }

    #[cfg(feature = "rust-rgb")]
    #[test]
    fn test_convert_rust_rgb_to_color() {
        let rgb = RGB::new(0.0, 0.5, 1.0);

        assert_eq!(Color::from_rgb(0.0, 0.5, 1.0), Color::from(rgb));

        let rgba = RGBA::new(1.0, 0.5, 0.0, 0.5);

        assert_eq!(Color::from_rgba(1.0, 0.5, 0.0, 0.5), Color::from(rgba));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serialize_to_hex() {
        let color = Color::from_rgba(1.0, 1.0, 0.5, 0.5);
        serde_test::assert_ser_tokens(&color, &[serde_test::Token::Str("#ffff8080")]);
    }

    #[cfg(all(feature = "serde", feature = "named-colors"))]
    #[test]
    fn test_serde_deserialize_from_string() {
        let named = Color::from_rgb(1.0, 1.0, 0.0);
        serde_test::assert_de_tokens(&named, &[serde_test::Token::Str("yellow")]);

        let hex = Color::from_rgba(0.0, 1.0, 0.0, 1.0);
        serde_test::assert_de_tokens(&hex, &[serde_test::Token::Str("#00ff00ff")]);

        let rgb = Color::from_rgba(0.0, 1.0, 0.0, 1.0);
        serde_test::assert_de_tokens(&rgb, &[serde_test::Token::Str("rgba(0,255,0,1)")]);
    }
}
