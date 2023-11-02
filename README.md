![Alt text](img/banner.png)

# Tank - Game development library
An ECS library of all the generic elements of the games I work on. Built on top of the Bevy game engine using Rust.

Currently not very user friendly, and the majority of implementations are naive, as my focus is on getting a game published.

**As of November 2023, this is basically unuseable for anyone but me.** <span style="font-size:8px">It's not even very useable for me.</span> You may find some value in examining my implementations if you're new to ECS, but I wouldn't recommend using the library yet. Examples & build guide will be added when the library causes less mental damage to its users.

## Architecture

### 1. AI
- The newest part of the library. I don't know anything about programming AI so I'm still just kinda shooting in the dark here and seeing what sticks.

### 2. Audio
- Thin wrapper for bevy_kira_audio.
- Issues:
    - Needs better audio library, or audio from scratch.
    - Spatial audio implementation is poor.
    - Ability to alter sounds is poor. (Pitch, Reverb, low/high pass filtering, etc.)
    - I implemented a much better audio system which allowed for proper spatial audio, sound editing, etc. using kira_audio directly, but it crashes once you've played ~50 sounds. It's in archive for now until I have time to figure out what I messed up.

### 3. Camera
- Some Camera control stuff. Works decently well, but no smoothing. 
- Implementation feels hacky, but I did get it to work with Rapier so that cameras can track global transforms of entities without any jitter. (Importantly, this includes physics entities).
- Orbit camera works as both 3rd person and FPS controller. Just set the zoom to 0 for FPS.

### 4. Gui
- Bevy's UI doesn't allow for using multiple windows. Egui doesn't allow for pixel perfect image display. Currently this is a kind've hacked together approach that uses Bevy's RenderLayers along with Camera2d to display ui using sprites.
- Issues:
    - Performs fine but this is scuffed. Try imgui? Or anything else?

### 5. Input
- I'm not a fan of Bevy's input system, this is an attempt to do something I like more + extend it to allow for bindings, but implementation falls flat.
- Currently allows for assigning input bindings and trigger states (pressed, released, held) to values of a user created input actions enum. Players can then be assigned devices through InputDeviceReceiver.
- Implementation is pretty derpy. Needs a refactor. Creating separate bindings for the same key to detect held/pressed/released is dumb.

### 6. Networking
- Doesn't exist at the moment. Literally just example code from a networking library I was looking at.

### 7. Packages
- Gathers up all assets from hardcoded folders (fonts, images, models, sounds).
- Loads ALL assets, so this is a major bottleneck on game scale. Will need to be refactored to allow for contextual loading. And control over what is loaded when. Ideally I want to be able to assign sets of assets to groups that can be loaded/unloaded via events.

### 8. Player
- A Player Controller. Just an invisible Entity that handles connections to the camera, input, controlled actors, etc...

### 9. Profile
- For saving/loading game settings. And probably game data too. I have an old (bad) implementation that I still need to refactor & port over.

### 10. State
- EngineInit, GameInit, and Main states. The game using Tank is meant to advance the GameInit state to the Main state.

### 11. Thing
- A loose object in the level.

#### 11.1. Action
- Actions a thing can carry out. Building, issuing commands... complex behaviors.

#### 11.2. Emitter
- Guns, thrusters, spawners.

#### 11.3. Item
- Things to pick up, equip, use.

#### 11.4. Movement
- Things that move themselves.

#### 11.5. Part
- Things that can be attached or detached. Like limbs, or a hat...?

#### 11.6. Projectile
- Things that explode on contact. Or perform segmented hitscans over a certain distance.

#### 11.7. Turret
- This will probably get split up into the above categories. Where do these belong?
    - A component that shoots a projectile if it has a valid target it doesn't like.
    - A component that rotates closest to facing a target along only one local axis.

### 12. Util

#### 12.1. Generics
- Components that could be used anywhere on anything.

#### 12.2. Mesh
- Some mesh generation (from voxel/heightmap data) and gltf loading.

#### 12.3. Everything Else
- Bitflag: Old, to be removed or refactored.
- Bitmask: Gods gift to programmers, implementation adapted from OpenVDB.
- Bundle: A single convenience bundle for adding Visibility and Transform to empties.
- Database: Old, to be removed or refactored.
- Image: Meant to be for image generation. Currently just generates a texture for a heightmap.
- Math: Whatever Bevy or Rust lacks (or I couldn't find).
- Noise: Currently just generates perlin 2d heightmaps.
- Serial: Meant to make loading/saving ron files effortless. Extremely old. I think Packages use it. Needs a refactor.
- Thread: Meant to make multithreaded operations effortless. Not used anywhere anymore. Probably needs a refactor.

### 13. Voxel
- The oldest part of the library, and a mess. Half of the code in here doesn't work anymore. Don't use this; probably don't even look at this.

## Notes
- Just get rid of any idea of `Actors`? All game objects are just `Things` with different components. Terrain is special because it's static/fixed (heightmap, voxels, editable or not, etc.), but debris from terrain could be `Things`.
- `Things` that are definitely `Things`:
    - Humanoids
    - Items (physically in the level, not data in your inventory)
    - Projectiles
    - Turrets
    - Vehicles
- If you have a lever, are the base and the stick separate `Thing`s? Verdict: Yes
    - They should be usable as a single `Thing` through the base (as developer). But you should be able to pull off the stick too. If needed.
- Are buildings `Things`? Verdict: Sometimes
    - They behave more like terrain, but they aren't joined with the terrain's data structure (voxel/heightmap array), so really they're more like mini terrains.
        - Though they could be, as in games like Minecraft where there is no distinction between building and terrain.
    - Buildings could have potentially similar data structures to terrain (buildings made from voxels, imagine the ores shoved into the heightmap in Valheim) or be closer to characters/projectiles (single mesh, or a few connected meshes). The possibility for both in the same game should remain open...
    - And while most buildings are going to be static like the terrain, you could imagine a tower with its base blown up; the top half would become a physics object, in which case it is most definitely a `Thing`. But then it's just the same as terrain, a static object which creates `Thing`s as debris...
- Why the distinction?
    - `Actors` were for all the components that a player or an AI would control via input components... but isn't that too broad?
    - A `Projectile` might have a vector for direction and magnitude, but even something like that could potentially be an `Actor`. Like a remote control rocket. Or a magic bullet you control in slow motion. So would you have a marker component that says 'edit the projectile values based on MoveInput(3d/2d)'? And would that marker component belong in the `Projectile` files or in `Actor` files?