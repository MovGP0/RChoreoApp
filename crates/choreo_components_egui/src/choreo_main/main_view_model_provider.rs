use std::cell::RefCell;
use std::rc::Rc;

use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;

use super::behavior_pipeline::MainBehaviorDependencies;
use super::behavior_pipeline::MainBehaviorPipeline;
use super::main_view_model::MainViewModel;

#[derive(Default)]
pub struct MainViewModelProviderDependencies {
    pub behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
    pub behavior_dependencies: MainBehaviorDependencies,
}

pub struct MainViewModelProvider {
    view_model: Rc<RefCell<MainViewModel>>,
    behavior_pipeline: MainBehaviorPipeline,
}

impl MainViewModelProvider {
    pub fn new(deps: MainViewModelProviderDependencies) -> Self {
        let behavior_pipeline = MainBehaviorPipeline::from_dependencies(deps.behavior_dependencies);
        let mut view_model = MainViewModel::new(deps.behaviors);
        let mut disposables = CompositeDisposable::new();
        if let Some(behavior) = behavior_pipeline.open_svg_file_behavior.as_ref() {
            behavior.activate(&mut view_model, &mut disposables);
        }
        if let Some(behavior) = behavior_pipeline.open_audio_behavior.as_ref() {
            behavior.activate(&mut view_model, &mut disposables);
        }
        if let Some(behavior) = behavior_pipeline.open_image_behavior.as_ref() {
            behavior.activate(&mut view_model, &mut disposables);
        }
        if let Some(behavior) = behavior_pipeline.apply_interaction_mode_behavior.as_ref() {
            behavior.activate(&mut view_model, &mut disposables);
        }
        behavior_pipeline
            .show_dialog_behavior
            .activate(&mut view_model, &mut disposables);
        behavior_pipeline
            .hide_dialog_behavior
            .activate(&mut view_model, &mut disposables);
        view_model.activate();
        Self {
            view_model: Rc::new(RefCell::new(view_model)),
            behavior_pipeline,
        }
    }

    pub fn main_view_model(&self) -> Rc<RefCell<MainViewModel>> {
        Rc::clone(&self.view_model)
    }

    pub fn behavior_pipeline(&self) -> &MainBehaviorPipeline {
        &self.behavior_pipeline
    }
}
