#[derive(Debug, Clone, PartialEq)]
pub struct DrawFloorCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PanUpdatedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PinchUpdatedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerPressedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerMovedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerReleasedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerWheelChangedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct TouchCommand;
