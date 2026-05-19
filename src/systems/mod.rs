//! Module contenant tous les systèmes du jeu 2048.
//!
//! ## Organisation
//!
//! | Fichier | Contenu |
//! |---------|---------|
//! | [`camera`] | Configuration de la caméra 2D |
//! | [`input`] | Capture des touches clavier |
//! | [`game_logic`] | Algorithme 2048 (mouvement + fusion + spawn + win/lose) |
//! | [`rendering`] | Fond de la grille et affichage visuel |
//! | [`ui`] | Score, titre, overlays Game Over / You Win |

mod camera;
pub mod game_logic;
mod input;
pub mod rendering;
pub mod ui;

pub use camera::setup_camera;
pub use input::handle_input;
