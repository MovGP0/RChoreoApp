use crate::clone_mode::CloneMode;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FloorModel {
    pub size_front: i32,
    pub size_back: i32,
    pub size_left: i32,
    pub size_right: i32,
}

impl FloorModel {
    pub fn clone_with(&self, _mode: CloneMode) -> Self {
        self.clone()
    }
}
