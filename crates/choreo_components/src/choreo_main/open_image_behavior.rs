use nject::injectable;

use super::messages::OpenImageRequested;
use super::messages::OpenSvgFileCommand;
use super::state::ChoreoMainState;

#[injectable]
#[inject(|| Self)]
#[derive(Clone)]
pub struct OpenImageBehavior;

impl OpenImageBehavior {
    #[must_use]
    pub fn apply(&self, request: OpenImageRequested) -> OpenSvgFileCommand {
        OpenSvgFileCommand {
            file_path: request.file_path,
        }
    }
}

pub(super) fn request_open_image(state: &mut ChoreoMainState, file_path: String) {
    state
        .outgoing_open_svg_commands
        .push(OpenSvgFileCommand { file_path });
}
