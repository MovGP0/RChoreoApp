use std::cell::Cell;
use std::time::Duration;

use crate::settings;

use choreo_components::preferences::InMemoryPreferences;
use choreo_components::settings::{MaterialSchemeHelper, MaterialSchemeUpdater, SettingsViewModel, ThemeMode};
use choreo_components::shell::ShellMaterialSchemeApplier;
use choreo_components::MaterialPalette;
use choreo_components::SettingsInfo;
use choreo_components::ShellHost;
use slint::Color;
use slint::ComponentHandle;
use settings::Report;

thread_local! {
    static INIT_TEST_BACKEND: Cell<bool> = const { Cell::new(false) };
}

fn ensure_slint_test_backend() {
    INIT_TEST_BACKEND.with(|initialized| {
        if initialized.get() {
            return;
        }
        i_slint_backend_testing::init_no_event_loop();
        initialized.set(true);
    });
}

#[test]
#[serial_test::serial]
fn material_theme_application_spec() {
    let suite = rspec::describe("material theme application", (), |spec| {
        spec.it("applies light and dark palette color scheme from theme toggle", |_| {
            let test_thread = std::thread::Builder::new()
                .name("material-theme-application-spec".to_string())
                .stack_size(32 * 1024 * 1024)
                .spawn(|| {
                    ensure_slint_test_backend();
                    let view =
                        ShellHost::new().expect("shell host should be created in test backend");
                    let updater = MaterialSchemeHelper::new(ShellMaterialSchemeApplier::new(&view));
                    let preferences = InMemoryPreferences::new();
                    let mut view_model = SettingsViewModel::new();
                    view_model.use_system_theme = false;

                    let settings_info = view.global::<SettingsInfo<'_>>();
                    settings_info.set_use_system_theme(false);
                    settings_info.set_is_dark_mode(true);

                    view_model.theme_mode = ThemeMode::Dark;
                    let dark_expected =
                        MaterialSchemeHelper::<ShellMaterialSchemeApplier>::build_schemes(&view_model)
                            .dark
                            .background;
                    updater.update(&view_model, &preferences);
                    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(20));
                    slint::platform::update_timers_and_animations();

                    let palette = view.global::<MaterialPalette<'_>>();
                    assert_eq!(
                        palette.get_background(),
                        Color::from_argb_u8(
                            dark_expected.a,
                            dark_expected.r,
                            dark_expected.g,
                            dark_expected.b,
                        )
                    );

                    settings_info.set_is_dark_mode(false);
                    view_model.theme_mode = ThemeMode::Light;
                    let light_expected =
                        MaterialSchemeHelper::<ShellMaterialSchemeApplier>::build_schemes(&view_model)
                            .light
                            .background;
                    updater.update(&view_model, &preferences);
                    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(20));
                    slint::platform::update_timers_and_animations();

                    assert_eq!(
                        palette.get_background(),
                        Color::from_argb_u8(
                            light_expected.a,
                            light_expected.r,
                            light_expected.g,
                            light_expected.b,
                        )
                    );
                })
                .expect("material theme spec thread should start");

            match test_thread.join() {
                Ok(()) => {}
                Err(payload) => std::panic::resume_unwind(payload),
            }
        });
    });

    let report = settings::run_suite(&suite);
    assert!(report.is_success());
}
