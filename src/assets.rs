use piston_window::*;
use find_folder::*;
use std::path::*;

pub struct FontAsset {
    pub glyphs: Glyphs,
    pub name: String,
}

pub struct ImageAsset{
    pub texture : G2dTexture,
    pub name: String,
}

pub struct Assets {}

impl Assets {
    pub fn load_image (window: &PistonWindow, file_name: &str) -> ImageAsset {
        ImageAsset {
            texture: Texture::from_path(
                &mut window.factory.clone(),
                Assets::load_from_file(file_name),
                Flip::None,
                &TextureSettings::new()
            ).unwrap(),
            name: String::from(file_name),
        }
    }

    pub fn load_font (window: &PistonWindow, file_name: &str) -> FontAsset {
        FontAsset {
            glyphs: Glyphs::new(
                Assets::load_from_file(file_name),
                window.factory.clone()
            ).unwrap(),
            name: String::from(file_name),
        }
    }

    fn load_from_file(file_name: &str) -> ::std::path::PathBuf {
        let asset: ::std::path::PathBuf = ::find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").expect("folder [assets] not found!");
        let asset: ::std::path::PathBuf = asset.join(file_name);
        if !asset.exists() { panic!("file [{}] not found!", asset.to_str().unwrap()) }
        return asset;
    }
}
