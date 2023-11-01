use glam;

use std::collections::HashMap;

pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_SIZE_USIZE: usize = CHUNK_SIZE as usize;
pub const CHUNK_SIZE_MIN1: i32 = CHUNK_SIZE-1;

pub type VoxelID = u8;
pub type VoxelList = [VoxelID; CHUNK_SIZE_USIZE*CHUNK_SIZE_USIZE*CHUNK_SIZE_USIZE];
pub type VoxelPosition = glam::IVec3;
pub type ChunkPosition = glam::IVec3;
pub const DEFAULT_VOXELS: VoxelList = [0; CHUNK_SIZE_USIZE*CHUNK_SIZE_USIZE*CHUNK_SIZE_USIZE];
pub const RELATIVE_NEIGHBOURS: [(usize, i32, ChunkPosition); 6] = [
    (0, 0              , glam::ivec3(-1, 0, 0)),
    (0, CHUNK_SIZE_MIN1, glam::ivec3( 1, 0, 0)),
    (1, 0              , glam::ivec3( 0,-1, 0)),
    (1, CHUNK_SIZE_MIN1, glam::ivec3( 0, 1, 0)),
    (2, 0              , glam::ivec3( 0, 0,-1)),
    (2, CHUNK_SIZE_MIN1, glam::ivec3( 0, 0, 1)),
];

// Chunk - 32x32x32 array of voxels
#[derive(Clone)]
pub struct Chunk {
    pub voxels: VoxelList,
    pub position: ChunkPosition,
    pub blocks_to_add: HashMap<VoxelPosition, VoxelID>,
}

impl Chunk {
    // Turns 3D coordinates into an index in a 32x32x32 array
    pub fn coordinates_to_index(coordinate: VoxelPosition) -> usize {
        return ((CHUNK_SIZE * CHUNK_SIZE * coordinate.y) + (CHUNK_SIZE * coordinate.z) + coordinate.x) as usize
    }
    // Checks if a coordinate is out of bounds
    pub fn coordinate_out_of_bounds(coordinate: VoxelPosition) -> bool {
        coordinate.x < 0               || coordinate.y < 0               || coordinate.z < 0               ||
        coordinate.x > CHUNK_SIZE_MIN1 || coordinate.y > CHUNK_SIZE_MIN1 || coordinate.z > CHUNK_SIZE_MIN1 
    }
    // Turns an index in a 32x32x32 array to 3D coordinates
    pub fn index_to_coordinates(index: usize) -> VoxelPosition {
        return glam::IVec3 {
            y: (index / (CHUNK_SIZE_USIZE * CHUNK_SIZE_USIZE)) as i32,
            z: ((index / CHUNK_SIZE_USIZE).rem_euclid(CHUNK_SIZE_USIZE)) as i32,
            x: (index.rem_euclid(CHUNK_SIZE_USIZE))        as i32,
        }
    }
    // These get and set functions should only be called by the chunk_manager!
    // Returns the voxel at the specified index
    pub fn get_voxel_from_index(&self, index: usize) -> VoxelID {
        assert!(index < CHUNK_SIZE_USIZE*CHUNK_SIZE_USIZE*CHUNK_SIZE_USIZE);
        return self.voxels[index];
    }
    // Returns the voxel at the specified coordinates
    pub fn get_voxel_from_coordinate(&self, coordinate: VoxelPosition) -> VoxelID {
        return self.get_voxel_from_index(Chunk::coordinates_to_index(coordinate))
    }
    // Sets the voxel at the specified index
    pub fn set_voxel_from_index(&mut self, index: usize, voxel_id: VoxelID) {
        assert!(index < CHUNK_SIZE_USIZE*CHUNK_SIZE_USIZE*CHUNK_SIZE_USIZE, "Tried to check out of bounds block in chunk!! Index: {:?}, Block: {:?}", index, voxel_id);
        self.voxels[index] = voxel_id;
    }
    // Sets the voxel at the specified coordinates
    pub fn set_voxel_from_coordinate(&mut self, coordinate: VoxelPosition, voxel_id: VoxelID) {
        self.set_voxel_from_index(Chunk::coordinates_to_index(coordinate), voxel_id);
    }
}

// For conversions
pub struct Convert {} impl Convert {
    pub fn local_to_global(chunk_position: ChunkPosition, local_voxel_pos: VoxelPosition) -> VoxelPosition {
        (chunk_position*CHUNK_SIZE)+local_voxel_pos
    }
    pub fn global_to_local(global_coord: VoxelPosition) -> VoxelPosition {
        VoxelPosition::new(
            global_coord.x.rem_euclid(CHUNK_SIZE),
            global_coord.y.rem_euclid(CHUNK_SIZE),
            global_coord.z.rem_euclid(CHUNK_SIZE),)
    }
    pub fn global_to_chunk(global_coord: VoxelPosition) -> ChunkPosition {
        // TODO: There's probably a better way of doing this without floating point math
        ChunkPosition::new(
            (global_coord.x as f32/(CHUNK_SIZE as f32)).floor() as i32,
            (global_coord.y as f32/(CHUNK_SIZE as f32)).floor() as i32,
            (global_coord.z as f32/(CHUNK_SIZE as f32)).floor() as i32,)
    }
}