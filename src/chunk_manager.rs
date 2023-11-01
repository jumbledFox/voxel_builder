use std::{collections::HashMap, cmp::Ordering, hash::Hash};
use {bracket_noise::prelude::*, bracket_random::prelude::RandomNumberGenerator};

use crate::{chunk::{Convert, Chunk, self, ChunkPosition, VoxelPosition, VoxelID, VoxelList}, voxel_data_manager::VoxelDataManager};

pub struct ChunkManager {
    pub chunks: HashMap<ChunkPosition, Chunk>,
    pub chunk_voxel_queue: HashMap<ChunkPosition, HashMap<VoxelPosition, VoxelID>>,
    pub voxel_data_manager: VoxelDataManager,
    noise: FastNoise,
}

impl ChunkManager {
    pub fn new(voxel_data_manager: VoxelDataManager) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let mut cm = ChunkManager { chunks: HashMap::new(), noise: FastNoise::seeded(rng.next_u64()), voxel_data_manager: voxel_data_manager, chunk_voxel_queue: HashMap::new()};
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
        let c: Chunk = chunk::Chunk{position:position, voxels: self.get_world_generation(position), blocks_to_add: HashMap::new()};
    
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
        let mut voxels_to_add: HashMap<VoxelPosition, VoxelID> = HashMap::new();
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
                    // } else {
                    //     v = rng.range(1, 11);
                    // }
                    } else if n == global_y {
                        // grass
                        if rng.range(0, 90) == 0 {
                            v = 2;
                            // make tree
                            let trunktype = rng.range(1, 11);
                            for trunkheight in 1..15 {
                                if trunkheight+y >= 32 {
                                    // outisde chunk
                                    let global_block_pos = glam::ivec3(x, trunkheight+y, z) + (chunk_pos * 32);
                                    let outside_chunk_pos = Convert::global_to_chunk(global_block_pos);
                                    if self.chunks.contains_key(&outside_chunk_pos) {
                                        self.set_voxel(global_block_pos, trunktype);
                                    } else {
                                        if !self.chunk_voxel_queue.contains_key(&outside_chunk_pos) {
                                            self.chunk_voxel_queue.insert(outside_chunk_pos, HashMap::new());
                                        }
                                        self.chunk_voxel_queue.get_mut(&outside_chunk_pos).unwrap().insert(Convert::global_to_local(global_block_pos), trunktype);
                                    }
                                } else {
                                    voxels_to_add.insert(glam::ivec3(x, trunkheight+y, z), trunktype);
                                }
                            }
                        } else {
                            v = 1;
                        }
                    } else if n <= global_y + 3 {
                        v = 2;
                    } else if n <= global_y + 32 {
                        v = 3;
                    } else {
                        v = 4;
                    }

                    voxels[Chunk::coordinates_to_index(glam::ivec3(x, y, z))] = v;
                }    
                //for y in 0..n {
                //}
            }
        }
        for (k, v) in voxels_to_add {
            voxels[Chunk::coordinates_to_index(k)] = v;
        }
        if self.chunk_voxel_queue.contains_key(&chunk_pos) {
            for i in self.chunk_voxel_queue.get(&chunk_pos) {
                for (&k, &v) in i {
                    voxels[Chunk::coordinates_to_index(k)] = v;
                }
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