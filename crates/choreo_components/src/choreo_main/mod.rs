mod behaviors;
mod view_model;

pub use behaviors::{
    build_main_behaviors, ApplyInteractionModeBehavior, CloseDialogCommand, HideDialogBehavior,
    MainBehaviorDependencies, MainBehaviors, OpenAudioBehavior, OpenImageBehavior, OpenSvgFileBehavior,
    OpenSvgFileCommand, ShowDialogBehavior, ShowDialogCommand,
};
pub use view_model::{build_main_view_model, InteractionModeOption, MainDependencies, MainViewModel};
