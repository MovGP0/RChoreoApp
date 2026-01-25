#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedrawFloorCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShowTimestampsChangedEvent {
    pub is_enabled: bool,
}
