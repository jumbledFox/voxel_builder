use std::collections::HashMap;

use crate::chunk::VoxelID;

pub struct VoxelData {
    pub name: String,
    pub texture_ids: [u32; 6], // left top front right bottom back
}
impl VoxelData {
    pub fn new(name: String, texture_ids: [u32; 6]) -> Self {
        VoxelData { name: name, texture_ids: texture_ids }
    }
}

fn find_in_vec(v: &Vec<&str>, f: &str) -> u32 {
    v.iter().position(|&x| x == f).unwrap() as u32
}

pub struct VoxelDataManager {
    voxel_data: Vec<VoxelData>
}
impl VoxelDataManager {
    pub fn new(in_data: Vec<(&str, Vec<&str>)>, images: &mut Vec<glium::texture::RawImage2d<'_, u8>>) -> Self {
        // Loop through in_data and make vector of images with no dupliates
        let mut image_names: Vec<&str> = vec![];
        for (_, voxel_textures) in &in_data {
            for t in voxel_textures {
                if image_names.contains(&t) { continue; }
                image_names.push(t);
            }
        }
        // Construct actual voxeldata
        let mut voxel_data: Vec<VoxelData> = vec![];
        for (voxel_name, voxel_textures) in in_data {
            let texture_ids: [u32; 6] = match voxel_textures.len() {
                1 => [find_in_vec(&image_names, voxel_textures[0]); 6],
                3 => [  find_in_vec(&image_names, voxel_textures[2]),
                        find_in_vec(&image_names, voxel_textures[0]),
                        find_in_vec(&image_names, voxel_textures[2]),
                        find_in_vec(&image_names, voxel_textures[2]),
                        find_in_vec(&image_names, voxel_textures[1]),
                        find_in_vec(&image_names, voxel_textures[2])
                    ],
                6 => [  find_in_vec(&image_names, voxel_textures[4]),
                        find_in_vec(&image_names, voxel_textures[0]),
                        find_in_vec(&image_names, voxel_textures[2]),
                        find_in_vec(&image_names, voxel_textures[5]),
                        find_in_vec(&image_names, voxel_textures[1]),
                        find_in_vec(&image_names, voxel_textures[3])
                    ],
                _ => [0; 6],
            };
            voxel_data.push(VoxelData { name: voxel_name.to_string(), texture_ids: texture_ids });
        }
        // Load images
        for image_name in &image_names {
            use image::GenericImageView;
            let loaded_image = image::open("res/textures/".to_owned() + &image_name.to_string() + ".png").unwrap();
            let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            &loaded_image.to_rgba8().into_raw(),
            loaded_image.dimensions());
            images.push(image);
        }

        Self { voxel_data: voxel_data }
    }
    
    pub fn get_texture_id(&self, voxel: VoxelID, side: usize) -> u32 {
        self.voxel_data[voxel as usize].texture_ids[side]
    }

    pub fn get_name(&self, voxel: VoxelID) -> String {
        self.voxel_data[voxel as usize].name.clone()
    }
}