extern crate image;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;
use std::sync::Arc;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
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

fn bgcolor(ren: &RenderEnv, direction: &Vec3) -> RenderColor{
    use std::f32::consts::PI;
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

#[wasm_bindgen]
pub fn render_func(context: &CanvasRenderingContext2d, width: usize, height: usize, pos: Vec<f32>, pyr: Vec<f32>) -> Result<(), JsValue> {
    let xmax = width;
    let ymax = height;
    let xfov = 1.;
    let yfov = ymax as f32 / xmax as f32;
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

    let mut data = vec![0u8; 4 * width * height];

    for y in 0..height {
        for x in 0..width {
            data[(x + y * width) * 4    ] = ((x) * 255 / width) as u8;
            data[(x + y * width) * 4 + 1] = ((y) * 255 / height) as u8;
            data[(x + y * width) * 4 + 2] = ((x + y) % 32 + 32) as u8;
            data[(x + y * width) * 4 + 3] = 255;
        }
    }

    let mut putpoint = |x: i32, y: i32, fc: &RenderColor| {
        data[(x as usize + y as usize * width) * 4    ] = (fc.r * 255.).min(255.) as u8;
        data[(x as usize + y as usize * width) * 4 + 1] = (fc.g * 255.).min(255.) as u8;
        data[(x as usize + y as usize * width) * 4 + 2] = (fc.b * 255.).min(255.) as u8;
    };

    let mut ren: RenderEnv = RenderEnv::new(
        if 3 == pos.len() {
            Vec3::new(pos[0], pos[1], pos[2])
        }
        else {
            Vec3::new(0., -150., -300.)
        }, /* cam */
        if 3 == pyr.len() {
            Vec3::new(pyr[0], pyr[1], pyr[2])
        }
        else {
            Vec3::new(0., -PI / 2., -PI / 2.)
        }, /* pyr */
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

    let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&mut data), width as u32, height as u32)?;
    context.put_image_data(&image_data, 0., 0.);

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

#[wasm_bindgen]
pub fn deserialize_string(save_data: &str, width: usize, height: usize, callback: js_sys::Function) -> Result<(), JsValue>{
    let xmax = width;
    let ymax = height;
    let xfov = 1.;
    let yfov = ymax as f32 / xmax as f32;
    use std::f32::consts::PI;

    let mut ren: RenderEnv = RenderEnv::new(
        Vec3::new(0., -150., -300.), /* cam */
        Vec3::new(0., -PI / 2., -PI / 2.), /* pyr */
        xmax as i32,
        ymax as i32, /* xres, yres */
        xfov,
        yfov, /* xfov, yfov*/
        //pointproc: putpoint, /* pointproc */
        bgcolor, /* bgproc */
    );
    ren.deserialize(&save_data).map_err(|e| JsValue::from(
        "Deserialize error: ".to_string() + &e.s))?;
    log(&format!("deserialized materials: {}, objects: {}, camera_motion: {}", ren.materials.len(), ren.objects.len(), ren.camera_motion.0.len()));

    let mut data = vec![0u8; 4 * width * height];

    for y in 0..height {
        for x in 0..width {
            data[(x + y * width) * 4    ] = ((x) * 255 / width) as u8;
            data[(x + y * width) * 4 + 1] = ((y) * 255 / height) as u8;
            data[(x + y * width) * 4 + 2] = ((x + y) % 32 + 32) as u8;
            data[(x + y * width) * 4 + 3] = 255;
        }
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i >= ren.camera_motion.0.len() {
            i = i % ren.camera_motion.0.len();
            // body().set_text_content(Some("All done!"));

            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            // let _ = f.borrow_mut().take();
            // return;
        }
        let motion = &ren.camera_motion.0[i];

        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        document().get_element_by_id("label").unwrap().set_inner_html(&text);

        log(&format!("  camera: {} {} {}", motion.camera.position.x, motion.camera.position.y, motion.camera.position.z));

        let mut putpoint = |x: i32, y: i32, fc: &RenderColor| {
            data[(x as usize + y as usize * width) * 4    ] = (fc.r * 255.).min(255.) as u8;
            data[(x as usize + y as usize * width) * 4 + 1] = (fc.g * 255.).min(255.) as u8;
            data[(x as usize + y as usize * width) * 4 + 2] = (fc.b * 255.).min(255.) as u8;
        };

        ren.camera = motion.camera;
    
        render(&ren, &mut putpoint, 1);

        log(&format!("data: {}, {}", data[0], data.len()));

        let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&mut data), width as u32, height as u32).unwrap();

        // context.put_image_data(&image_data, 0., 0.);

        callback.call1(&window(), &JsValue::from(image_data));
    
        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    // request_animation_frame(g.borrow().as_ref().unwrap());
    // for motion in &ren.camera_motion.0 {
    //     log(&format!("  camera: {} {} {}", motion.camera.position.x, motion.camera.position.y, motion.camera.position.z));

    //     let mut putpoint = |x: i32, y: i32, fc: &RenderColor| {
    //         data[(x as usize + y as usize * width) * 4    ] = (fc.r * 255.).min(255.) as u8;
    //         data[(x as usize + y as usize * width) * 4 + 1] = (fc.g * 255.).min(255.) as u8;
    //         data[(x as usize + y as usize * width) * 4 + 2] = (fc.b * 255.).min(255.) as u8;
    //     };

    //     ren.camera = motion.camera;
    
    //     render(&ren, &mut putpoint, 1);

    //     log(&format!("data: {}, {}", data[0], data.len()));
    
    //     let image_data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&mut data), width as u32, height as u32)?;

    //     rendered.call1(&JsValue::from(web_sys::window()), &JsValue::from(image_data));
    // }
    // println!("deserialized {} materials and {} objects", ren.materials.len(), ren.objects.len());
    // for material in ren.materials.iter() {
    //     println!("  {:?}", material);
    // }
    // for (i, object) in ren.objects.iter().enumerate() {
    //     println!("  [{}]: {}", i, object.get_interface().get_material().get_name());
    // }
    Ok(())
}

