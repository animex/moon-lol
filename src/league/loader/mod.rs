mod game;
mod map;
mod reader;
mod saver;
mod wad;
mod wad_parse;

pub use game::*;
pub use map::*;
pub use reader::*;
pub use saver::*;
pub use wad::*;
pub use wad_parse::*;

#[derive(thiserror::Error, Debug)]
pub enum LeagueLoaderError {
    #[error("Could not load mesh: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not load texture: {0}")]
    BinRW(#[from] binrw::Error),

    #[error("Could not load texture: {0}")]
    Texture(#[from] bevy::image::TextureError),

    #[error("Could not load prop file: {0}")]
    PropError(#[from] cdragon_prop::PropError),

    #[error("Could not serialize: {0}")]
    Bincode(#[from] bincode::Error),
}
