use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub(crate) struct EguiWantsFocus(pub HashSet<Entity>);

pub(crate) struct EguiPanCamPlugin;

impl Plugin for EguiPanCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EguiWantsFocus>()
            .add_systems(PostUpdate, check_egui_wants_focus);
    }
}

fn check_egui_wants_focus(
    #[cfg(feature = "bevy_egui_0_39")] mut contexts: Query<(
        Entity,
        &mut bevy_egui_0_39::EguiContext,
    )>,
    #[cfg(feature = "bevy_egui_0_39")] window_to_egui: Res<
        bevy_egui_0_39::input::WindowToEguiContextMap,
    >,
    mut wants_focus: ResMut<EguiWantsFocus>,
) {
    wants_focus.0.clear();

    #[cfg(feature = "bevy_egui_0_39")]
    for (entity, mut ctx) in contexts.iter_mut() {
        let egui_ctx = ctx.get_mut();
        if egui_ctx.wants_pointer_input() || egui_ctx.wants_keyboard_input() {
            if let Some(&window) = window_to_egui.context_to_window.get(&entity) {
                wants_focus.0.insert(window);
            }
        }
    }
}
