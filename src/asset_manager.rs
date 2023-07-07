extern crate alloc;
use alloc::string::String;
use std::collections::HashMap;
use macroquad::prelude::*;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "data/"]
pub struct ExeFS;

pub struct AssetManager{
    pub textures: HashMap<String,Texture2D>
}

impl AssetManager{
    //run once on init. Any "fancy" file types will get automatically loaded into
    //more runtime suitable structs/things accessible on the struct's maps.
    pub fn load_all(&mut self){
        for file in ExeFS::iter() {
            if file.ends_with(".png"){
                println!("[rs] [assets] loading file as image: {}",file.as_ref());
                let data = ExeFS::get(file.as_ref()).expect("[rs] [assets] something has gone horribly wrong! A listed file proceeded to not exist.");
                let texture = Texture2D::from_file_with_format(
                    data.data.as_ref(),
                    None,
                );
                let id = file.as_ref().replace(".png",""); //ids are cleaned of extensions and other cruft
                self.textures.insert(id,texture);
            }
        }
    }
}