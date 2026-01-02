use bevy::prelude::*;

#[derive(Default)]
pub struct MagnetitePlugin;

impl Plugin for MagnetitePlugin {
    fn build(&self, _app: &mut App) {}
}

mod tests {
    #[test]
    fn it_works() {
        let _ = super::MagnetitePlugin::default();
    }
}
