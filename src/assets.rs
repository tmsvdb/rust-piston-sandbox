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

pub struct Assets<'a> {
    window: &'a PistonWindow,
    images: Vec<ImageAsset>,
    fonts: Vec<FontAsset>,
}

impl <'a >Assets<'a> {
    pub fn new (window: &'a PistonWindow) -> Assets<'a> {
        Assets {
            window: window,
            images: Vec::new(),
            fonts: Vec::new(),
        }
    }

    pub fn load_image (&mut self, file_name: &str) -> &ImageAsset {
        //let window = &mut self.window.factory;
        self.images.push(ImageAsset{
            texture: Texture::from_path(
                &mut self.window.factory.clone(),
                Assets::load_from_file(file_name),
                Flip::None,
                &TextureSettings::new()
            ).unwrap(),
            name: String::from(file_name),
        });
        return &self.images[self.images.len()-1];
    }

    pub fn load_font (&mut self, file_name: &str) -> &FontAsset {
        self.fonts.push(FontAsset {
            glyphs: Glyphs::new(
                Assets::load_from_file(file_name),
                self.window.factory.clone()
            ).unwrap(),
            name: String::from(file_name),
        });
        return &self.fonts[self.fonts.len()-1];
    }

    fn load_from_file(file_name: &str) -> ::std::path::PathBuf {
        let asset: ::std::path::PathBuf = ::find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").expect("folder [assets] not found!");
        let asset: ::std::path::PathBuf = asset.join(file_name);
        if !asset.exists() { panic!("file [{}] not found!", asset.to_str().unwrap()) }
        return asset;
    }
}
