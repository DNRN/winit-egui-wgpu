// fn draw_png (image_width: u32, image_height: u32, ) {
//     let ratio = image_height as f32 / image_width as f32;
//     let center = Point {x: 0.0, y: 0.0};
//     let width = 4.0 as f32;
//     let height = width * ratio;
//     let init_x = center.x - (width / 2 as f32);
//     let init_y = center.y - (height / 2 as f32);
//     let inc = width / (image_width as f32);

//     let mut image_buffer = image::ImageBuffer::new(
//         image_width, image_height);

//     for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
//         // let u = x as f32 / image_height as f32;
//         // let v = y as f32 / image_height as f32;
//         // let t = mandelbrot(2.5 * (u - 0.5) - 1.4, 2.5 * (v - 0.5));
//         let u = init_x + (x as f32 * inc);
//         let v = init_y + (y as f32 * inc);
//         let t = mandelbrot(u, v);
//         *pixel = image::Rgb(color((2.0 * t + 0.5) % 1.0));
//     }

//     image_buffer.save("mandelbrot.png").unwrap();
// }

struct FractalPlot {
    center: Point,
    width: f32,
    height: f32,
    init_x: f32,
    init_y: f32,
    inc: f32,
}

impl FractalPlot {
    pub fn new(center: Point, screen_size: winit::dpi::PhysicalSize<u32>) -> Self {
        let ratio = screen_size.height as f32 / screen_size.width as f32;
        let width = 4.0 as f32;
        let height = width * ratio;
        let init_x = center.x - (width / 2 as f32);
        let init_y = center.y - (height / 2 as f32);
        let inc = width / (screen_size.width as f32);

        Self { center, width, height, init_x, init_y, inc }
    }

    pub fn getPoint(&self, screenCoordinate: (i16, i16)) -> (f32, f32) {
        let u = self.init_x + (screenCoordinate.0 as f32 * self.inc);
        let v = self.init_y + (screenCoordinate.1 as f32 * self.inc);
        return (u, v)
    }
}

struct Point {
    pub x: f32,
    pub y: f32
}

#[derive(Clone, Copy)]
struct Complex {
    pub a: f32,
    pub b: f32,
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex { 
            a: self.a * rhs.a - self.b * rhs.b, 
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}

impl Complex {
    fn arg_sq(self) -> f32 {
        self.a * self.a + self.b * self.b
    }
}

fn mandelbrot(x: f32, y: f32) -> f32 {
    let mut z = Complex { a: 0.0, b: 0.0 };
    let c = Complex { a: x, b: y };
    let max = 256;
    let mut i = 0;
    while i < max && z.arg_sq() < 32.0 {
        z = z * z + c;
        i += 1;
    }
    return (i as f32 - z.arg_sq().log2().log2()) / (max as f32);
}

fn color(t: f32) -> [u8; 3] {
    let a = (0.5, 0.5, 0.5);
    let b = (0.5, 0.5, 0.5);
    let c = (1.0, 1.0, 1.0);
    let d = (0.0, 0.10, 0.20);
    let r = b.0 * (6.28318 * (c.0 * t + d.0)).cos() + a.0;
    let g = b.1 * (6.28318 * (c.1 * t + d.1)).cos() + a.1;
    let b = b.2 * (6.28318 * (c.2 * t + d.2)).cos() + a.2;
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}