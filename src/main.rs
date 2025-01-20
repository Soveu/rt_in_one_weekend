use png;
use std::error::Error;

mod vec;
mod color;
mod ray;
mod hittable;
mod sphere;
mod world;
use vec::{Vec3, Point3};
use color::Color;
use ray::Ray;
use world::World;
use crate::sphere::Sphere;

const IMG_WIDTH: usize = 600;
const IMG_HEIGHT: usize = 400;
// const IMG_RATIO: f32 = 1.5;
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ((IMG_WIDTH as f32) / (IMG_HEIGHT as f32));

type Image = [[Color; IMG_WIDTH]; IMG_HEIGHT];
const ZEROED_IMAGE: Image = [[Color::new(); IMG_WIDTH]; IMG_HEIGHT];

fn dump_to_png(image: &Image) -> Result<(), Box<dyn Error>> {
    use std::fs::File;
    use std::io::BufWriter;

    let file = File::create("result.png")?;
    let mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(&mut w, IMG_WIDTH as u32, IMG_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Sixteen);

    let data: Vec<u8> = image
        .iter()
        .flatten()
        .flat_map(|c: &Color| c.to_rgb16())
        .collect();

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&data)?;

    Ok(())
}

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = center.sub(r.orig);

    let a = r.dir.len_square();
    let h = r.dir.dot(oc);
    let c = oc.len_square() - radius*radius;
    let discriminant = h*h - a*c;

    return if discriminant >= 0.0 {
        (h - discriminant.sqrt()) / a
    } else {
        -1.0
    };
}

fn color_lerp(n: f32, color_a: Color, color_b: Color) -> Color {
    let a = color_a.to_vec3().mul_scalar(1.0 - n);
    let b = color_b.to_vec3().mul_scalar(n);
    Color::from_vec3(a.add(b))
}

fn ray_color(ray: &Ray, world: &world::World) -> Color {
    let initial_range = (0.0 .. f32::INFINITY);
    let unit_color = Color { r: 1.0, b: 1.0, g: 1.0 };
    if let Some(hh) = world.hit(ray, initial_range) {
        return color_lerp(0.5, unit_color, Color::from_vec3(hh.normal));
    }

    let unit_dir = ray.dir.unit();
    let n = 0.5 * (unit_dir.0[1] + 1.0);
    return color_lerp(
        n,
        Color { r: 1.0, g: 1.0, b: 1.0 },
        Color { r: 0.5, g: 0.7, b: 1.0 },
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut img = ZEROED_IMAGE;

    let focal_length = 1.0;
    let camera_center: Point3 = Vec3([0.0, 0.0, 0.0]);
    let viewport_u = Vec3([VIEWPORT_WIDTH, 0.0, 0.0]);
    let viewport_v = Vec3([0.0, -VIEWPORT_HEIGHT, 0.0]);
    let pixel_delta_u = viewport_u.mul_scalar(1.0 / (IMG_WIDTH as f32));
    let pixel_delta_v = viewport_v.mul_scalar(1.0 / (IMG_HEIGHT as f32));
    let viewport_upper_left = camera_center
        .sub(Vec3([0.0, 0.0, focal_length]))
        .sub(viewport_u.mul_scalar(0.5))
        .sub(viewport_v.mul_scalar(0.5));
    let pixel00_loc = viewport_upper_left
        .add(pixel_delta_u.add(pixel_delta_v).mul_scalar(0.5));

    let world = World {
        spheres: vec![
            Sphere { center: Vec3([0.0, 0.0, -1.0]), radius: 0.5 },
            Sphere { center: Vec3([0.0, -100.5, -1.0]), radius: 100.0 },
        ]
    };

    for h in 0..IMG_HEIGHT {
        for w in 0..IMG_WIDTH {
            let pixel_center = pixel00_loc
                .add(pixel_delta_u.mul_scalar(w as f32))
                .add(pixel_delta_v.mul_scalar(h as f32));
            let ray_direction = pixel_center.sub(camera_center);

            let r = Ray { orig: camera_center, dir: ray_direction };
            img[h][w] = ray_color(&r, &world);
        }
    }

    dump_to_png(&img)?;

    Ok(())
}
