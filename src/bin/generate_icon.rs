use std::fs::File;
use image::GenericImageView;
use ico::{IconDir, IconImage};

fn main() {
    let src_path = "logo/logo.png";
    let dst_path = "assets/rustscan.ico";
    
    println!("Opening {}", src_path);
    // Ensure assets dir exists
    std::fs::create_dir_all("assets").unwrap();

    let img = image::open(src_path).expect("Failed to open source image");
    let mut icon_dir = IconDir::new(ico::ResourceType::Icon);

    let sizes = vec![256, 128, 64, 48, 32, 16];

    for size in sizes {
        println!("Processing size {}x{}", size, size);
        let resized = img.resize(size, size, image::imageops::FilterType::Lanczos3);
        
        // Save png for app usage if size is 256
        if size == 256 {
            resized.save("assets/logo.png").expect("Failed to save logo.png");
            println!("Saved assets/logo.png");
        }
        
        // Convert to ICO Image
        let width = resized.width();
        let height = resized.height();
        let raw = resized.to_rgba8().into_raw();
        
        let icon_image = IconImage::from_rgba_data(width, height, raw);
        icon_dir.add_entry(ico::IconDirEntry::encode(&icon_image).unwrap());
    }

    let file = File::create(dst_path).expect("Failed to create ICO file");
    icon_dir.write(file).expect("Failed to write ICO file");
    
    println!("Successfully created {}", dst_path);
}
