use egui::Context;
use egui::Frame;
use egui::Margin;

use crate::choreo_main;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::material::styling::material_palette::MaterialPalette;
use crate::material::styling::material_palette::material_palette_for_theme;
use crate::material::styling::material_palette::with_current_material_palette;

use super::state::AppShellState;

pub fn draw_splash(context: &Context, state: &AppShellState, main_page_state: &ChoreoMainState) {
    let palette = material_palette_for_theme(
        &main_page_state.settings_state.material_scheme,
        main_page_state.settings_state.theme_mode,
    );
    egui::CentralPanel::default()
        .frame(root_panel_frame(palette))
        .show(context, |ui| {
            crate::splash_screen_host::ui::draw(ui, &state.splash_screen_state);
        });
}

pub fn draw_main_page(
    context: &Context,
    main_page_state: &ChoreoMainState,
) -> Vec<ChoreoMainAction> {
    let mut actions = Vec::new();
    let palette = material_palette_for_theme(
        &main_page_state.settings_state.material_scheme,
        main_page_state.settings_state.theme_mode,
    );

    egui::CentralPanel::default()
        .frame(root_panel_frame(palette))
        .show(context, |ui| {
            with_current_material_palette(palette, || {
                actions = choreo_main::ui::draw(ui, main_page_state);
            });
        });

    actions
}

fn root_panel_frame(palette: MaterialPalette) -> Frame {
    Frame::new()
        .fill(palette.background)
        .inner_margin(Margin::same(0))
}
