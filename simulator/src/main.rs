//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::{prelude::*, asset::LoadState};
use bevy_flycam::prelude::*;
//mod loader;
#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
enum AppState{
    Loading,
    Loaded,
}

impl Default for AppState{
    fn default() -> Self {
        Self::Loading
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_system(loader.in_schedule(OnEnter::<AppState>(AppState::Loading)))
        .add_system(loader_handler.in_schedule(OnUpdate::<AppState>(AppState::Loading)))
        //.add_startup(setup)
        .add_plugin(NoCameraPlayerPlugin)
        .run();
    //let z = 
}

#[derive(Resource)]
struct MyAssets{
    //ambiente: Handle<Scene>,
    sotto_orto: Handle<Scene>,
    //sopra_orto: Handle<Scene>,
   // braccio_orto: Handle<Scene>,
}



fn loader(asset_server: Res<AssetServer>, mut commands: Commands){
    let t = MyAssets{
        sotto_orto: asset_server.load("untitled.glb#Scene0"),
    };
    
    commands.spawn(Camera2d{
        ..Default::default()
    });
    commands.spawn(Text2dBundle{
        text: Text::from_section("loading", TextStyle::default()),
        ..Default::default()
    });

    while asset_server.get_load_state(t.sotto_orto.clone()) != LoadState::Loaded{

    };
    commands.insert_resource(t);
    
}

fn loader_handler(asset_server: Res<AssetServer>, data: Res<MyAssets>, mut commands: Commands){
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FlyCam,
    ));
    
    commands.spawn(SceneBundle {
        scene: asset_server.load("untitled.glb#Scene0"),
        ..default()
    });

}