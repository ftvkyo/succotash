use super::img_features;

/// Image -- path to it and its contents.
/// Convert to [`Img`] to make useful.
pub struct ImgRaw<P>
where
    P: AsRef<std::path::Path>,
{
    /// Path to where the image was loaded from
    pub path: P,
    /// Contents of the image.
    /// Can be any enum variant depending on the actual file.
    pub data: image::DynamicImage,
}

impl<P> ImgRaw<P>
where
    P: AsRef<std::path::Path>,
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
    pub fn load(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let data = image::io::Reader::open(path.as_ref())?.decode()?;
        Ok(Self { path, data })
    }
}

/// Like [`Img`] but with features and without content.
/// Has fields that describe features of the image.
/// Use From/Into to convert [`Img`] into this.
pub struct Img<P>
where
    P: AsRef<std::path::Path>,
{
    pub raw: ImgRaw<P>,
    pub features: img_features::ImgFeatures,
}

impl<P> From<ImgRaw<P>> for Img<P>
where
    P: AsRef<std::path::Path>,
{
    fn from(original: ImgRaw<P>) -> Img<P> {
        Img {
            features: img_features::ImgFeatures::find(&original),
            raw: original,
        }
    }
}
