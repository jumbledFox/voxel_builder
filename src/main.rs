use std::{time::SystemTime, vec, collections::HashMap, str::CharIndices};
use glam;
use bracket_noise::prelude::*;
use bracket_random::prelude::*;

use glium::{glutin::{event_loop, event::ElementState, window::CursorGrabMode}, Surface, Blend};

use voxel_builder::{chunk::{Convert, Chunk, self, ChunkPosition, VoxelPosition}, chunk_manager::ChunkManager, chunk_mesh::{self, ChunkVertex, ChunkMesh}, window_context, camera::{self, FlyCamera}, voxel_data_manager::{VoxelDataManager, VoxelData}};

#[macro_use]
extern crate glium;
extern crate image;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

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
    FlyCamera::reset_mouse_pos(&display);

    let mut set_mode: u8 = 0;
    let mut draw_mode: u32 = 0;
    let mut colour_chunks: bool = true;
    let mut polygon_mode = glium::PolygonMode::Fill;
    let mut cull_mode = glium::draw_parameters::BackfaceCullingMode::CullingDisabled;

    // Load images
    let mut images: Vec<glium::texture::RawImage2d<'_, u8>> = vec![];

    let voxel_data_manager = VoxelDataManager::new(vec![
        ("Air",         1, vec!["missing"]),
        ("Grass Block", 0, vec!["grass_top", "dirt", "grass_side"]),
        ("Dirt",        0, vec!["dirt"]),
        ("Stone",       0, vec!["stone"]),
        ("Deep Stone",  0, vec!["deep_stone"]),
        ("Sand",        0, vec!["sand"]),
        ("Oak Log",     0, vec!["oak_log_top", "oak_log_top", "oak_log_side"]),
        ("Oak Planks",  0, vec!["oak_planks"]),
        ("Leaves",      0, vec!["leaves"]),
        ("Grass",       1, vec!["grass"]),
        ("Cobblestone", 0, vec!["cobblestone"]),
        ("Bricks",      0, vec!["bricks"]),
        ("C4",          0, vec!["c4", "c4", "c4_side"]),
    ], &mut images);
    
    let texture_2d_array = glium::texture::SrgbTexture2dArray::new(&display, images).unwrap();

    let mut chunk_manager = ChunkManager::new(voxel_data_manager);

    let mut chunk_info: HashMap<ChunkPosition, (glium::VertexBuffer<ChunkVertex>, glium::IndexBuffer<u32>, u32)> = HashMap::new();

    for xi in -8..8 {
        for yi in -5..2 {
            for zi in -8..8 {
                let chunk_pos: ChunkPosition = glam::ivec3(xi, yi, zi);
                chunk_manager.add_chunk(chunk_pos);
    } } }

    let mut q = 0;
    for xi in -8..8 {
        for yi in -5..2 {
            for zi in -8..8 {
                let chunk_pos: ChunkPosition = glam::ivec3(xi, yi, zi);
                
                //let c = chunk_manager.get_chunk_mut(chunk_pos).unwrap();

                let chunk_mesh: chunk_mesh::ChunkMesh = chunk_mesh::ChunkMeshBuilder::build_chunk_mesh(chunk_pos, &mut chunk_manager);
                
                let vertex_buffer = glium::VertexBuffer::new(&display, &chunk_mesh.vertices).unwrap();
                let indexs = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                    &chunk_mesh.indices).unwrap();
                chunk_info.insert(chunk_pos, (vertex_buffer, indexs, q));
                q+=1;
            }
        }
        q+=2;
    }

    let vertex_shader_src = include_str!("default.vert");
    let fragment_shader_src = include_str!("default.frag");

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

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
                    {kb.process_input(input);},
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
            glutin::event::Event::MainEventsCleared => {
                // Rendering
                let mut target = display.draw();

                cam.camera.calculate_perspective_matrix(&target);
                cam.camera.calculate_view_matrix();
        
                // Clear the screen
                target.clear_color_and_depth((0.05078125, 0.0546875, 0.0859375, 1.0), 1.0);
                // Draw the triangle
        
                for (pos, (vertex_buffer, index_buffer, q)) in &chunk_info {
                    let uniforms = uniform! {
                        matrix: [
                            [1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0f32],
                        ],
                        perspective: cam.camera.perspective_matrix.to_cols_array_2d(),
                        view: cam.camera.view_matrix.to_cols_array_2d(),
                        texture_array: texture_2d_array.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest).minify_filter(glium::uniforms::MinifySamplerFilter::Nearest),
                        // chunk_position: glam::ivec3(0, 0, 0).to_array(),
                        chunk_position: pos.to_array(),
                        chunk_colour: *q,
                        draw_mode: draw_mode,
                        colour_chunks: colour_chunks,
                    };
            
                    target.draw(vertex_buffer, index_buffer, &program, &uniforms, &glium::DrawParameters {
                        polygon_mode: polygon_mode,
                        depth: glium::Depth {
                            test: glium::draw_parameters::DepthTest::IfLess,
                            write: true,
                            .. Default::default()
                        },
                        blend: Blend::alpha_blending(),
                        backface_culling: cull_mode,
                        .. Default::default()
                    }).unwrap();  
                }
                
                target.finish().unwrap();
            },
            _ => (),

            // TODO: make transparent objects their own mesh
        }

        if kb.key_pressed(glutin::event::VirtualKeyCode::Escape) {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        if kb.key_pressed(glutin::event::VirtualKeyCode::C) {
            println!("Wireframe toggled");
            polygon_mode = if matches!(polygon_mode, glium::PolygonMode::Line) {glium::PolygonMode::Fill} else {glium::PolygonMode::Line};
        }
        if kb.key_pressed(glutin::event::VirtualKeyCode::B) {
            println!("Backface Culling toggled");
            cull_mode = if matches!(cull_mode, glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise) {glium::draw_parameters::BackfaceCullingMode::CullingDisabled} else {glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise};
        }
        if kb.key_pressed(glutin::event::VirtualKeyCode::E) {
            looking = !looking;
            // Reset mouse to prevent jump (TODO: Make work)
            if looking == true {
                FlyCamera::reset_mouse_pos(&display);
                display.gl_window().window().set_cursor_visible(false);
            } else {
                display.gl_window().window().set_cursor_visible(true);
            }
        }
        if kb.key_pressed(glutin::event::VirtualKeyCode::X) {
            colour_chunks = !colour_chunks;
        }
        if kb.key_pressed(glutin::event::VirtualKeyCode::Z) {
            draw_mode += 1;
            if draw_mode > 2 {
                draw_mode = 0;
            }
        }
        if kb.key_pressed(glutin::event::VirtualKeyCode::Q) {
            set_mode += 1;
            if set_mode > 11 {
                set_mode = 0;
            }
            println!("Block: {:?}", chunk_manager.voxel_data_manager.get_name(set_mode));
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
            FlyCamera::reset_mouse_pos(&display);
        }

        cam.handle_movement(&kb, &deltatime);
        
        let mut to_rebuild: Vec<ChunkPosition> = vec![];
        
        // TODO: AO needs updating
        for i in 0..27 {
            let cam_head_pos = cam.camera.position.as_ivec3() + glam::ivec3(i % 3, (i / 9)-1, (i / 3)%3)-(3/2);
            if chunk_manager.get_voxel(cam_head_pos) == Some(set_mode) {
                continue;
            }
            match chunk_manager.set_voxel(cam_head_pos, set_mode) {
                true => {
                    let cam_head_pos_chunk = Convert::global_to_chunk(cam_head_pos);
                    to_rebuild.push(cam_head_pos_chunk);
                    for (j, vp, cp) in chunk::RELATIVE_NEIGHBOURS {
                        if Convert::global_to_local(cam_head_pos).to_array()[j] == vp {
                            let chunk_neigbour = cam_head_pos_chunk + cp;
                            if chunk_manager.get_chunk(chunk_neigbour).is_some() {
                                to_rebuild.push(chunk_neigbour);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        to_rebuild.dedup();
        
        for cp in to_rebuild.clone() {
            let new_chunk_mesh = chunk_mesh::ChunkMeshBuilder::build_chunk_mesh(cp, &mut chunk_manager);
            chunk_info.get_mut(&cp).unwrap().0 = glium::VertexBuffer::new(&display, &new_chunk_mesh.vertices).unwrap();
            chunk_info.get_mut(&cp).unwrap().1 = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                    &new_chunk_mesh.indices).unwrap();
                        
        }
        

        // -- END LOGIC -- //
        kb.clear();
        m.clear();
    });
}
