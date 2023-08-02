use bevy::{prelude::*, asset::AssetLoader};

pub struct Loader;



fn loader(asset_server: Res<AssetServer>,  state: Res<State<LoadState>>){
    
    
}

impl Plugin for Loader {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();//State::Loading
        app.add_system(loader.in_schedule(OnEnter::<LoadState>(LoadState::Loading)));

    }
}

