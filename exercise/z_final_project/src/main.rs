const SIZE: u32 = 800;
const SCALE: f32 = 0.00375;

fn main() {
    //
    // Challenge: Use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            let sigma: f32 = args.remove(0).parse().expect("Failed to parse sigma"); // Converts the String into f32
            // let sigma: f32 = args.remove(0).parse().unwrap();
            // let sigma = args.remove(0).parse::<f32>().unwrap();

            blur(infile, outfile, sigma);
        }

        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let value: i32 = args.remove(0).parse().expect("Failed to parse value");

            brighten(infile, outfile, value);
        }

        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).parse().expect("Failed to parse x");
            let y: u32 = args.remove(0).parse().expect("Failed to parse y");
            let width: u32 = args.remove(0).parse().expect("Failed to parse width");
            let height: u32 = args.remove(0).parse().expect("Failed to parse height");

            crop(infile, outfile, x, y, width, height);
        }

        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let degree = args.remove(0);

            rotate(infile, outfile, degree);
        }

        "invert" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let direction = args.remove(0);

            invert(infile, outfile, direction);
        }

        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            grayscale(infile, outfile);
        }

        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        "generate" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            let width: u32 = args.remove(0).parse().expect("Failed to parse width");
            let height: u32 = args.remove(0).parse().expect("Failed to parse height");
            let green: u8 = args.remove(0).parse().expect("Failed to parse green");
            let red: u8 = args.remove(0).parse().expect("Failed to parse red");
            let blue: u8 = args.remove(0).parse().expect("Failed to parse blue");
            generate(outfile, width, height, green, red, blue);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE Sigma(f32)");
    println!("brighten INFILE OUTFILE value(i32)");
    println!("crop INFILE OUTFILE x y width height");
    println!("rotate INFILE OUTFILE 90/180/270");
    println!("invert INFILE OUTFILE h/v");
    println!("grayscale INFILE OUTFILE");
    println!("fractal OUTFILE");
    std::process::exit(-1);
}

fn blur(infile: String, outfile: String, sigma: f32) {
    // Open an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");

    // Takes one argument (f32)
    let img2 = img.blur(sigma);

    // Save an image to a file. (Creates the file if it doesnt exist yet)
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, value: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    // Takes one argument (i32). Positive numbers brighten the image.
    // Negative numbers darken it.
    let img2 = img.brighten(value);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");

    // Takes four arguments:
    let img2 = img.crop(x, y, width, height);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, degree: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    // The compiler will let you declare unassigned variables if you properly assign them later
    let img2;

    match degree.as_str() {
        "90" => {
            img2 = img.rotate90();
        }
        "180" => {
            img2 = img.rotate180();
        }
        "270" => {
            img2 = img.rotate270();
        }
        _ => {
            println!("{} is not included in the possibilities of rotation", degree);
            std::process::exit(-1);
        }
    }

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String, direction: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2;

    match direction.as_str() {
        "h" => {
            img2 = img.fliph();
        }
        "v" => {
            img2 = img.flipv();
        }
        _ => {
            println!("{} is not included in the possibilities of invertion", direction);
            std::process::exit(-1);
        }
    }

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = img.grayscale();

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn generate(outfile: String, width: u32, height: u32, green: u8, red: u8, blue: u8) {
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Actually set the red, green, and blue u8 values!
        println!("Pixel x:{}  y:{}", x, y);
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let mut imgbuf = image::ImageBuffer::new(SIZE, SIZE);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * SCALE - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cx);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** 
// Make all of the subcommands stackable!
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ... program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
