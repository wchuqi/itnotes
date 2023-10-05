extern crate num;
extern crate image;
extern crate crossbeam;

use num::Complex;

use std::str::FromStr;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

use std::io::Write;

fn main() {
    println!("Hello, world!");
    // f1();
    f2();
}

// 非并行
fn f1() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(std::io::stderr(),
        "Usage: mandelbrot FILE PIXELS UPPERLEFT
        LOWERRIGHT")
        .unwrap();
        writeln!(std::io::stderr(),
        "Example: {} mandel.png 1000x750 -1.20,0.35
        -1,0.20",
        args[0])
        .unwrap();
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x')
    .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
    .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
    .expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];
    render(&mut pixels, bounds, upper_left, lower_right);
    write_image(&args[1], &pixels, bounds)
    .expect("error writing PNG file");
}

// 并行
fn f2() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(std::io::stderr(),
        "Usage: mandelbrot FILE PIXELS UPPERLEFT
        LOWERRIGHT")
        .unwrap();
        writeln!(std::io::stderr(),
        "Example: {} mandel.png 1000x750 -1.20,0.35
        -1,0.20",
        args[0])
        .unwrap();
        std::process::exit(1);
    }
    let bounds = parse_pair(&args[2], 'x')
    .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
    .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
    .expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [u8]> =
        pixels.chunks_mut(rows_per_band * bounds.0).collect();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left =
                pixel_to_point(bounds, (0, top), upper_left,
                lower_right);
                let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top +
                height),
                upper_left, lower_right);
                spawner.spawn(move || {
                render(band, band_bounds, band_upper_left,
                band_lower_right);
                });
            }
        });
    }

    write_image(&args[1], &pixels, bounds)
    .expect("error writing PNG file");
}

/// 确定c是否属于曼德布洛特集合，最多循环limit次
///
/// 如果c不是成员，就返回Some(i)，其中i是在z离开以原点为圆心、
/// 半径为2的圆时循环的次数。如果c是成员（更准确地说，若达到
/// 循环上限尚未证明c不是成员），则返回None
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/// 解析字符串s，格式为一对坐标值，如"400x600"或"1.0,0.5"
///
/// 特别地，s的格式应该是"<左值><分隔符><右值>"的形式，其中<分隔符>
/// 就是separator参数传入的字符，而<左值>和<右值>都是字符串，可以通过
/// T::from_str来解析
///
/// 如果s的格式没错，就返回Some<(x, y)>。如果解析出错，则返回None
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}
#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,
    20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5,
    1.5)));
}

/// 将由逗号分隔的一对浮点数值解析为一个复数
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}
   #[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
    Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// 给定输出图像中一个像素的行和列，对应到复平面上的一个点
///
/// bounds是一个元组，值为以像素计量的图像的宽和高
/// pixel是(列,行)元组，表示图像中一个特定的像素
/// upper_left和lower_right参数是复平面中的两个点，
/// 指定了图像涵盖的区域
fn pixel_to_point(bounds: (usize, usize),
 pixel: (usize, usize),
 upper_left: Complex<f64>,
 lower_right: Complex<f64>)
 -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re,
    upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0
        as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1
        as f64
        // 这里为什么要用减法？pixel.1越往下越大，而虚部越往上越大
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 100), (25, 75),
    Complex { re: -1.0, im: 1.0 },
    Complex { re: 1.0, im: -1.0 }),
    Complex { re: -0.5, im: -0.5 });
}

/// 将矩形区域内的曼德布洛特集合渲染为像素保存在缓冲区
///
/// bounds参数给出缓冲区pixels的宽度和高度，后者的每个字节都保存一个
/// 灰阶像素。upper_left和lower_right参数指定与像素缓冲区中左上角和
/// 右下角的点对应的复平面上的点
fn render(pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0 .. bounds.1 {
        for column in 0 .. bounds.0 {
            let point = pixel_to_point(bounds, (column, row),
            upper_left,
            lower_right);
            pixels[row * bounds.0 + column] =
            match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

/// 把缓冲区中的pixels（大小由bounds指定）写到
/// 名为filename的文件中
fn write_image(filename: &str, pixels: &[u8], bounds: (usize,
    usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
    bounds.0 as u32, bounds.1 as u32,
    ColorType::Gray(8))?;
    Ok(())
}
    
// cargo build --release
// cd target/release
// andelbrot_test.exe mandel.png 1000x750 -1.20,0.35 -1,0.20
