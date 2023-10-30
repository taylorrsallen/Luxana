use crate::*;

use std::marker::PhantomData;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait DevName {
    fn dev_name() -> &'static str;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ObjectDevPlugin<T: Component + DevName + Sync + Send + 'static>(PhantomData<T>);

impl<T: Component + DevName + Sync + Send + 'static> Default for ObjectDevPlugin<T> {
    fn default() -> Self { Self { 0: PhantomData } }
}

impl<T: Component + DevName + Sync + Send + 'static> Plugin for ObjectDevPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppDevState::Enabled), onsys_exit_dev::<T>)
            .add_systems(Update, (
                sys_update_dev::<T>,
                sys_update_name::<T>,
            ).run_if(in_state(AppDevState::Enabled)));
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct ObjectDevRoot<T: Component + DevName + Sync + Send + 'static>(PhantomData<T>);

impl<T: Component + DevName + Sync + Send + 'static> Default for ObjectDevRoot<T> {
    fn default() -> Self { Self { 0: PhantomData } }
}

fn onsys_exit_dev<T: Component + DevName + Sync + Send + 'static>(
    mut commands: Commands,
    window_query: Query<Entity, (With<T>, With<Parent>)>,
    root_query: Query<Entity, With<ObjectDevRoot<T>>>,
) {
    for entity in window_query.iter() { commands.entity(entity).remove::<(Name, Parent)>(); }
    for entity in root_query.iter() { commands.entity(entity).despawn(); }
}

fn sys_update_dev<T: Component + DevName + Sync + Send + 'static>(
    mut commands: Commands,
    object_query: Query<Entity, (With<T>, Without<Parent>)>,
    id_query: Query<&Id, (With<T>, Without<Parent>)>,
    root_query: Query<Entity, With<ObjectDevRoot<T>>>,
) {
    if object_query.is_empty() { return; }

    let root_entity = if let Ok(root) = root_query.get_single() {
            root
        } else {
            commands.spawn(ObjectDevRoot::<T>::default())
                .insert(TransformBundle { local: Transform::IDENTITY, global: GlobalTransform::IDENTITY })
                .insert(VisibilityBundle::default())
                .insert(Name::new(T::dev_name().to_string() + "Root"))
                .id()
        };

    let entities: Vec<Entity> = object_query.iter()
        .map(|entity| {
            let mut name = T::dev_name().to_string();
            if let Ok(id) = id_query.get(entity) { name += &("#".to_string() + &id.get().to_string()); }
            commands.entity(entity).insert(Name::new(name)).id()
        })
        .collect();
    
    commands.entity(root_entity).push_children(&entities);
}

fn sys_update_name<T: Component + DevName + Sync + Send + 'static>(
    mut object_query: Query<(&mut Name, &Id), (Changed<Id>, With<T>)>,
) {
    for (mut name, id) in object_query.iter_mut() {
        name.set(T::dev_name().to_string() + "#" + &id.get().to_string());
    }
}