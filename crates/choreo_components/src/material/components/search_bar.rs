use std::borrow::Cow;

use egui::Color32;
use egui::CornerRadius;
use egui::Id;
use egui::Image;
use egui::Layout;
use egui::PopupCloseBehavior;
use egui::Response;
use egui::Sense;
use egui::TextEdit;
use egui::Ui;
use egui::UiBuilder;
use egui::vec2;

use crate::material::components::divider::HorizontalDivider;
use crate::material::components::icon::MaterialIconStyle;
use crate::material::components::icon::icon_with_style;
use crate::material::components::icon_button::MaterialIconButton;
use crate::material::components::list::AvatarStyle;
use crate::material::components::list::ListTile;
use crate::material::components::list::avatar;
use crate::material::components::list_view::ListView;
use crate::material::components::material_text::MaterialTextOverflow;
use crate::material::components::material_text::material_text;
use crate::material::components::scroll_view::ScrollViewState;
use crate::material::items::list_item::ListItem;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

const SEARCH_BAR_CORNER_RADIUS: u8 = 28;

#[derive(Default)]
pub struct SearchBarState {
    pub popup_list: ScrollViewState,
}

pub struct SearchBarResponse {
    pub response: Response,
    pub text_changed: bool,
    pub accepted: bool,
    pub activated_item: Option<usize>,
    pub action_button_clicked: Option<usize>,
    pub popup_open: bool,
}

pub struct SearchBar<'a> {
    pub id: Id,
    pub leading_icon: Option<Image<'static>>,
    pub trailing_icon: Option<Image<'static>>,
    pub avatar_icon: Option<Image<'static>>,
    pub avatar_background: Option<Color32>,
    pub placeholder_text: Cow<'a, str>,
    pub empty_text: Cow<'a, str>,
    pub text: &'a mut String,
    pub current_index: Option<usize>,
    pub items: &'a [ListItem],
    pub enabled: bool,
    pub width: Option<f32>,
}

impl<'a> SearchBar<'a> {
    #[must_use]
    pub fn new(
        id_source: impl std::hash::Hash,
        text: &'a mut String,
        items: &'a [ListItem],
    ) -> Self {
        Self {
            id: Id::new(id_source),
            leading_icon: Some(default_menu_icon()),
            trailing_icon: None,
            avatar_icon: None,
            avatar_background: None,
            placeholder_text: Cow::Borrowed(""),
            empty_text: Cow::Borrowed(""),
            text,
            current_index: None,
            items,
            enabled: true,
            width: None,
        }
    }

    #[must_use]
    pub fn leading_icon(mut self, leading_icon: Image<'static>) -> Self {
        self.leading_icon = Some(leading_icon);
        self
    }

    #[must_use]
    pub fn trailing_icon(mut self, trailing_icon: Image<'static>) -> Self {
        self.trailing_icon = Some(trailing_icon);
        self
    }

    #[must_use]
    pub fn avatar_icon(mut self, avatar_icon: Image<'static>) -> Self {
        self.avatar_icon = Some(avatar_icon);
        self
    }

    #[must_use]
    pub fn avatar_background(mut self, avatar_background: Color32) -> Self {
        self.avatar_background = Some(avatar_background);
        self
    }

    #[must_use]
    pub fn placeholder_text(mut self, placeholder_text: impl Into<Cow<'a, str>>) -> Self {
        self.placeholder_text = placeholder_text.into();
        self
    }

    #[must_use]
    pub fn empty_text(mut self, empty_text: impl Into<Cow<'a, str>>) -> Self {
        self.empty_text = empty_text.into();
        self
    }

    #[must_use]
    pub fn current_index(mut self, current_index: Option<usize>) -> Self {
        self.current_index = current_index;
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui, state: &mut SearchBarState) -> SearchBarResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let popup_id = self.id.with("popup");
        let width = self.width.unwrap_or_else(|| ui.available_width().max(0.0));
        let min_size = vec2(width, metrics.sizes.size_56);
        let (rect, response) = ui.allocate_exact_size(min_size, Sense::click());

        ui.painter().rect_filled(
            rect,
            CornerRadius::same(SEARCH_BAR_CORNER_RADIUS),
            palette.surface_container_high,
        );

        ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
            let content_rect =
                rect.shrink2(vec2(metrics.paddings.padding_4, metrics.paddings.padding_4));
            ui.scope_builder(UiBuilder::new().max_rect(content_rect), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_4;

                    if let Some(icon) = self.leading_icon.clone() {
                        let _ = ui.add(icon_with_style(
                            icon,
                            MaterialIconStyle {
                                size: vec2(
                                    metrics.icon_sizes.icon_size_24,
                                    metrics.icon_sizes.icon_size_24,
                                ),
                                tint: palette.on_surface_variant,
                            },
                        ));
                    }

                    let collapsed_text = if self.text.is_empty() {
                        self.placeholder_text.clone()
                    } else {
                        Cow::Borrowed(self.text.as_str())
                    };
                    let collapsed_color = if self.text.is_empty() {
                        palette.on_surface_variant
                    } else {
                        palette.on_surface
                    };
                    let _ = material_text(ui, collapsed_text)
                        .text_style(MATERIAL_TYPOGRAPHY.body_large)
                        .color(collapsed_color)
                        .overflow(MaterialTextOverflow::Elide)
                        .show(ui);

                    if let Some(icon) = self.trailing_icon.clone() {
                        let _ = ui.add(icon_with_style(
                            icon,
                            MaterialIconStyle {
                                size: vec2(
                                    metrics.icon_sizes.icon_size_24,
                                    metrics.icon_sizes.icon_size_24,
                                ),
                                tint: palette.on_surface_variant,
                            },
                        ));
                    }

                    if self.avatar_icon.is_some() || self.avatar_background.is_some() {
                        let _ = avatar(
                            ui,
                            self.avatar_icon.clone(),
                            AvatarStyle {
                                background: self.avatar_background.unwrap_or(Color32::TRANSPARENT),
                                size: vec2(metrics.sizes.size_32, metrics.sizes.size_32),
                            },
                        );
                    }
                });
            });
        });

        if self.enabled && response.clicked() {
            egui::Popup::open_id(ui.ctx(), popup_id);
        }

        let mut text_changed = false;
        let mut accepted = false;
        let mut activated_item = None;
        let mut action_button_clicked = None;
        let is_open = egui::Popup::is_id_open(ui.ctx(), popup_id);
        let mut opened_popup = false;

        let _ = egui::Popup::from_response(&response)
            .id(popup_id)
            .open_memory(None)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .align(egui::RectAlign::TOP_START)
            .width(width)
            .show(|ui| {
                opened_popup = true;
                ui.set_min_width(width);

                egui::Frame::new()
                    .fill(palette.surface_container_high)
                    .corner_radius(CornerRadius::same(SEARCH_BAR_CORNER_RADIUS))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_4;

                            let back_button = MaterialIconButton::new(default_back_icon()).show(ui);
                            if back_button.response.clicked() {
                                egui::Popup::close_id(ui.ctx(), popup_id);
                            }

                            let text_edit_id = self.id.with("input");
                            let text_response = ui.add_sized(
                                vec2(
                                    (width
                                        - metrics.sizes.size_40 * 2.0
                                        - metrics.spacings.spacing_4 * 2.0
                                        - metrics.paddings.padding_8)
                                        .max(0.0),
                                    metrics.sizes.size_40,
                                ),
                                TextEdit::singleline(self.text)
                                    .id(text_edit_id)
                                    .frame(false)
                                    .hint_text(self.placeholder_text.as_ref()),
                            );
                            text_changed = text_response.changed();
                            accepted = text_response.lost_focus()
                                && ui.input(|input| input.key_pressed(egui::Key::Enter));
                            if is_open {
                                ui.memory_mut(|memory| memory.request_focus(text_edit_id));
                            }

                            let clear_button =
                                MaterialIconButton::new(default_close_icon()).show(ui);
                            if clear_button.response.clicked() {
                                self.text.clear();
                                text_changed = true;
                                egui::Popup::close_id(ui.ctx(), popup_id);
                            }
                        });

                        let _ = ui.add(HorizontalDivider);

                        if self.items.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(metrics.spacings.spacing_16);
                                let _ = material_text(ui, self.empty_text)
                                    .text_style(MATERIAL_TYPOGRAPHY.label_small)
                                    .color(palette.on_surface_variant)
                                    .show(ui);
                                ui.add_space(metrics.spacings.spacing_16);
                            });
                        } else {
                            let max_height =
                                metrics.sizes.size_72 * popup_visible_rows(self.items.len()) as f32;
                            let _ = ui.allocate_ui_with_layout(
                                vec2(width, max_height),
                                Layout::top_down(egui::Align::Min),
                                |ui| {
                                    let _ = ListView::new(&mut state.popup_list).show(ui, |ui| {
                                        for (index, item) in self.items.iter().enumerate() {
                                            let tile_response =
                                                show_search_tile(ui, item, self.enabled);
                                            if tile_response.action_clicked {
                                                action_button_clicked = Some(index);
                                            }
                                            if tile_response.response.clicked() {
                                                activated_item = Some(index);
                                                egui::Popup::close_id(ui.ctx(), popup_id);
                                            }
                                        }
                                    });
                                },
                            );
                        }
                    });
            });

        SearchBarResponse {
            response,
            text_changed,
            accepted,
            activated_item,
            action_button_clicked,
            popup_open: is_open || opened_popup,
        }
    }
}

struct SearchTileResponse {
    response: Response,
    action_clicked: bool,
}

fn show_search_tile(ui: &mut Ui, item: &ListItem, enabled: bool) -> SearchTileResponse {
    let mut action_clicked = false;
    let response = ListTile {
        text: Cow::Borrowed(item.text.as_str()),
        supporting_text: Cow::Borrowed(item.supporting_text.as_str()),
        avatar_icon: item.avatar_icon.clone(),
        avatar_text: Cow::Borrowed(item.avatar_text.as_str()),
        avatar_background: item.avatar_background,
        avatar_foreground: item.avatar_foreground,
        enabled,
        color: None,
        tooltip: Cow::Borrowed(""),
    }
    .show(ui, |ui| {
        if let Some(icon) = item.action_button_icon.clone() {
            let action_response = MaterialIconButton::new(icon).show(ui);
            action_clicked = action_response.response.clicked();
        }
    });
    SearchTileResponse {
        response,
        action_clicked,
    }
}

#[must_use]
pub const fn popup_visible_rows(item_count: usize) -> usize {
    if item_count == 0 {
        3
    } else if item_count > 6 {
        6
    } else if item_count < 3 {
        3
    } else {
        item_count
    }
}

fn default_menu_icon() -> Image<'static> {
    Image::new(egui::include_image!("../../../assets/icons/Menu.svg"))
}

fn default_back_icon() -> Image<'static> {
    Image::new(egui::include_image!("../../../assets/icons/ArrowLeft.svg"))
}

fn default_close_icon() -> Image<'static> {
    Image::new(egui::include_image!("../../../assets/icons/Close.svg"))
}

#[cfg(test)]
mod tests {
    use egui::Color32;
    use egui::Context;
    use egui::Image;

    use super::SearchBar;
    use super::SearchBarState;
    use super::popup_visible_rows;
    use crate::material::items::list_item::ListItem;

    #[test]
    fn popup_row_count_clamps_between_three_and_six_items() {
        assert_eq!(popup_visible_rows(0), 3);
        assert_eq!(popup_visible_rows(1), 3);
        assert_eq!(popup_visible_rows(3), 3);
        assert_eq!(popup_visible_rows(5), 5);
        assert_eq!(popup_visible_rows(9), 6);
    }

    #[test]
    fn collapsed_search_bar_renders_without_panicking() {
        let context = Context::default();
        let mut text = String::new();
        let items = [ListItem::new("Result")];
        let mut state = SearchBarState::default();
        let mut positive = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = SearchBar::new("search", &mut text, &items)
                    .placeholder_text("Search")
                    .empty_text("Nothing found")
                    .avatar_background(Color32::LIGHT_BLUE)
                    .show(ui, &mut state);
                positive = response.response.rect.height() >= 56.0;
            });
        });
        assert!(positive);
    }

    #[test]
    fn search_tile_supports_action_icons() {
        let context = Context::default();
        let mut text = String::new();
        let mut item = ListItem::new("Result");
        item.supporting_text = String::from("Supporting");
        item.action_button_icon = Some(Image::new(egui::include_image!(
            "../../../assets/icons/Close.svg"
        )));
        let items = [item];
        let mut state = SearchBarState::default();
        let mut positive = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = SearchBar::new("search-tile", &mut text, &items)
                    .placeholder_text("Search")
                    .empty_text("Nothing found")
                    .trailing_icon(Image::new(egui::include_image!(
                        "../../../assets/icons/Magnify.svg"
                    )))
                    .show(ui, &mut state);
                positive = response.response.rect.width() > 0.0;
            });
        });
        assert!(positive);
    }
}
