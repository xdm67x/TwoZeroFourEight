use bevy::prelude::*;

/// Score actuel du joueur.
#[derive(Resource, Debug, Default)]
pub struct Score {
    /// Valeur du score.
    pub value: u32,
}

impl Score {
    /// Ajoute des points au score.
    pub fn add(&mut self, points: u32) {
        self.value += points;
    }

    /// Réinitialise le score.
    pub fn reset(&mut self) {
        self.value = 0;
    }
}
