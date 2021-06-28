//! TODO

use std::convert::TryFrom;

/// Locality-sensitive hash of an image.
///
/// Can not be used as a key for sorting in a regular way,
/// as any changes in its bits have equal importance.
/// Basically, if it was a `u8`, then these two hashes:
/// `0b01001111` and `0b0001111` would have the same "difference"
/// between them as these two hashes:
/// `0b10000001` and `0b10000000`.
/// Read about Hamming distance if you want to know more.
///
/// When used as a key for sorting, will cause "clusters"
/// to appear in the dataset, each having the same number of enabled bits
/// in the hashes of its members.
///
/// Other properties of the LSH are implementation defined,
/// it is supposed to be compared with other hashes only
/// by using Equality or Hamming distance.
///
/// PartialOrd for this struct is defined
/// on the number of bits the hash has.
///
/// Ord is not defined, as LsHash does not have a total order.
///
/// # Examples
///
/// ## PartialEq and Eq
/// ```
/// # use libsuccotash::analyze::img_features::LsHash;
/// let just_a = LsHash::new(0b00100000u64);
/// let also_a = LsHash::new(0b00100000u64);
/// let just_b = LsHash::new(0b00000001u64);
/// assert!(just_a == also_a);
/// assert!(just_a != just_b);
/// ```
///
/// ## PartialOrd
/// ```
/// # use libsuccotash::analyze::img_features::LsHash;
/// let a = LsHash::new(0b00100000u64);
/// let b = LsHash::new(0b00000011u64);
/// assert!(a < b);
/// assert!(b > a);
/// ```
///
/// ## Not Ord
/// ```
/// # use libsuccotash::analyze::img_features::LsHash;
/// let a = LsHash::new(0b00100000u64);
/// let b = LsHash::new(0b00000001u64);
/// assert!(a != b);
///
/// assert!(!(a < b));
/// assert!(!(a > b));
/// assert!(a.partial_cmp(&b) == None);
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct LsHash(u64);

impl LsHash {
    pub fn new(lshash: u64) -> Self {
        Self(lshash)
    }

    /// Find [`LsHash`] of an image.
    ///
    /// # Arguments
    ///
    /// * `original` - the image to find [`LsHash`] for.
    pub fn find(original: &image::RgbImage) -> Self {
        let original = image::DynamicImage::ImageRgb8(original.clone());

        // Convert the picture to grayscale and then downscale it to 8x8.
        let grayscale = original.grayscale();
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

        Self::new(lshash)
    }
}

impl std::fmt::Display for LsHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl PartialOrd for LsHash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_ones = self.0.count_ones();
        let other_ones = other.0.count_ones();

        if self_ones < other_ones {
            Some(std::cmp::Ordering::Less)
        } else if self_ones > other_ones {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}
