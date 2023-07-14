use std::{collections::HashMap, cmp::Ordering};
use {bracket_noise::prelude::*, bracket_random::prelude::RandomNumberGenerator};

use crate::{chunk::{Convert, Chunk, self, ChunkPosition, VoxelPosition, VoxelID, VoxelList}, voxel_data_manager::VoxelDataManager};

pub struct ChunkManager {
    pub chunks: HashMap<ChunkPosition, Chunk>,
    pub voxel_data_manager: VoxelDataManager,
    noise: FastNoise,
}

impl ChunkManager {
    pub fn new(voxel_data_manager: VoxelDataManager) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut cm = ChunkManager { chunks: HashMap::new(), noise: FastNoise::seeded(rng.next_u64()), voxel_data_manager: voxel_data_manager };
        // Set up noise
        cm.noise.set_noise_type(NoiseType::SimplexFractal);
        cm.noise.set_fractal_type(FractalType::Billow);
        cm.noise.set_interp(Interp::Quintic);
        cm.noise.set_fractal_octaves(5);
        cm.noise.set_fractal_gain(0.6);
        cm.noise.set_fractal_lacunarity(2.0);
        cm.noise.set_frequency(2.0);
        cm
    }
    pub fn add_chunk(&mut self, position: ChunkPosition) {
        let c: Chunk = chunk::Chunk{position:position, voxels: self.get_world_generation(position) };
    
        self.chunks.insert(position, c);
    }
    pub fn get_chunk(&self, position: VoxelPosition) -> Option<&Chunk> {
        self.chunks.get(&position)
    }
    pub fn get_chunk_mut(&mut self, position: VoxelPosition) -> Option<&mut Chunk> {
        self.chunks.get_mut(&position)
    }
    pub fn get_voxel(&self, global_coord: VoxelPosition) -> Option<VoxelID> {
        match self.get_chunk(Convert::global_to_chunk(global_coord)) {
            Some(chunk) => {Some(chunk.get_voxel_from_coordinate(Convert::global_to_local(global_coord)))},
            _ => {None}
        }
    }
    pub fn set_voxel(&mut self, global_coord: VoxelPosition, voxel_id: VoxelID) -> bool {
        match self.get_chunk_mut(Convert::global_to_chunk(global_coord)) {
            Some(chunk) => {
                chunk.set_voxel_from_coordinate(Convert::global_to_local(global_coord), voxel_id); true},
            _ => {false}
        }
    }

    pub fn get_world_generation(&mut self, chunk_pos: ChunkPosition) -> VoxelList {
        // TODO: Fix whatever the fuck this shit is (and make it a seperate file)
        // and also make random numbers part of chunk_manager
        // God, why don't i just do things right the first time?! 
        let mut rng = RandomNumberGenerator::new();

        let mut voxels = chunk::DEFAULT_VOXELS;
        for x in 0..32 {
            for z in 0..32 {
                // Get noise height
                let n = (self.noise.get_noise((x + chunk_pos.x*32) as f32 / 1000.0, (z + chunk_pos.z*32) as f32 / 1000.0) * 50.0) as i32;
                // // If block is in chunk
                // if (n as f32/(chunk::CHUNK_SIZE as f32)).floor() as i32 == chunk_pos.y {
                // }

                for y in 0..32 {
                    let v: VoxelID;
                    let global_y = y + (chunk_pos.y*32);
                    if n < global_y {
                        v = 0;
                    } else if n == global_y {
                        // grass
                        if rng.range(0, 90) == 0 {
                            v = 2;
                            // make tree
                            
                        } else {
                            v = 1;
                        }
                    } else if n <= global_y + 3 {
                        v = 2;
                    } else {
                        v = 3;
                    }

                    voxels[Chunk::coordinates_to_index(glam::ivec3(x, y, z))] = v;
                }    
                //for y in 0..n {
                //}
            }
        }
        //voxels.fill(2);
        voxels
    }

    pub fn is_void(&self, global_coord: VoxelPosition) -> bool {
        let v = self.get_voxel(global_coord);
        v.is_none() || v == Some(0)
    }
}