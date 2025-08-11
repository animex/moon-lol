mod enums;
mod skinned;
mod static_mesh;
mod types;

pub use enums::*;
pub use skinned::*;
pub use static_mesh::*;
pub use types::*;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LeagueMaterial {
    pub texture_path: String,
}
