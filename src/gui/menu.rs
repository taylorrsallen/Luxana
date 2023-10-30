use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct LuxanaGuiMenuPlugin;
impl Plugin for LuxanaGuiMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GuiMenu>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Component, Default, Reflect)]
pub struct GuiMenu {
    pub selection: u32,
    item_count: u32,
    flags: u32,
}

impl GuiMenu {
    pub fn new(selection: u32, item_count: u32, looping: bool) -> Self {
        let mut menu = Self { selection, item_count, flags: 0 };
        menu.set_looping(looping);
        menu
    }

    pub fn move_selection(&mut self, amount: i32) {
        let mut new_selection = self.selection as i32 + amount;
        
        if self.is_looping() {
            if new_selection < 0 { new_selection = (self.item_count - 1) as i32; } else if new_selection >= self.item_count as i32 { new_selection = 0; }
        } else {
            if new_selection < 0 { new_selection = 0; } else if new_selection >= self.item_count as i32 { new_selection = (self.item_count - 1) as i32; }
        }

        self.selection = new_selection as u32;
    }

    pub fn is_looping(&self) -> bool { self.flags & 1 == 1 }
    pub fn set_looping(&mut self, active: bool) { if active { self.flags |= 1 } else { self.flags &= !1 } }
}