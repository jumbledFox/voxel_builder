use crate::chunk;

#[derive(Copy, Clone)]
pub struct ChunkVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub struct ChunkMesh {
    pub vertices: Vec<ChunkVertex>,
    pub indices: Vec<u32>,
}
pub struct ChunkMeshBuilder { }
impl ChunkMeshBuilder {
    pub fn build_chunk_mesh(voxels: &chunk::VoxelList) -> ChunkMesh {
        //let mesh = ChunkMesh { vertices: vec![], indices: vec![] };
        let mut vertices: Vec<ChunkVertex> = vec![];
        let mut indices: Vec<u32> = vec![];
        // Naive approach - Put a cube at every block position
        let mut index_count: u32 = 0;
        for (i, &voxel_id) in voxels.iter().enumerate() {
            if voxel_id == 1 {
                continue;
            }
            let c = chunk::Chunk::index_to_coordinates(i);
            let i = i as u32;
            let x = c.x as f32;
            let y = c.y as f32;
            let z = c.z as f32;
            // One face
            vertices.push(ChunkVertex{position: [x    , y+1.0, z], tex_coords: [0.0, 1.0]});
            vertices.push(ChunkVertex{position: [x+1.0, y+1.0, z], tex_coords: [1.0, 1.0]});
            vertices.push(ChunkVertex{position: [x    , y    , z], tex_coords: [0.0, 0.0]});
            vertices.push(ChunkVertex{position: [x+1.0, y    , z], tex_coords: [1.0, 0.0]});
            indices.append(&mut vec![index_count+0, index_count+1, index_count+2, index_count+1, index_count+3, index_count+2]);
            index_count += 4;
        }
        ChunkMesh { vertices: vertices, indices: indices }
    }
}