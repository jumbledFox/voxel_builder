use std::{time::SystemTime, vec};
use glam;

use glium::{glutin::{event_loop, event::ElementState, window::CursorGrabMode}, Surface};

use crate::chunk::Chunk;

#[macro_use]
extern crate glium;
extern crate image;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

pub mod chunk_mesh;
pub mod chunk;
pub mod window_context;
pub mod camera;

fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    //display.gl_window().window().set_cursor_grab(CursorGrabMode::Confined);

    let mut kb = window_context::Keyboard::new();
    let mut m = window_context::Mouse::new();

    let mut cam = camera::FlyCamera::new();
    cam.reset_mouse_pos(&display);

    let mut polygon_mode = glium::PolygonMode::Fill;

    use chunk_mesh::ChunkVertex;

    implement_vertex!(ChunkVertex, position, tex_coords);

    let mut chunk: Chunk = Chunk::new();
    chunk.set_voxel_coordinate(glam::U64Vec3{x: 1, y: 0, z: 0}, 1);
    let chunk_mesh: chunk_mesh::ChunkMesh = chunk_mesh::ChunkMeshBuilder::build_chunk_mesh(&chunk.voxels);
    // implement_vertex!(Vertex, position, tex_coords);

    // let shape = vec![
    //    Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [0.0, 1.0] },
    //    Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [1.0, 1.0] },
    //    Vertex { position: [-0.5, -0.5, 1.0], tex_coords: [0.0, 0.0] },
    //    Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [1.0, 0.0] },
    // ];
    // let ind: Vec<u16> = vec![0, 1, 2, 1, 3, 2];

    let vertex_buffer = glium::VertexBuffer::new(&display, &chunk_mesh.vertices).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
        &chunk_mesh.indices).unwrap();

    let vertex_shader_src = include_str!("default.vert");
    let fragment_shader_src = include_str!("default.frag");

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    
    // Load image
    use std::io::Cursor;
    let image = image::load(Cursor::new(&include_bytes!("../res/white.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let mut looking = false;
    let mut fullscreen = false;

    let mut deltatime = 0.0;
    let mut deltatimer = SystemTime::now();
    event_loop.run(move |ev, _, control_flow| {
        deltatime = deltatimer.elapsed().unwrap().as_secs_f32();
        deltatimer = SystemTime::now();
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        // -- LOGIC -- //
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => 
                    *control_flow = glutin::event_loop::ControlFlow::Exit,
                glutin::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => 
                    kb.process_input(input),
                glutin::event::WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                    m.process_input(state, button);
                }
                glutin::event::WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => 
                    m.set_scroll_delta(delta),
                glutin::event::WindowEvent::CursorMoved { device_id, position, modifiers } => 
                    { m.set_pos(position); if (looking) { cam.handle_mouse_looking(&display, &position); }
                    },
                
                _ => return,
            },
            _ => (),
        }

        if kb.key_pressed(glutin::event::VirtualKeyCode::Escape) {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        if kb.key_pressed(glutin::event::VirtualKeyCode::C) {
            println!("Wireframe mode");
            polygon_mode = if matches!(polygon_mode, glium::PolygonMode::Line) {glium::PolygonMode::Fill} else {glium::PolygonMode::Line};
        }
        if kb.key_pressed(glutin::event::VirtualKeyCode::E) {
            looking = !looking;
            // Reset mouse to prevent jump (TODO: Make work)
            if looking == true {
                cam.reset_mouse_pos(&display);
            }
        }
        if kb.key_held(glutin::event::VirtualKeyCode::LAlt) && kb.key_pressed(glutin::event::VirtualKeyCode::Return) {
            if fullscreen {
                display.gl_window().window().set_fullscreen(None);
                fullscreen = false;
            } else {
                 let monitor_handle = display.gl_window().window().available_monitors().next().unwrap();
                let fs = glutin::window::Fullscreen::Borderless(Some(monitor_handle));
                display.gl_window().window().set_fullscreen(Some(fs));
                fullscreen = true;
            }
            // Reset mouse to prevent jump (TODO: Make work)
            cam.reset_mouse_pos(&display);
        }

        cam.handle_movement(&kb, &deltatime);

        // -- DRAWING -- //
        
        let mut target = display.draw();

        cam.camera.calculate_perspective_matrix(&target);
        cam.camera.calculate_view_matrix();
        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
        };
        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 2.0, 1.0f32],
            ],
            perspective: cam.camera.perspective_matrix.to_cols_array_2d(),
            view: cam.camera.view_matrix.to_cols_array_2d(),
            tex: glium::uniforms::Sampler(&texture, behavior),
        };

        // Clear the screen
        target.clear_color_and_depth((0.05078125, 0.0546875, 0.0859375, 1.0), 1.0);
        // Draw the triangle
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &glium::DrawParameters {
            polygon_mode: polygon_mode,
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise,
            .. Default::default()
        }).unwrap();  
        target.finish().unwrap();

        // -- END LOGIC -- //
        kb.clear();
        m.clear();
    });
}
