use crate::main_page::Report;
use choreo_components::main_page::ui::audio_panel_layer_order;
use choreo_components::main_page::ui::top_bar_layer_order;
use material3::components::drawer_host::ui::content_layer_order;
use material3::components::drawer_host::ui::panel_layer_order;
use material3::components::drop_down_menu::dropdown_menu_layer_order;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn z_order_spec() {
    let suite = rspec::describe("main page z-order hierarchy", (), |spec| {
        spec.it(
            "keeps app chrome in the foreground and below tooltips",
            |_| {
                let mut errors = Vec::new();

                check_eq!(errors, top_bar_layer_order(), egui::Order::Foreground);
                check_eq!(errors, dropdown_menu_layer_order(), egui::Order::Foreground);
                check_eq!(errors, audio_panel_layer_order(), egui::Order::Foreground);
                check_eq!(errors, panel_layer_order(), egui::Order::Foreground);
                check!(errors, dropdown_menu_layer_order() < egui::Order::Tooltip);
                check!(errors, panel_layer_order() > content_layer_order());

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::main_page::run_suite(&suite);
    assert!(report.is_success());
}
