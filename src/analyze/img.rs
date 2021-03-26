use std::convert::TryFrom;

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

pub struct ImgHash<P>
where
    P: AsRef<std::path::Path>,
{
    pub path: P,
    pub hash: u64,
}

impl<P> From<Img<P>> for ImgHash<P>
where
    P: AsRef<std::path::Path>,
{
    fn from(img: Img<P>) -> ImgHash<P> {
        // Convert the picture to grayscale and then downscale it to 8x8.
        let grayscale = img.data.grayscale();
        let small_8x8 = grayscale.resize_exact(8, 8, image::imageops::FilterType::Triangle);

        // Find mean value of the grayscale 8x8 image.
        let small_8x8_sum = small_8x8
            .as_bytes()
            .iter()
            .fold(0u64, |acc, v| acc + u64::from(*v));
        let mean = u8::try_from(small_8x8_sum / 64).expect(
            "Mean is supposed to be less or equal to max, and max couldn't be greater than 255",
        );

        // Shift 0 or 1 to some position based on counter, making a "bit vector" that is the "imghash" of the image.
        let hash = small_8x8
            .as_bytes()
            .iter()
            .fold((0u8, 0u64), |(counter, acc), v| {
                let bit = *v >= mean;
                let bit_positioned = u64::from(bit) << counter;
                (counter + 1, acc + bit_positioned)
            })
            .1;

        ImgHash {
            path: img.path,
            hash,
        }
    }
}
