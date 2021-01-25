use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use ndarray::{arr3, aview1, Axis};
use netcdf;

fn main() {
    let netcdf_file = netcdf::open("test.nc")
        .unwrap();

    for var in netcdf_file.variables() {
        println!("{}", var.name());
        println!("\tAttributes:");
        for att in var.attributes() {
            println!("\t{} -> {:?}", att.name(), att.value().unwrap());
        }
        println!("\tDimenstions:");
        for dim in var.dimensions() {
            println!("\t{} -> {:?}", dim.name(), dim.len());
        }
        println!();
    }
    let area = &netcdf_file.variable("area").expect("no area var");
    let land = &netcdf_file.variable("longitude").expect("no land var");
    println!("area type: {:?}", area.vartype());
    println!("area len: {:?}", area.len());
    
    println!("land type: {:?}", land.vartype());
    println!("land len: {:?}", land.len());
    println!("land dim: {:?}", land.dimensions());
    
    let land_data_f32 = land.values::<f32>(None, None).unwrap();
    println!("{:?}", land_data_f32);
    println!("{:?}", land_data_f32.dim());

    let shape = land_data_f32.dim();
    let mut img = RgbImage::new(shape[0] as u32, shape[1] as u32);

    for i in 0..shape[0] {
        for j in 0..shape[1] {
            let p = ((land_data_f32[[j, i]] + 180.0) / 2.0) as u8;
            img.put_pixel(i as u32, j as u32, image::Rgb([p, p, p]));
        }
    }

    img.save("test.png").unwrap();
}
