use std::convert::TryFrom;

pub struct Img<'path> {
    pub path: &'path str,
    pub data: image::DynamicImage,
}

impl<'path> Img<'path> {
    pub fn load(&self, path: &'path str) -> Result<Self, Box<dyn std::error::Error>> {
        let data = image::io::Reader::open(path)?.decode()?;
        Ok(Self {
            path,
            data,
        })
    }
}

pub struct ImgHash<'path> {
    pub path: &'path str,
    pub hash: u64,
}

impl<'path> Into<ImgHash<'path>> for Img<'path> {
    fn into(self) -> ImgHash<'path> {
        let grayscale = self.data.grayscale();
        let small_8x8 = grayscale.resize_exact(8, 8, image::imageops::FilterType::Triangle);
        let small_8x8_sum = small_8x8.as_bytes().iter().fold(0u64, |acc, v| acc + u64::from(*v));
        let mean = u8::try_from(small_8x8_sum / 64).expect("Mean is supposed to me <= to the max, and max couldn't be more than u8");
        // Shift 0 or 1 to some position based on counter, making a "bit vector"
        let hash = small_8x8.as_bytes().iter().fold((0u8, 0u64), |(counter, acc), v| (counter + 1, acc + u64::from(*v >= mean) << counter)).1;

        ImgHash {
            path: self.path,
            hash,
        }
    }
}
