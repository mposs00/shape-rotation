#![feature(str_split_whitespace_as_str)]

mod vec3;
mod obj;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

use vec3::Vec3f;
use obj::Object;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 800)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let (width, height) = canvas.window().size();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let obj = Object::new("./model.obj");
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        //line(0, 0, width as i32, height as i32, &mut canvas, Color::RGB(255, 255, 255));
        //line(width as i32, 0, 0, height as i32, &mut canvas, Color::RGB(255, 0, 0));
        
        //line(354, 205, 343, 207, &mut canvas, Color::RGB(0, 255, 0));
        for i in 0..obj.num_faces() {
            //println!("drawing face {}, verts {} {} {}", i, obj.faces[i].verts[0], obj.faces[i].verts[1], obj.faces[i].verts[2]);
            let verts = obj.get_face_verts(i);
            for n in 0..verts.len() {
                //println!("v0 : vert {}: {} {} {}", n, verts[n].x, verts[n].y, verts[n].z);
                //println!("v1 : vert {}: {} {} {}", (n + 1) % verts.len(), verts[(n + 1) % verts.len()].x, verts[(n + 1) % verts.len()].y, verts[(n + 1) % verts.len()].z);
                let v0 = verts[n];
                let v1 = verts[(n + 1) % verts.len()];
                let x0 = (v0.x + 1.) * (width as f32 / 2.);
                let y0 = (v0.y + 1.) * (height as f32 / 2.);
                let x1 = (v1.x + 1.) * (width as f32 / 2.);
                let y1 = (v1.y + 1.) * (height as f32 / 2.);
                //println!("drawing vert idx {} for face {}", n, i);

                //println!("x0 {} y0 {} x1 {} y1 {}", x0, y0, x1, y1);
                line(x0 as i32, height as i32 - y0 as i32, x1 as i32, height as i32 - y1 as i32, &mut canvas, Color::RGB(0, 255, 0));
                canvas.present();
                //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 ));
            }
        }
        canvas.present();
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, color: Color) {
    //println!("x0 {} y0 {} x1 {} y1 {}", x0, y0, x1, y1);
    canvas.set_draw_color(color);
    
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }

    //println!("transformed: x0 {} y0 {} x1 {} y1 {}", x0, y0, x1, y1);
    let (dx, dy) = (x1 - x0, y1 - y0);
    let derror2 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    for x in x0..=x1 {
        if steep {
            canvas.draw_point(Point::new(y, x));
        }
        else {
            canvas.draw_point(Point::new(x, y));
        }

        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}