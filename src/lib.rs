use bevy::prelude::*;

#[derive(Default)]
pub struct MagnetitePlugin;

impl Plugin for MagnetitePlugin {
    fn build(&self, _app: &mut App) {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    #[test]
    fn it_works() {
        let _ = super::MagnetitePlugin;
    }
    
    #[test]
    fn can_build_app() {
        App::new()
            .add_plugins(super::MagnetitePlugin)
            .run();
    }
}
