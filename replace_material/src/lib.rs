use bevy::{gltf::GltfMaterialName, prelude::*, scene::SceneInstanceReady};

pub mod prelude {
    pub use crate::{ReplaceMaterial, ReplaceMaterialPlugin};
}

pub struct ReplaceMaterialPlugin;

impl Plugin for ReplaceMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(replace_material);
    }
}

#[derive(Component, Debug)]
pub struct ReplaceMaterial(String, Handle<StandardMaterial>);

impl ReplaceMaterial {
    pub fn new(name: impl Into<String>, material: Handle<StandardMaterial>) -> Self {
        Self(name.into(), material)
    }
}

fn replace_material(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    replace_material: Query<&ReplaceMaterial>,
    mesh_materials: Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
) {
    let Ok(replace_material) = replace_material.get(scene_ready.entity) else {
        return;
    };
    for descendant in children.iter_descendants(scene_ready.entity) {
        if mesh_materials.get(descendant).is_err() {
            continue;
        };
        let Ok((_, material_name)) = mesh_materials.get(descendant) else {
            continue;
        };

        if material_name.0.as_str() == replace_material.0.as_str() {
            commands
                .entity(descendant)
                .insert(MeshMaterial3d(replace_material.1.clone()));
        }
    }
}
