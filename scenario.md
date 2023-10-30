# The Menu, Inventory, Character, & Player Dilemma
## Attempt 1
- `Character A`, controlled by `Player A`, opens `Chest A`, registering `Character A` as having access to the contents of `Chest A`
- Given the circumstance that `Character A` has access to a chest, the contents should be drawn as a menu (`Menu A`) for `Player A` on their camera, `Camera A`
- `Menu A` is being drawn on `Camera A` via `Player A`'s reference to it, and is drawing the contents of `Chest A` via `Character A`'s reference to it
- If `Player A` switches to `Camera B`, `Menu A` will appear on the new camera
- If `Character A` loses its reference to `Chest A`, then the menu will terminate
- If the menu is terminated, then `Character A` will lose access to `Chest A`
- Menu -> Character -> Chest
- Menu -> Player -> Camera

- ~~Conclusion~~
    - The `Container` component only cares about its own inventory data
    - All `Character`s capable of opening `Container`s must have a component containing a vec of containers they are using
    - `Menu` must track its connected `Player` to access the `Camera` it must be drawn on
    - `Menu` must track its connected `Character` to access the `Container` data it must draw

- Conclusion
    - The `Container` component only cares about its own inventory data
    - Spawn a `Menu` connected to `Player`, the `Character`, and the `Container`
        - Add a `Binding` component to the menu which will despawn it if access to the container is no longer valid
    - The `Menu` is the tool the player can use to interact with `Container` data

## Attempt 2
- `Character A`, receiving input from a `Controller`, opens `Container A`
- If the `Controller` is a `Player`, then `Menu A` is added to that `Player`'s container view list

## AI?
- When an AI wants a Character to open a Container, it polls for the Container it is searching for
    - Via Character perceptions
        - Sight performs a limited area search
        - Memory performs a lookup into the vec of remembered objects
- After finding a valid container, it queues an order for the Character to perform an Action on the Container
- When the Order is ready:
    - As long as the Character cannot perform the desired Action, the AI will attempt to help it reach a state where it can, possibly adding state dependent orders (ex: while state != `Can Perform [Action]`, this order is valid)
    - Likewise for the Character having access to the targeted Container
    - Once the Character can perform the Action (ex: insert 5 potatoes), and they have access to the Container, they will perform the action (which may not be instant)
    - When the Action is completed the Order will be removed

# The Main Menu
- `Animation A` plays and when it finishes slides up, revealing `Area A` beneath
- `Area A` consists of a number of buttons
    - Each button highlights and plays a sound when hovered
    - Each button has a faded highlight when clicked but not yet released
    - To be selected, the same button must be clicked and released

# RTS Unit Movement Dilemma
- `Player A` selects a group of Characters under their control
- This adds the units to a vec of selected units on the Player
- For each selected unit, a selection marker entity is spawned which will follow the character
- They right click empty terrain
- This creates `Order A`:
    - Target: Terrain intersection position from click
    - Action: Move
- `Order A` is then added to an Orders component on all selected units
- Once the units are deselected, their selection markers are despawned

- Conclusion
    - For despawning selection markers to be efficient, they must be tracked somehow
        - A vec of selection markers stored on the player?
        - Reference to selection marker stored on the unit?