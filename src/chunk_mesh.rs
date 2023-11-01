use glam;
use glium::implement_vertex;
implement_vertex!(ChunkVertex, position, tex_coords, light_level, texture_id, ambient_occlusion);

use crate::{chunk::{Chunk, VoxelPosition, ChunkPosition, Convert, VoxelID}, chunk_manager::ChunkManager};

#[derive(Copy, Clone)]
pub struct ChunkVertex {
    pub position: [f32; 3],
    pub tex_coords: u8,
    pub light_level: u8,
    pub texture_id: u32,
    pub ambient_occlusion: f32,
}

pub struct MeshFace {
    pub vertices: [u8; 12],
    pub light_level: u8,
}

pub struct BlocksAround {
    pub blocks: [u8; 8],
}

const BLOCKS_AROUND_RELATIVE: [VoxelPosition; 8] = [
    glam::ivec3(-1, 1, -1),
    glam::ivec3( 0, 1, -1),
    glam::ivec3( 1, 1, -1),
    glam::ivec3(-1, 1,  0),
    glam::ivec3( 1, 1,  0),
    glam::ivec3(-1, 1,  1),
    glam::ivec3( 0, 1,  1),
    glam::ivec3( 1, 1,  1),
];


pub const FRONT_FACE  : MeshFace = MeshFace{ vertices: [1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1], light_level: 4 };
pub const BACK_FACE   : MeshFace = MeshFace{ vertices: [0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0], light_level: 4 };
pub const LEFT_FACE   : MeshFace = MeshFace{ vertices: [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1], light_level: 3 };
pub const RIGHT_FACE  : MeshFace = MeshFace{ vertices: [1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0], light_level: 3 };
pub const TOP_FACE    : MeshFace = MeshFace{ vertices: [1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1], light_level: 5 };
pub const BOTTOM_FACE : MeshFace = MeshFace{ vertices: [0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1], light_level: 2 };

pub struct ChunkMesh {
    pub vertices: Vec<ChunkVertex>,
    pub indices: Vec<u32>,
    indices_count: usize,
}
impl ChunkMesh {
    pub fn new() -> Self {
        Self { vertices: vec![], indices: vec![], indices_count: 0 }
    }
    pub fn add_face(&mut self, face: MeshFace, position: VoxelPosition, texture_id: u32, ao: [f32;4]) {
        let mut index: usize = 0;
        for i in 0..4 {
            let x = face.vertices[index] as i32 + position.x;
            index += 1;
            let y = face.vertices[index] as i32 + position.y;
            index += 1;
            let z = face.vertices[index] as i32 + position.z;
            index += 1;
            self.vertices.push(ChunkVertex { position: [x as f32, y as f32, z as f32], tex_coords: i as u8, light_level: face.light_level, texture_id: texture_id, ambient_occlusion: ao[i] });
        }
        // First triangle
        self.indices.push(self.indices_count as u32);
        self.indices.push(self.indices_count as u32 + 1);
        self.indices.push(self.indices_count as u32 + 2);
        // Second triangle
        self.indices.push(self.indices_count as u32 + 2);
        self.indices.push(self.indices_count as u32 + 3);
        self.indices.push(self.indices_count as u32);
        
        self.indices_count += 4;
    }
}

// A list of block faces and where to look (relatively) to check if they should be added to the mesh or not
pub const FACES_AND_OFFSETS: [(MeshFace, glam::IVec3, u8); 6] = [
    (TOP_FACE   , glam::ivec3( 0,  1,  0), 1),
    (BOTTOM_FACE, glam::ivec3( 0, -1,  0), 4),
    (LEFT_FACE  , glam::ivec3(-1,  0,  0), 0),
    (RIGHT_FACE , glam::ivec3( 1,  0,  0), 3),
    (FRONT_FACE , glam::ivec3( 0,  0,  1), 2),
    (BACK_FACE  , glam::ivec3( 0,  0, -1), 5),
];

pub struct ChunkMeshBuilder { }
impl ChunkMeshBuilder {
    // Builds a chunk mesh from a given chunk
    pub fn build_chunk_mesh(chunk_position: ChunkPosition, chunk_manager: &mut ChunkManager) -> ChunkMesh {
        // Make the mesh
        let mut mesh = ChunkMesh::new();

        let chunk = chunk_manager.get_chunk(chunk_position);
        if chunk.is_none() {
            println!("Tried to build mesh of chunk that doesn't exist!! {:?}", chunk_position);
            return mesh;
        }
        let chunk = chunk.unwrap();

        // For every block in the chunk
        for (i, &voxel_id) in chunk.voxels.iter().enumerate() {
            // If it's air
            if voxel_id == 0 { continue; }
            // For every face of the block, if it's neighbour is transparent, add the face to the mesh
            for (face, offset, plane) in FACES_AND_OFFSETS {
                let neighbour_coord = Chunk::index_to_coordinates(i) + offset;
                let should_add_face;
                if Chunk::coordinate_out_of_bounds(neighbour_coord) {
                    // If the coordinate is out of bounds, check the neighbouring chunk
                    let gotten_neighbour = chunk_manager.get_voxel(
                        Convert::local_to_global(chunk_position,
                            Chunk::index_to_coordinates(i)) + offset);
                    should_add_face = gotten_neighbour == Some(0) || gotten_neighbour == None;
                } else {
                    // Otherwise check the current chunk
                    should_add_face = chunk.get_voxel_from_coordinate(neighbour_coord) == 0;
                }
                if should_add_face {
                    let texture_id = chunk_manager.voxel_data_manager.get_texture_id(voxel_id, plane as usize);
                    // TODO: find a way to optimise this garbage
                    let ao = ChunkMeshBuilder::get_ambient_occlusion(Convert::local_to_global(chunk_position, Chunk::index_to_coordinates(i) + offset), chunk_manager, plane);
                    //mesh.add_face(face, Chunk::index_to_coordinates(i), texture_id, [3.0, 3.0, 3.0, 3.0]);
                    mesh.add_face(face, Chunk::index_to_coordinates(i), texture_id, ao);
                }
            }
        }

        mesh // return mesh 
    }

    pub fn get_ambient_occlusion(global_voxel_position: VoxelPosition, chunk_manager: &ChunkManager, plane: u8) -> [f32;4] {
        let x = global_voxel_position.x;
        let y = global_voxel_position.y;
        let z = global_voxel_position.z;
        let (a, b, c, d, e, f, g, h): (f32, f32, f32, f32, f32, f32, f32, f32);
        if plane % 3 == 1 {
            a = chunk_manager.is_void(glam::ivec3(x  , y, z-1)) .into();
            b = chunk_manager.is_void(glam::ivec3(x-1, y, z-1)).into();
            c = chunk_manager.is_void(glam::ivec3(x-1, y, z)).into();
            d = chunk_manager.is_void(glam::ivec3(x-1, y, z+1)).into();
            e = chunk_manager.is_void(glam::ivec3(x  , y, z+1)).into();
            f = chunk_manager.is_void(glam::ivec3(x+1, y, z+1)).into();
            g = chunk_manager.is_void(glam::ivec3(x+1, y, z)).into();
            h = chunk_manager.is_void(glam::ivec3(x+1, y, z-1)).into();
            if plane == 1 {
                return [(g + h + a), (a + b + c), (c + d + e), (e + f + g)];
            } else {
                return [(a + b + c), (g + h + a), (e + f + g), (c + d + e)];
            }
        } else if plane % 3 == 0 {
            a = chunk_manager.is_void(glam::ivec3(x, y, z-1)).into();
            b = chunk_manager.is_void(glam::ivec3(x, y-1, z-1)).into();
            c = chunk_manager.is_void(glam::ivec3(x, y-1, z)).into();
            d = chunk_manager.is_void(glam::ivec3(x, y-1, z+1)).into();
            e = chunk_manager.is_void(glam::ivec3(x, y, z+1)).into();
            f = chunk_manager.is_void(glam::ivec3(x, y+1, z+1)).into();
            g = chunk_manager.is_void(glam::ivec3(x, y+1, z)).into();
            h = chunk_manager.is_void(glam::ivec3(x, y+1, z-1)).into(); 
            let o = [(e + f + g), (g + h + a), (a + b + c), (c + d + e)];
            if plane == 0 {
                return o;
            } else {
                return [o[1], o[0], o[3], o[2]];
            }
        } else {
            a = chunk_manager.is_void(glam::ivec3(x-1, y, z)).into();
            b = chunk_manager.is_void(glam::ivec3(x-1, y-1, z)).into();
            c = chunk_manager.is_void(glam::ivec3(x, y-1, z)).into();
            d = chunk_manager.is_void(glam::ivec3(x+1, y-1, z)).into();
            e = chunk_manager.is_void(glam::ivec3(x+1, y, z)).into();
            f = chunk_manager.is_void(glam::ivec3(x+1, y+1, z)).into();
            g = chunk_manager.is_void(glam::ivec3(x, y+1, z)).into();
            h = chunk_manager.is_void(glam::ivec3(x-1, y+1, z)).into(); 
            let o = [(e + f + g), (g + h + a), (a + b + c), (c + d + e)];
            if plane == 2 {
                return o;
            } else {
                return [o[1], o[0], o[3], o[2]];
            } 
        }
    }
}