use std::borrow::Cow;

use egui::Align;
use egui::Color32;
use egui::Context;
use egui::CornerRadius;
use egui::Layout;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use egui::Vec2;
use egui::vec2;

use crate::components::divider::draw_horizontal_divider;
use crate::components::drawer::Drawer;
use crate::components::drawer::DrawerHeader;
use crate::components::drawer::DrawerPosition;
use crate::components::drawer::ModalDrawer;
use crate::components::icon::MaterialIconStyle;
use crate::components::icon::icon_with_style;
use crate::components::material_text;
use crate::components::material_text::MaterialTextOverflow;
use crate::items::navigation_item::NavigationGroup;
use crate::items::navigation_item::NavigationItem;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::MATERIAL_TYPOGRAPHY;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NavigationSelection {
    pub group_index: usize,
    pub item_index: usize,
}

#[derive(Debug)]
pub struct NavigationItemResponse {
    pub selection: NavigationSelection,
    pub response: Response,
}

pub struct NavigationDrawerResponse {
    pub response: Response,
    pub selected: Option<NavigationSelection>,
    pub item_responses: Vec<NavigationItemResponse>,
}

pub struct ModalNavigationDrawerResponse {
    pub backdrop_response: Response,
    pub selected: Option<NavigationSelection>,
    pub close_requested: bool,
    pub item_responses: Vec<NavigationItemResponse>,
}

pub struct NavigationDrawer<'a> {
    title: Cow<'a, str>,
    groups: &'a [NavigationGroup],
    current_group: i32,
    current_index: i32,
    min_width: f32,
}

impl<'a> NavigationDrawer<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>, groups: &'a [NavigationGroup]) -> Self {
        Self {
            title: title.into(),
            groups,
            current_group: -1,
            current_index: -1,
            min_width: material_style_metrics().sizes.size_360,
        }
    }

    #[must_use]
    pub fn current_selection(mut self, current_group: i32, current_index: i32) -> Self {
        self.current_group = current_group;
        self.current_index = current_index;
        self
    }

    #[must_use]
    pub fn min_width(mut self, min_width: f32) -> Self {
        self.min_width = min_width;
        self
    }

    pub fn show(self, ui: &mut Ui) -> NavigationDrawerResponse {
        let mut selected = None;
        let mut item_responses = Vec::new();
        let response = Drawer {
            title: self.title,
            min_width: self.min_width,
        }
        .show(ui, |ui| {
            let render =
                show_navigation_groups(ui, self.groups, self.current_group, self.current_index);
            selected = render.selected;
            item_responses = render.item_responses;
        })
        .response;

        NavigationDrawerResponse {
            response,
            selected,
            item_responses,
        }
    }
}

pub struct ModalNavigationDrawer<'a> {
    title: Cow<'a, str>,
    groups: &'a [NavigationGroup],
    current_group: i32,
    current_index: i32,
    position: DrawerPosition,
    size: Vec2,
}

impl<'a> ModalNavigationDrawer<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>, groups: &'a [NavigationGroup]) -> Self {
        Self {
            title: title.into(),
            groups,
            current_group: -1,
            current_index: -1,
            position: DrawerPosition::Left,
            size: vec2(material_style_metrics().sizes.size_360, 0.0),
        }
    }

    #[must_use]
    pub fn current_selection(mut self, current_group: i32, current_index: i32) -> Self {
        self.current_group = current_group;
        self.current_index = current_index;
        self
    }

    #[must_use]
    pub fn position(mut self, position: DrawerPosition) -> Self {
        self.position = position;
        self
    }

    #[must_use]
    pub fn size(mut self, size: Vec2) -> Self {
        self.size = size;
        self
    }

    pub fn show(
        self,
        ctx: &Context,
        id_source: impl std::hash::Hash,
        available_rect: Rect,
    ) -> ModalNavigationDrawerResponse {
        let mut selected = None;
        let mut item_responses = Vec::new();
        let modal = ModalDrawer {
            title: self.title,
            position: self.position,
            size: self.size,
        }
        .show(ctx, id_source, available_rect, |ui| {
            let render =
                show_navigation_groups(ui, self.groups, self.current_group, self.current_index);
            selected = render.selected;
            item_responses = render.item_responses;
        });

        ModalNavigationDrawerResponse {
            backdrop_response: modal.backdrop_response,
            selected,
            close_requested: navigation_drawer_close_requested(modal.close_requested, selected),
            item_responses,
        }
    }
}

struct NavigationGroupsRender {
    selected: Option<NavigationSelection>,
    item_responses: Vec<NavigationItemResponse>,
}

fn show_navigation_groups(
    ui: &mut Ui,
    groups: &[NavigationGroup],
    current_group: i32,
    current_index: i32,
) -> NavigationGroupsRender {
    let metrics = material_style_metrics();
    let mut selected = None;
    let mut item_responses = Vec::new();

    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;

        for (group_index, group) in groups.iter().enumerate() {
            if !group.title.is_empty() {
                let _ = DrawerHeader::new(group.title.as_str()).show(ui);
            }

            for (item_index, item) in group.items.iter().enumerate() {
                let selection = NavigationSelection {
                    group_index,
                    item_index,
                };
                let response = show_navigation_item(
                    ui,
                    item,
                    is_current_selection(current_group, current_index, selection),
                );
                if response.clicked() {
                    selected = Some(selection);
                }
                item_responses.push(NavigationItemResponse {
                    selection,
                    response,
                });
            }

            if group_index + 1 < groups.len() {
                ui.add_space(metrics.spacings.spacing_12);
                let _ = draw_horizontal_divider(ui);
                ui.add_space(metrics.spacings.spacing_12);
            }
        }
    });

    NavigationGroupsRender {
        selected,
        item_responses,
    }
}

fn show_navigation_item(ui: &mut Ui, item: &NavigationItem, selected: bool) -> Response {
    let palette = material_palette_for_visuals(ui.visuals());
    let metrics = material_style_metrics();
    let desired_size = vec2(ui.available_width().max(0.0), metrics.sizes.size_56);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    let content_color = if selected {
        palette.on_secondary_container
    } else {
        palette.on_surface_variant
    };
    let pill_radius = CornerRadius::same((rect.height() / 2.0).round() as u8);
    let base_fill = if selected {
        palette.secondary_container
    } else {
        Color32::TRANSPARENT
    };
    ui.painter().rect_filled(rect, pill_radius, base_fill);

    let overlay = navigation_item_overlay_fill(&response, selected, content_color);
    if overlay != Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, pill_radius, overlay);
    }

    let inner_rect = Rect::from_min_max(
        rect.min + vec2(metrics.paddings.padding_16, 0.0),
        rect.max - vec2(metrics.paddings.padding_24, 0.0),
    );
    ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |ui| {
        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_12;
            show_navigation_item_leading(ui, item, content_color, selected);
            let _ = material_text(ui, item.text.as_str())
                .text_style(MATERIAL_TYPOGRAPHY.label_large)
                .color(content_color)
                .overflow(MaterialTextOverflow::Elide)
                .show(ui);
            ui.add_space(ui.available_width().max(0.0));
            if should_show_badge(item) {
                let _ = material_text(ui, item.badge.as_str())
                    .text_style(MATERIAL_TYPOGRAPHY.label_large)
                    .color(content_color)
                    .overflow(MaterialTextOverflow::Elide)
                    .show(ui);
            }
        });
    });

    response
}

fn show_navigation_item_leading(
    ui: &mut Ui,
    item: &NavigationItem,
    color: Color32,
    selected: bool,
) {
    let metrics = material_style_metrics();
    let icon_size = vec2(
        metrics.icon_sizes.icon_size_24,
        metrics.icon_sizes.icon_size_24,
    );
    if let Some(image) = navigation_item_icon(item, selected) {
        let style = MaterialIconStyle {
            size: icon_size,
            tint: color,
        };
        let _ = ui.add(icon_with_style(image, style));
        return;
    }

    let (rect, _) = ui.allocate_exact_size(icon_size, Sense::hover());
    let radius = metrics.paddings.padding_6;
    ui.painter().circle_filled(rect.center(), radius, color);
}

fn navigation_item_icon(item: &NavigationItem, selected: bool) -> Option<egui::Image<'static>> {
    if selected {
        item.selected_icon.clone().or_else(|| item.icon.clone())
    } else {
        item.icon.clone()
    }
}

fn should_show_badge(item: &NavigationItem) -> bool {
    item.show_badge || !item.badge.is_empty()
}

fn is_current_selection(
    current_group: i32,
    current_index: i32,
    selection: NavigationSelection,
) -> bool {
    current_group == selection.group_index as i32 && current_index == selection.item_index as i32
}

fn navigation_item_overlay_fill(response: &Response, selected: bool, color: Color32) -> Color32 {
    if selected {
        return Color32::TRANSPARENT;
    }

    let opacity = if response.is_pointer_button_down_on() {
        material_style_metrics().state_opacities.pressed
    } else if response.hovered() {
        material_style_metrics().state_opacities.hover
    } else {
        0.0
    };

    if opacity <= 0.0 {
        return Color32::TRANSPARENT;
    }

    color.gamma_multiply(opacity)
}

#[must_use]
pub fn resolve_navigation_selection(
    groups: &[NavigationGroup],
    group_index: i32,
    item_index: i32,
) -> Option<NavigationSelection> {
    let resolved = NavigationSelection {
        group_index: usize::try_from(group_index).ok()?,
        item_index: usize::try_from(item_index).ok()?,
    };
    let group = groups.get(resolved.group_index)?;
    let _item = group.items.get(resolved.item_index)?;
    Some(resolved)
}

#[must_use]
pub fn navigation_drawer_close_requested(
    backdrop_close_requested: bool,
    selected: Option<NavigationSelection>,
) -> bool {
    backdrop_close_requested || selected.is_some()
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use egui::Color32;
    use egui::Context;
    use egui::Id;
    use egui::Image;
    use egui::Rect;
    use egui::pos2;
    use egui::vec2;

    use super::ModalNavigationDrawer;
    use super::NavigationDrawer;
    use super::NavigationSelection;
    use super::navigation_drawer_close_requested;
    use super::resolve_navigation_selection;
    use crate::components::drawer::DrawerPosition;
    use crate::items::navigation_item::NavigationGroup;
    use crate::items::navigation_item::NavigationItem;

    #[test]
    fn resolve_selection_rejects_out_of_bounds_entries() {
        let groups = vec![NavigationGroup {
            title: String::from("Main"),
            items: vec![NavigationItem::new("Home")],
        }];

        assert_eq!(resolve_navigation_selection(&groups, -1, 0), None);
        assert_eq!(resolve_navigation_selection(&groups, 0, -1), None);
        assert_eq!(resolve_navigation_selection(&groups, 1, 0), None);
        assert_eq!(resolve_navigation_selection(&groups, 0, 1), None);
        assert_eq!(
            resolve_navigation_selection(&groups, 0, 0),
            Some(NavigationSelection {
                group_index: 0,
                item_index: 0,
            })
        );
    }

    #[test]
    fn modal_drawer_requests_close_after_selection() {
        assert!(navigation_drawer_close_requested(
            false,
            Some(NavigationSelection {
                group_index: 0,
                item_index: 0,
            }),
        ));
        assert!(navigation_drawer_close_requested(true, None));
        assert!(!navigation_drawer_close_requested(false, None));
    }

    #[test]
    fn navigation_drawer_renders_grouped_items_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let mut item_count = 0;
        let groups = demo_groups();

        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = NavigationDrawer::new("Browse", &groups)
                    .current_selection(0, 1)
                    .show(ui);
                width = response.response.rect.width();
                item_count = response.item_responses.len();
            });
        });

        assert!(width >= 360.0);
        assert_eq!(item_count, 3);
    }

    #[test]
    fn modal_navigation_drawer_renders_without_panicking() {
        let context = Context::default();
        let mut backdrop_width = 0.0;
        let groups = demo_groups();

        let _ = context.run(egui::RawInput::default(), |ctx| {
            let response = ModalNavigationDrawer::new(Cow::Borrowed("Browse"), &groups)
                .current_selection(1, 0)
                .position(DrawerPosition::Right)
                .size(vec2(360.0, 480.0))
                .show(
                    ctx,
                    Id::new("navigation_drawer_test"),
                    Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0)),
                );
            backdrop_width = response.backdrop_response.rect.width();
        });

        assert!(backdrop_width >= 800.0);
    }

    fn demo_groups() -> Vec<NavigationGroup> {
        vec![
            NavigationGroup {
                title: String::from("Main"),
                items: vec![demo_item("Home", true, "7"), NavigationItem::new("Library")],
            },
            NavigationGroup {
                title: String::from("Secondary"),
                items: vec![NavigationItem {
                    icon: Some(
                        Image::new((egui::TextureId::Managed(1), vec2(24.0, 24.0)))
                            .tint(Color32::WHITE),
                    ),
                    selected_icon: None,
                    text: String::from("Settings"),
                    show_badge: false,
                    badge: String::new(),
                }],
            },
        ]
    }

    fn demo_item(text: &str, show_badge: bool, badge: &str) -> NavigationItem {
        NavigationItem {
            icon: None,
            selected_icon: None,
            text: String::from(text),
            show_badge,
            badge: String::from(badge),
        }
    }
}
