use std::cell::RefCell;
use std::rc::Rc;

use crate::ShellHost;

use super::{NavBarViewModel, mode_index, mode_option_from_index};

pub fn apply_nav_bar_view_model(view: &ShellHost, view_model: &NavBarViewModel) {
    view.set_selected_mode_index(mode_index(view_model.selected_mode));
    view.set_is_mode_selection_enabled(view_model.is_mode_selection_enabled);
    view.set_nav_width(view_model.nav_width);
    view.set_is_nav_open(view_model.is_nav_open);
    view.set_is_choreography_settings_open(view_model.is_choreography_settings_open);
    view.set_is_audio_player_open(view_model.is_audio_player_open);
}

pub fn bind_nav_bar(view: &ShellHost, view_model: Rc<RefCell<NavBarViewModel>>) {
    {
        let view_model = Rc::clone(&view_model);
        view.on_toggle_nav(move || {
            view_model.borrow_mut().toggle_navigation();
        });
    }

    {
        let view_model = Rc::clone(&view_model);
        view.on_close_nav(move || {
            view_model.borrow_mut().close_navigation();
        });
    }

    {
        let view_model = Rc::clone(&view_model);
        view.on_close_settings(move || {
            view_model.borrow_mut().close_choreography_settings();
        });
    }

    {
        let view_model = Rc::clone(&view_model);
        view.on_open_audio(move || {
            view_model.borrow_mut().open_audio();
        });
    }

    {
        let view_model = Rc::clone(&view_model);
        view.on_open_image(move || {
            view_model.borrow_mut().open_image();
        });
    }

    {
        let view_model = Rc::clone(&view_model);
        view.on_open_settings(move || {
            view_model.borrow_mut().open_choreography_settings();
        });
    }

    {
        let view_model = Rc::clone(&view_model);
        view.on_select_mode(move |index| {
            let Some(mode) = mode_option_from_index(index) else {
                return;
            };

            view_model.borrow_mut().set_selected_mode(mode);
        });
    }
}
