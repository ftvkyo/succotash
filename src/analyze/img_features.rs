//! Various features of images that can narrow down a dataset that
//! a search can be performed on. Some of the features can be used
//! to sort the dataset, others don't. See documentation to learn.

use std::{borrow::Borrow, convert::TryFrom};

use super::img::ImgRaw;

/// Locality-sensitive hash of an image.
/// Can not be used to sort the dataset, as any changes in its
/// bits have equal importance. Basically, if it was a `u8`,
/// then these two hashes: `0b01001111` and `0b0001111` would
/// have the same "difference" between them as these two hashes:
/// `0b10000001` and `0b10000000`.
/// Read about Hamming distance if you want to know more.
///
/// Other properties of the hash are implementation defined,
/// it is supposed to be compared with other hashes only
/// by using Equality or Hamming distance.
#[derive(PartialEq, Eq, Ord, Debug)]
pub struct LSHash(u64);

impl LSHash {
    pub fn new(lshash: u64) -> Self {
        Self(lshash)
    }
}

impl std::fmt::Display for LSHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl PartialOrd for LSHash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_ones = self.0.count_ones();
        let other_ones = other.0.count_ones();

        if self_ones < other_ones {
            Some(std::cmp::Ordering::Less)
        } else if self_ones < other_ones {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

fn find_lshash(original_rgb: &image::DynamicImage) -> LSHash {
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

    return LSHash::new(lshash);
}

pub type Hue = angle::Turns<f64>;

fn find_hue(original_rgb: &image::DynamicImage) -> Hue {
    use prisma::FromColor;

    // Find mean color and then extract its hue in the HSV color space.
    let mut color_rgb = (0., 0., 0.);
    let original_rgb_bytes = original_rgb.as_bytes();
    let pixels = original_rgb_bytes.len() / 3;
    for i in 0..pixels {
        color_rgb = (
            color_rgb.0 + original_rgb_bytes[i + 0] as f64,
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
    let hue: angle::Turns<f64> = color_hsv.hue();

    return hue;
}

#[derive(PartialEq, PartialOrd)]
pub struct ImgFeatures {
    /// Locality-sensitive hash of the image.
    pub lshash: LSHash,
    /// Hue characteristic of the image.
    ///
    /// This value may be calculated in a weird way, don't rely on it
    /// to comply with some standard "mean" hue of and image.
    pub hue: Hue,
}

impl ImgFeatures {
    pub fn find<P: AsRef<std::path::Path>, I: Borrow<ImgRaw<P>>>(original: I) -> Self {
        let original_rgb = image::DynamicImage::ImageRgb8(original.borrow().data.to_rgb8());

        Self {
            lshash: find_lshash(&original_rgb),
            hue: find_hue(&original_rgb),
        }
    }
}
