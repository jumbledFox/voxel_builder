use glam;

pub type VoxelID = u8;
pub type VoxelList = [VoxelID; 32*32*32];

// Chunk - 32x32x32 array of voxels
pub struct Chunk {
    pub voxels: VoxelList,
}

impl Chunk {
    pub fn new() -> Self {
        Self { voxels: [0; 32*32*32]}
    }
    // Turns 3D coordinates into an index in a 32x32x32 array
    pub fn coordinates_to_index(coordinate: glam::U64Vec3) -> usize {
        return ((32 * 32 * coordinate.z) + (32 * coordinate.y) + coordinate.x) as usize
    }
    // Turns an index in a 32x32x32 array to 3D coordinates
    pub fn index_to_coordinates(index: usize) -> glam::U64Vec3 {
        return glam::U64Vec3 {
            x: (index / (32 * 32)) as u64,
            y: ((index / 32) % 32) as u64,
            z: (index % 32)        as u64,
        }
    }
    // Returns the voxel at the specified index, doesn't check bounds - unsafe!
    pub fn get_voxel_index(&mut self, index: usize) -> VoxelID {
        return self.voxels[index];
    }
    // Returns the voxel at the specified coordinates, doesn't check bounds - unsafe!
    pub fn get_voxel_coordinate(&mut self, coordinate: glam::U64Vec3) -> VoxelID {
        return self.voxels[Chunk::coordinates_to_index(coordinate)];
    }
    // Sets the voxel at the specified index, doesn't check bounds - unsafe!
    pub fn set_voxel_index(&mut self, index: usize, voxel_id: VoxelID) {
        self.voxels[index] = voxel_id;
    }
    // Sets the voxel at the specified coordinates, doesn't check bounds - unsafe!
    pub fn set_voxel_coordinate(&mut self, coordinate: glam::U64Vec3, voxel_id: VoxelID) {
        self.voxels[Chunk::coordinates_to_index(coordinate)] = voxel_id;
    }
}