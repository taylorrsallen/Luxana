use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct TankThingActorPlugin;
impl Plugin for TankThingActorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actor>()
            .register_type::<Grabber>()
            .add_systems(PostUpdate, sys_init_actor_rig);
        
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// A Thing with Interactors, on it and/or as children.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Actor {
    pub interactors: Vec<Entity>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Reflect)]
pub struct GrabInteraction {
    pub entity: Entity,
    /// From origin of grabbed entity.
    pub offset: Vec3,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// An entity that can grab things, LittleBigPlanet style.
/// 
/// Required for equipping [HeldEquippable]s, but does not have the functionality without [EquipSlot].
#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Grabber {
    interaction: Option<GrabInteraction>,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component, Default)]
pub struct EquipSlot;

////////////////////////////////////////////////////////////////////////////////////////////////////
/// A Thing mesh with an armature. 
/// 
/// Because splitting characters into 17 parts and rendering (without instancing) max ~200
/// brought to mind that if they weren't in parts, 3,200 could be rendered without instancing.
/// 
/// And with instancing, the 17 part characters could go up to ~2,500 without lag: meaning that with instancing,
/// single model characters could be up to ~42,500 on screen at once.
/// 
/// The problem is actually the sheer number of models, not their complexity. So further optimizations may be possible
/// using visual distortions to turn far off groups of models into splats of color. LoD on model complexity for my uses
/// doesn't matter since the models already look like typical lowest detail versions when you're right next to them.
/// 
/// Tested on PC:
/// - Ubuntu 22.04 (x86-64)
/// - Linux 6.2.0-36-generic
/// - AMD Ryzen 7 2700X Eight-Core Processor x 8
/// - 15.5 GiB RAM
/// - GeForce GTX 1080
#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct ActorRig {
    pub armature: Option<Entity>,
}

#[derive(Default)]
struct ActorRigSetupData {
    armature: Option<Entity>,
    arm_ends: Vec<Entity>,
    bones: Vec<Entity>,
    hitboxes: Vec<ActorRigHitbox>,
}

struct ActorRigHitbox {
    parent: Entity,
    entity: Entity,
}

impl ActorRig {
    /// Use on a Scene entity with ActorRig attached, which is always an empty container for the scene.
    fn parse(
        entity: Entity,
        children_query: &Query<&Children>,
        name_query: &Query<&Name>,
        setup_data: &mut ActorRigSetupData,
    ) {
        let Ok(children) = children_query.get(entity) else { return };
        for child in children.iter() { Self::parse_recursive(entity, *child, children_query, name_query, setup_data); }
    }

    fn parse_recursive(
        parent_entity: Entity,
        entity: Entity,
        children_query: &Query<&Children>,
        name_query: &Query<&Name>,
        setup_data: &mut ActorRigSetupData,
    ) {
        if let Ok(name) = name_query.get(entity) {
            if name.contains("Bone") {
                setup_data.bones.push(entity);
                if name.contains("ArmEnd") { setup_data.arm_ends.push(entity); }
            } else if name.contains("Hitbox") {
                setup_data.hitboxes.push(ActorRigHitbox { parent: parent_entity, entity });
                return;
            } else if name.contains("Armature") {
                setup_data.armature = Some(entity);
            }
        }
    
        let Ok(children) = children_query.get(entity) else { return };
        for child in children.iter() { Self::parse_recursive(entity, *child, children_query, name_query, setup_data); }
    }
}

#[derive(Component, Debug)]
pub struct ActorLimb {
    upper: Entity,
    lower: Entity,
    end: Entity,
}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct ActorArmIK {

}

#[derive(Component, Default, Debug, Reflect)]
#[reflect(Component, Default)]
pub struct TestTimer {
    pub timer: Timer,
    pub index: u8,
}

fn sys_init_actor_rig(
    mut commands: Commands,
    mut actor_rig_query: Query<(Entity, &mut ActorRig), Added<ActorRig>>,
    mut actor_query: Query<&mut Actor>,
    transform_query: Query<&Transform>,
    children_query: Query<&Children>,
    name_query: Query<&Name>,
) {
    for (entity, mut rig) in actor_rig_query.iter_mut() {
        let mut setup_data = ActorRigSetupData::default();
        ActorRig::parse(entity, &children_query, &name_query, &mut setup_data);

        rig.armature = setup_data.armature;
        if let Some(armature) = setup_data.armature {
            commands.entity(armature).insert(TestTimer { timer: Timer::from_seconds(0.5, TimerMode::Repeating), index: 0 });
        }
        
        if !setup_data.arm_ends.is_empty() { commands.entity(entity).insert(Actor { interactors: setup_data.arm_ends }); }

        println!("Bones: {}, Hitboxes: {}", setup_data.bones.len(), setup_data.hitboxes.len());
        for hitbox in setup_data.hitboxes.iter() {
            let hitbox_transform = transform_query.get(hitbox.entity).unwrap().clone();
            commands.entity(hitbox.entity).despawn_recursive();
            commands.entity(hitbox.parent).with_children(|child_builder| {
                child_builder.spawn(PartHitboxBundle {
                    transform: hitbox_transform,
                    collider: Collider::cuboid(1.0, 1.0, 1.0),
                    ..default()
                });
            });
        }
    }
}