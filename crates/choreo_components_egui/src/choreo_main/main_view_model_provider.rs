use std::cell::RefCell;
use std::rc::Rc;

use crate::behavior::Behavior;

use super::main_view_model::MainViewModel;

#[derive(Default)]
pub struct MainViewModelProviderDependencies {
    pub behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
}

pub struct MainViewModelProvider {
    view_model: Rc<RefCell<MainViewModel>>,
}

impl MainViewModelProvider {
    pub fn new(deps: MainViewModelProviderDependencies) -> Self {
        let mut view_model = MainViewModel::new(deps.behaviors);
        view_model.activate();
        Self {
            view_model: Rc::new(RefCell::new(view_model)),
        }
    }

    pub fn main_view_model(&self) -> Rc<RefCell<MainViewModel>> {
        Rc::clone(&self.view_model)
    }
}
