![Alt text](img/banner.png)

# Tank - Game development library
An ECS library of all the generic elements of the games I work on. Built on top of the Bevy game engine using Rust.

Currently not very user friendly, and the majority of implementations are naive, as my focus is on getting a game published.

**As of November 2023, this is basically unuseable for anyone but me.** <span style="font-size:8px">It's not even very useable for me.</span> You may find some value in examining my implementations if you're new to ECS, but I wouldn't recommend using the library yet. Examples & build guide will be added when the library causes less mental damage to its users.

## Architecture

### 1. AI
- I don't know anything about programming AI so I'm still just kinda shooting in the dark here and seeing what sticks. May incorporate bevy_big_brain at some point.

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
- TODO:
    - [x] Splitscreen
    - [ ] Multiwindow

### 4. Gui
- Bevy's UI doesn't allow for using multiple windows. Egui doesn't allow for pixel perfect image display. Currently this is a kind've hacked together approach that uses Bevy's RenderLayers along with Camera2d to display ui using sprites.
- TODO:
    - [ ] Gui
        - [ ] Store guis as data, mark the players they will be displayed for, and create separate gui objects for each display which are linked to the data
    - [ ] Buttons
        - [ ] Sprites for different button states
        - [ ] Selector
    - [ ] Display the same gui to multiple players
    - [ ] Multiple player cursors per gui
    - [ ] Layouts
    - [ ] Refactor using Bevy's UI code, but for multiple windows
- Issues:
    - Performs fine but this is scuffed. Try imgui? Alter Bevy's UI code? Or anything else?

### 5. Input
- I'm not a fan of Bevy's input system, this is an attempt to do something I like more + extend it to allow for bindings, but implementation falls flat.
- Currently allows for assigning input bindings and trigger states (pressed, released, held) to values of a user created input actions enum. Players can then be assigned devices through InputDeviceReceiver.
- TODO:
    - [ ] InputActions created using a .ron instead of an enum
        - [ ] Parse InputActions file, storing InputAction names as strings in a HashMap associated with an input value
        - [ ] Check for InputAction names as strings and get rid of need for enums
    - [ ] Store state of an input (held, pressed, released) generically instead of only in an action looking for the state
- Issues:
    - Implementation is pretty derpy. Needs a refactor. Creating separate bindings for the same key to detect held/pressed/released is dumb.

### 6. Level
- The entities which will represent the game world. Called Level because Bevy uses World, and universe is too long.
- TODO:
    - [ ] Heightmap
        - > Does this belong here? Isn't this more of a raw data structure than a game world impl?
        - [ ] Fix lighting artifacts on chunk edges from incomplete normals calculations
    - [ ] Overworld
        - [ ] Landmarks to travel between
        - [ ] Paths represented as series of lines or curves between Landmarks

### 7. Packages
- Gathers up all assets from hardcoded folders (fonts, images, models, sounds).
- Loads ALL assets, so this is a major bottleneck on game scale. Will need to be refactored to allow for contextual loading. And control over what is loaded when. Ideally I want to be able to assign sets of assets to groups that can be loaded/unloaded via events.

### 8. Player
- A Player Controller. Just an invisible Entity that handles connections to the camera, input, controlled actors, etc...
- TODO:
    - [x] Framework for handling splitscreen & multi-window

### 9. Profile
- For saving/loading game settings. And probably game data too. I have an old (bad) implementation that I still need to refactor & port over.
- TODO:
    - [ ] Port profile code from yveB

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
- TODO:
    - [ ] Load in parts from .glb files
        - > I would love to use armature's for this, but Bevy stalls (no error msg) when trying to load a gltf with armatures
        - [x] Put every object not containing the word "Socket" or "Hitbox" in a HashMap of parts, along with its meshes and materials
        - [x] Put every object containing the word "Socket" in a sockets Vec
        - [x] Put every object containing the word "Hitbox" in a hitbox Vec
        - [x] For each socket, for each word that isn't Socket in the socket's name ("SocketChestStomach" = ["Chest", "Stomach"]), add the socket location to those entries in the parts HashMap
        - [x] For each hitbox, grab the word that isn't Hitbox ("HitboxChest" = "Chest") and assign the Hitbox to that entry in the parts HashMap
        - [ ] For each part in the HashMap, assign the next available PartDef id, add it to PartDefs, job done
            - [ ] Possibly store the name of the part as "FileName + PartName" in the PartDef
            - [ ] Possibly store PartDefs within BodyDefs, if they came in a file which defines an entire body (some files might just be a bunch of arms)
    - [ ] 

#### 11.6. Projectile
- Things that explode on contact. Or perform segmented hitscans over a certain distance.

#### 11.7. Turret
- This will probably get split up into the above categories. Where do these belong?
    - A component that shoots a projectile if it has a valid target it doesn't like
        - AI component + an interactor controller component (Is this trigger being pulled? Yes | No)
    - A component that rotates closest to facing a target along only one local axis
        - This is IK

### 12. Util

#### 12.1. Generics
- Components that could be used anywhere with anything.

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
- TODO:
    - [ ] Rename to "DataStructures" or something similar
        - > Data refers to save data in Tank. Too vague to just call these Structures? They *are* data though. I mean techhnically it's *all* data.
    - [ ] Refactor so that usage is easier
    - [ ] Fix broken math in Root3d

## Notes
- Just get rid of any idea of `Actor`s?
    - > No
    - All game objects are just `Thing`s with different components. `Terrain` is special because it's static/fixed (heightmap, voxels, editable or not, etc.), but debris from terrain could be `Thing`s.
    - The concept of an `Actor` is helpful for organizing `Action` execution. For now, I'm using `Actor` to store the list of direct interactors available on a `Thing` so that a full hierarchy traversal is only needed when the `Thing` loses or gains a child. The presence of an `Actor` component thus marks an entity capable of `Action`s.
- `Thing`s that are definitely `Thing`s:
    - Humanoids
    - Items (physically in the level, not data in your inventory)
    - Projectiles
    - Turrets
    - Vehicles
- If you have a lever, are the base and the stick separate `Thing`s?
    - > Yes
    - They should be usable as a single `Thing` through the base (as developer). But you should be able to pull off the stick too. If needed.
- Are `Building`s `Thing`s?
    - > They're `Terrain`
    - They behave more like `Terrain`, but they aren't joined with the `Terrain`'s data structure (voxel/heightmap array), so really they're mini `Terrain`s.
        - Though they could be part of the `Terrain`, as in games like Minecraft where there is no distinction between `Building` and `Terrain`.
    - `Building`s could have potentially similar data structures to `Terrain` (`Building`s made from voxels, imagine the ores shoved into the heightmap in Valheim) or be closer to `Actor`s/`Projectile`s (single mesh, or a few connected meshes). The possibility for both in the same game should remain open...
    - And while most `Building`s are going to be static like the `Terrain`, you could imagine a tower with its base blown up; the top half would become a physics object, in which case it is most definitely a `Thing`. But then it's just the same as `Terrain`, a static object which creates `Thing`s as debris...
- What is `Terrain`?
    - > An entity with a data structure that can be used to create a mesh + collider, with a fixed rigidbody.
- Why the distinction?
    - All of these classifications of game objects are just architectural concepts for component & code organization in ECS. `Thing`s, `Building`s, `Terrain`; none of these actually technically exist and one Entity could be any combination of multiple. For instance, `Vehicle`s will probably be `Thing`s that use `Terrain`-like data structures for their meshes & colliders.

- Shaders
    - Require a custom material with a hardcoded function to return the file location of the shader, in the assets folder of the project using the library. I was hoping to have built in shaders like clouds & water & fog volumes, but I think I may have to wait until Bevy updates how they handle shaders. I *could* make a plugin that adds a shader material and creates the shader file in the 'shaders' directory within 'assets', but that feels intrusive.