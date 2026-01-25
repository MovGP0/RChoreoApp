use std::fmt::Debug;

use crate::states::StateKind;
use crate::triggers::TriggerKind;

pub trait GlobalStateModel: Debug {}

pub trait ApplicationState: Debug {
    fn kind(&self) -> StateKind;
}

pub trait ApplicationTrigger: Debug {
    fn kind(&self) -> TriggerKind;
}
