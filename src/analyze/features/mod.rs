//! Image features.
//!
//! Various features of images that can narrow down a dataset that
//! a search can be performed on. Some of the features can be used
//! to sort the dataset, others don't. See documentation to learn.

mod hue;
mod lshash;

use super::img::ImgRaw;
use hue::Hue;
use lshash::LsHash;

/// Features of an image.
///
/// Can be used as a key when sorting a number of images
/// to later use binary search on these images.
///
/// Has more than one feature, when sorting,
/// higher features have higher priority.
///
/// # Examples
///
#[derive(PartialEq, PartialOrd)]
pub struct ImgFeatures {
    /// Locality-sensitive hash of the image.
    pub lshash: LsHash,
    /// Hue characteristic of the image.
    pub hue: Hue,
}

impl ImgFeatures {
    /// Find ImgFeatures for a given Image.
    ///
    /// # Arguments
    ///
    /// * `original` - image to find the features for.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use libsuccotash::analyze::img::ImgRaw;
    /// # use libsuccotash::analyze::img_features::ImgFeatures;
    /// let img_raw = ImgRaw {
    ///     path: "/home/user/pic.png",
    ///     data: image::DynamicImage::ImageRgb8(image::RgbImage::new(32, 32)),
    /// };
    /// let img_features = ImgFeatures::find(&img_raw);
    /// ```
    pub fn find<P>(original: &ImgRaw<P>) -> Self
    where
        P: AsRef<async_std::path::Path>,
    {
        let original_rgb = original.data.to_rgb8();

        Self {
            lshash: LsHash::find(&original_rgb),
            hue: Hue::find(&original_rgb),
        }
    }
}
