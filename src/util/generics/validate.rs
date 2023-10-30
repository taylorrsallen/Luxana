use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub trait Validate {
    fn validate(&mut self);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct ValidateComponentPlugin<T: Component + Validate>(PhantomData<T>);
impl<T: Component + Validate> Plugin for ValidateComponentPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, sys_validate_component::<T>);
    }
}

fn sys_validate_component<T: Component + Validate>(mut validate_query: Query<&mut T, Changed<T>>) {
    for mut validate in validate_query.iter_mut() { validate.validate(); }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Default)]
pub struct ValidateResourcePlugin<T: Resource + Validate>(PhantomData<T>);
impl<T: Resource + Validate> Plugin for ValidateResourcePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, sys_validate_resource::<T>.run_if(resource_changed::<T>()));
    }
}

fn sys_validate_resource<T: Resource + Validate>(mut validate: ResMut<T>) {
    validate.validate();
}