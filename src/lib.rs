extern crate image;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;
use std::sync::Arc;

use wasm_bindgen::prelude::*;
use render::{RenderColor,
    UVMap,
    RenderMaterial, RenderPattern,
    RenderObject, RenderSphere, RenderFloor,
    RenderEnv,
    render, render_frames};
use vec3::Vec3;

mod render;
mod vec3;
mod quat;
mod modutil;
mod pixelutil;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn helloworld() -> String {
    String::from("Hello world from Rust!")
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn example() {
    log("Log from Rust!!!");
}


#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[wasm_bindgen]
pub fn render_func(width: usize, height: usize) -> Vec<u8> {
    let xmax = width;
    let ymax = height;
    let xfov = 1.;
    let yfov = 1.;
    let thread_count = 1;
    use std::f32::consts::PI;

    let mut materials: HashMap<String, Arc<RenderMaterial>> = HashMap::new();

    let floor_material = Arc::new(RenderMaterial::new("floor".to_string(),
        RenderColor::new(1.0, 1.0, 0.0), RenderColor::new(0.0, 0.0, 0.0),  0, 0., 0.0)
        .pattern(RenderPattern::RepeatedGradation)
        .pattern_scale(300.)
        .pattern_angle_scale(0.2)
        .texture_ok("bar.png"));
    materials.insert("floor".to_string(), floor_material);

    let mirror_material = Arc::new(RenderMaterial::new("mirror".to_string(),
        RenderColor::new(0.0, 0.0, 0.0), RenderColor::new(1.0, 1.0, 1.0), 24, 0., 0.0)
        .frac(RenderColor::new(1., 1., 1.)));

    let red_material = Arc::new(RenderMaterial::new("red".to_string(),
        RenderColor::new(0.8, 0.0, 0.0), RenderColor::new(0.0, 0.0, 0.0), 24, 0., 0.0)
        .glow_dist(5.));

    let transparent_material = Arc::new(RenderMaterial::new("transparent".to_string(),
        RenderColor::new(0.0, 0.0, 0.0), RenderColor::new(0.0, 0.0, 0.0),  0, 1., 1.5)
        .frac(RenderColor::new(1.49998, 1.49999, 1.5)));


    let objects: Vec<RenderObject> = vec!{
        /* Plane */
            RenderObject::Floor(
                RenderFloor::new_raw(materials.get("floor").unwrap().clone(),       Vec3::new(  0.0, -300.0,  0.0),  Vec3::new(0., 1., 0.))
                .uvmap(UVMap::ZX),
            ),
            // RenderFloor::new (floor_material,       Vec3::new(-300.0,   0.0,  0.0),  Vec3::new(1., 0., 0.)),
        /* Spheres */
            RenderSphere::new(mirror_material.clone(), 80.0, Vec3::new(   0.0, -30.0,172.0)),
            RenderSphere::new(mirror_material, 80.0, Vec3::new(   -200.0, -30.0,172.0)),
            RenderSphere::new(red_material, 80.0, Vec3::new(-200.0,-200.0,172.0)),
        // /*	{80.0F,  70.0F,-200.0F,150.0F, 0.0F, 0.0F, 0.8F, 0.0F, 0.0F, 0.0F, 0.0F,24, 1., 1., {1.}},*/
            RenderSphere::new(transparent_material, 100.0, Vec3::new(  70.0,-200.0,150.0)),
        /*	{000.F, 0.F, 0.F, 1500.F, 0.0F, 0.0F, 0.0F, 0.0F, 1.0F, 1.0F, 1.0F,24, 0, 0},*/
        /*	{100.F, -70.F, -150.F, 160.F, 0.0F, 0.5F, 0.0F, 0.0F, 0.0F, 0.0F, 0.0F,24, .5F, .2F},*/
        };

    fn bgcolor(ren: &RenderEnv, direction: &Vec3) -> RenderColor{
        let phi = direction.z.atan2(direction.x);
        let the = direction.y.asin();
        let d = (50. * PI + phi * 10. * PI) % (2. * PI) - PI;
        let dd = (50. * PI + the * 10. * PI) % (2. * PI) - PI;
        let ret = RenderColor::new(
            0.5 / (15. * (d * d * dd * dd) + 1.),
            0.25 - direction.y / 4.,
            0.25 - direction.y / 4.,
        );
        let dot = ren.light.dot(direction);

        if dot > 0.9 {
            if 0.9995 < dot {
                RenderColor::new(2., 2., 2.)
            }
            else {
                let ret2 = if 0.995 < dot {
                    let dd = (dot - 0.995) * 150.;
                    RenderColor::new(ret.r + dd, ret.g + dd, ret.b + dd)
                } else { ret };
                let dot2 = dot - 0.9;
                RenderColor::new(ret2.r + dot2 * 5., ret2.g + dot2 * 5., ret2.b)
            }
        }
        else {
            ret
        }
        // else PointMandel(dir->x * 2., dir->z * 2., 32, ret);
    }

    let mut data = vec![0u8; 3 * width * height];

    for y in 0..height {
        for x in 0..width {
            data[(x + y * width) * 3    ] = ((x) * 255 / width) as u8;
            data[(x + y * width) * 3 + 1] = ((y) * 255 / height) as u8;
            data[(x + y * width) * 3 + 2] = ((x + y) % 32 + 32) as u8;
        }
    }

    let mut putpoint = |x: i32, y: i32, fc: &RenderColor| {
        data[(x as usize + y as usize * width) * 3    ] = (fc.r * 255.).min(255.) as u8;
        data[(x as usize + y as usize * width) * 3 + 1] = (fc.g * 255.).min(255.) as u8;
        data[(x as usize + y as usize * width) * 3 + 2] = (fc.b * 255.).min(255.) as u8;
    };

    let mut ren: RenderEnv = RenderEnv::new(
        Vec3::new(0., -150., -300.), /* cam */
        Vec3::new(0., -PI / 2., -PI / 2.), /* pyr */
        xmax as i32,
        ymax as i32, /* xres, yres */
        xfov,
        yfov, /* xfov, yfov*/
        //pointproc: putpoint, /* pointproc */
        bgcolor, /* bgproc */
    )
    .materials(materials)
    .objects(objects)
    .light(Vec3::new(50., 60., -50.))
    .use_raymarching(false)
    .glow_effect(None);
    log(&format!("pyr: {}, {}, {}", ren.camera.pyr.x, ren.camera.pyr.y, ren.camera.pyr.z));

    render(&ren, &mut putpoint, thread_count);

    log(&format!("data: {}, {}", data[0], data.len()));

    data
}