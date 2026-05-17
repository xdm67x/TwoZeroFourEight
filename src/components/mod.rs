//! Module contenant tous les composants du jeu 2048.
//!
//! Les composants représentent les données attachées aux entités (ex: tuiles, positions).
mod position;
mod tile;

pub use position::Position;
pub use tile::Tile;
