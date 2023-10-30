![Alt text](img/banner.png)

# Tank - Game development library
An ECS library of all the generic elements of the games I work on. Built on top of the Bevy game engine using Rust.

Currently not very user friendly, and the majority of implementations are naive, as my focus is on getting a game published.

**As of October 2023, this is basically unuseable for anyone but me.** <span style="font-size:8px">It's not even very useable for me.</span> You may find some value in examining my implementations if you're new to ECS, but I wouldn't recommend using the library yet.

# Architecture
- AI
    - The newest part of the library. I don't really know anything about programming AI so I'm still just kinda shooting in the dark here and seeing what sticks.
- Audio
    - Thin wrapper for bevy_kira_audio.
    - Issues:
        - Needs better audio library, or do audio from scratch.
        - Spatial audio implementation is poor.
        - Ability to alter sounds is poor. (Pitch, Reverb, low/high pass filtering, etc.)
        - I implemented a much better audio system which allowed for proper spatial audio, sound editing, etc. using kira_audio directly, but it crashes once you've played ~50 sounds. It's in archive for now until I have time to figure out what I messed up.
- Camera
    - Some Camera control stuff. Works decently well, but no smoothing. 
    - Implementation feels hacky, but I did get it to work with Rapier so that cameras can track global transforms of entities without any jerkiness. (Importantly, this includes physics entities).
    - Orbit camera works as both 3rd person and FPS controller. Just set the zoom to 0 for FPS.
- Character
    - Anything that can be controlled.
    - Issues:
        - Change this to actor?
- Gui
    - Bevy's UI doesn't allow for using multiple windows. Egui doesn't allow for pixel perfect image display. Currently this is a kind've hacked together approach that uses Bevy's RenderLayers along with Camera2d to display ui using sprites.
    - Issues:
        - Performs fine but this is scuffed. Try imgui? Or anything else?
- Input
    - I'm not a fan of Bevy's input system, this is an attempt to do something I like more + extend it to allow for bindings, but implementation falls flat.
    - Currently allows for assigning input bindings and trigger states (pressed, released, held) to values of a user created input actions enum. Players can then be assigned devices through InputDeviceReceiver.
    - Implementation is pretty derpy. Needs a refactor. Creating separate bindings for the same key to detect held/pressed/released is dumb.
- Networking
    - Doesn't exist at the moment. Literally just example code from a networking library I was looking at.
- Packages
    - Gathers up all assets from hardcoded folders (fonts, images, models, sounds).
    - Loads ALL assets, so this is a major bottleneck on game scale. Will need to be refactored to allow for contextual loading. And control over what is loaded when. Ideally I want to be able to assign sets of assets to groups that can be loaded/unloaded via events.
- Player
    - A Player Controller. Just an invisible Entity that handles connections to the camera, input, controlled characters, etc...
- State
    - EngineInit, GameInit, and Main states. The game using Tank is meant to advance the GameInit state to the Main state.
- Util
- Voxel
    - This is a mess. Half of the code in here doesn't work. Don't use this; probably don't even look at this.