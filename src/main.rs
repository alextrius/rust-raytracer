

fn main() {
    // Image dimensions
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining {}", image_height - j);

        for i in 0..image_width {
            let mut vec = Vec3::new(i / (image_width - 1), j / (image_height - 1), 0);
            write_color(vec);
        }
    }
    eprintln!("\rDone.");
}
