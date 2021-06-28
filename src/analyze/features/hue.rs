//! TODO

/// Hue of an image, in degrees, normalized.
///
/// Can be used as a key for sorting in a regular way.
/// This hue may be calculated in a weird way, don't rely on it
/// to be generated in some potentially existing standard way.
///
/// We make sure to limit the angle with [0, 360) by normalizing
/// the value on creation.
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Hue(f64);

impl std::fmt::Display for Hue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl Hue {
    pub fn new(a: angle::Deg<f64>) -> Self {
        use angle::Angle;
        Self(a.normalize().scalar())
    }

    /// Find [`Hue`] of an image.
    ///
    /// # Arguments
    ///
    /// * `original` - the image to find [`Hue`] for.
    pub fn find(original_rgb: &image::RgbImage) -> Self {
        use prisma::FromColor;

        // Find mean color and then extract its hue in the HSV color space.
        let mut color_rgb = (0., 0., 0.);
        let original_rgb_bytes = original_rgb.as_raw();
        let pixels = original_rgb_bytes.len() / 3;
        for i in 0..pixels {
            color_rgb = (
                color_rgb.0 + original_rgb_bytes[i] as f64,
                color_rgb.1 + original_rgb_bytes[i + 1] as f64,
                color_rgb.2 + original_rgb_bytes[i + 2] as f64,
            );
        }
        let color_rgb = prisma::Rgb::new(
            color_rgb.0 / pixels as f64,
            color_rgb.1 / pixels as f64,
            color_rgb.2 / pixels as f64,
        );
        let color_hsv = prisma::Hsv::from_color(&color_rgb);
        let hue = color_hsv.hue();

        Hue::new(hue)
    }
}
