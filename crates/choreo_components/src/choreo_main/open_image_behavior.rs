use crossbeam_channel::Sender;
use nject::injectable;

use super::messages::OpenSvgFileCommand;

#[injectable]
#[inject(|sender: Sender<OpenSvgFileCommand>| Self { sender })]
pub struct OpenImageBehavior {
    sender: Sender<OpenSvgFileCommand>,
}

impl OpenImageBehavior {
    pub fn new(sender: Sender<OpenSvgFileCommand>) -> Self {
        Self { sender }
    }

    pub fn open_svg(&self, path: String) {
        let _ = self.sender.send(OpenSvgFileCommand { file_path: path });
    }
}
