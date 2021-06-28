//! Implementation of internally-used image structures.

use super::img_features;

/// Image - path to it and its contents.
///
/// Convert to [`Img`] to make useful.
pub struct ImgRaw<P>
where
    P: AsRef<async_std::path::Path>,
{
    /// Path to where the image was loaded from
    pub path: P,
    /// Contents of the image.
    /// Can be any enum variant depending on the actual file.
    pub data: image::DynamicImage,
}

impl<P> ImgRaw<P>
where
    P: AsRef<async_std::path::Path>,
{
    /// Load an image from a given path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path where to load the image from.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use libsuccotash::analyze::img::ImgRaw;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let wallpaper = ImgRaw::load("/home/user/Pictures/wallpaper.png")?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let data_raw = async_std::fs::read(path.as_ref()).await?;
        let data = image::io::Reader::new(std::io::Cursor::new(data_raw))
            .with_guessed_format()?
            .decode()?;
        Ok(Self { path, data })
    }
}

/// An image and its features.
///
/// The "final" image structure you probably want to work with.
/// Has fields that describe features of the image.
/// Use From/Into to convert [`Img`] into this.
pub struct Img<P>
where
    P: AsRef<async_std::path::Path>,
{
    /// The original image we find features of.
    pub path: P,
    /// Features of the image.
    /// See [`img_features`] for details.
    pub features: img_features::ImgFeatures,
}

impl<P> From<ImgRaw<P>> for Img<P>
where
    P: AsRef<async_std::path::Path>,
{
    fn from(original: ImgRaw<P>) -> Img<P> {
        Img {
            features: img_features::ImgFeatures::find(&original),
            path: original.path,
        }
    }
}
