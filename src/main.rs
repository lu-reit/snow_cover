use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use netcdf;

fn main() {
    let netcdf_file = netcdf::open(
        "test.nc"
        ).unwrap();
    for dim in netcdf_file.variables() {
        println!("{:?}", dim.name());
    }

    let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    img.save("test.png").unwrap();
}
