use std::convert::TryFrom;

use prisma::FromColor;

pub struct Img<P>
where
    P: AsRef<std::path::Path>,
{
    pub path: P,
    pub data: image::DynamicImage,
}

impl<P> Img<P>
where
    P: AsRef<std::path::Path>,
{
    pub fn load(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let data = image::io::Reader::open(path.as_ref())?.decode()?;
        Ok(Self { path, data })
    }
}

pub struct ImgFeatures<P>
where
    P: AsRef<std::path::Path>,
{
    pub path: P,
    pub lshash: u64,
    pub hue: angle::Turns<f64>,
}

impl<P> From<Img<P>> for ImgFeatures<P>
where
    P: AsRef<std::path::Path>,
{
    fn from(img: Img<P>) -> ImgFeatures<P> {
        let original_rgb = image::DynamicImage::ImageRgb8(img.data.into_rgb8());

        // Convert the picture to grayscale and then downscale it to 8x8.
        let grayscale = original_rgb.grayscale();
        let grayscale_8x8 = grayscale.resize_exact(8, 8, image::imageops::FilterType::Triangle);

        // Find mean value of the grayscale 8x8 image.
        let grayscale_8x8_sum = grayscale_8x8
            .as_bytes()
            .iter()
            .fold(0u64, |acc, v| acc + u64::from(*v));
        let mean = u8::try_from(grayscale_8x8_sum / 64).expect(
            "Mean is supposed to be less or equal to max, and max couldn't be greater than 255",
        );

        // Shift 0 or 1 to some position based on counter, making a "bit vector" that is the "imghash" of the image.
        let lshash = grayscale_8x8
            .as_bytes()
            .iter()
            .fold((0u8, 0u64), |(counter, acc), v| {
                let bit = *v >= mean;
                let bit_positioned = u64::from(bit) << counter;
                (counter + 1, acc + bit_positioned)
            })
            .1;

        // Find mean color and then extract its hue in the HSV color space.
        let mut color_rgb = (0., 0., 0.);
        let original_rgb_bytes = original_rgb.as_bytes();
        let pixels = original_rgb_bytes.len() / 3;
        for i in 0..pixels {
            color_rgb = (
                color_rgb.0 + original_rgb_bytes[i+0] as f64,
                color_rgb.1 + original_rgb_bytes[i+1] as f64,
                color_rgb.2 + original_rgb_bytes[i+2] as f64,
            );
        }
        let color_rgb = prisma::Rgb::new(
            color_rgb.0 / pixels as f64,
            color_rgb.1 / pixels as f64,
            color_rgb.2 / pixels as f64,
        );
        let color_hsv = prisma::Hsv::from_color(&color_rgb);
        let hue: angle::Turns<f64> = color_hsv.hue();

        ImgFeatures {
            path: img.path,
            lshash,
            hue,
        }
    }
}
