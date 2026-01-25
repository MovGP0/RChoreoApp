use crate::traits::ApplicationTrigger;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    ApplicationTrigger,
    MovePositionsCompletedTrigger,
    MovePositionsDragCompletedTrigger,
    MovePositionsDragStartedTrigger,
    MovePositionsSelectionCompletedTrigger,
    MovePositionsSelectionStartedTrigger,
    MovePositionsStartedTrigger,
    PanCompletedTrigger,
    PanStartedTrigger,
    PlacePositionsCanceledTrigger,
    PlacePositionsCompletedTrigger,
    PlacePositionsStartedTrigger,
    RotateAroundCenterCompletedTrigger,
    RotateAroundCenterRotationCompletedTrigger,
    RotateAroundCenterRotationStartedTrigger,
    RotateAroundCenterSelectionCompletedTrigger,
    RotateAroundCenterSelectionStartedTrigger,
    RotateAroundCenterStartedTrigger,
    ScaleAroundDancerCompletedTrigger,
    ScaleAroundDancerDragCompletedTrigger,
    ScaleAroundDancerDragStartedTrigger,
    ScaleAroundDancerSelectionCompletedTrigger,
    ScaleAroundDancerSelectionStartedTrigger,
    ScaleAroundDancerStartedTrigger,
    ScalePositionsCompletedTrigger,
    ScalePositionsDragCompletedTrigger,
    ScalePositionsDragStartedTrigger,
    ScalePositionsSelectionCompletedTrigger,
    ScalePositionsSelectionStartedTrigger,
    ScalePositionsStartedTrigger,
    ZoomCompletedTrigger,
    ZoomStartedTrigger,
}

impl TriggerKind {
    pub fn is_assignable_from(self, other: TriggerKind) -> bool {
        self == TriggerKind::ApplicationTrigger || self == other
    }
}

macro_rules! trigger_struct {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl ApplicationTrigger for $name {
            fn kind(&self) -> TriggerKind {
                $kind
            }
        }
    };
}

trigger_struct!(ApplicationTriggerBase, TriggerKind::ApplicationTrigger);
trigger_struct!(
    MovePositionsCompletedTrigger,
    TriggerKind::MovePositionsCompletedTrigger
);
trigger_struct!(
    MovePositionsDragCompletedTrigger,
    TriggerKind::MovePositionsDragCompletedTrigger
);
trigger_struct!(
    MovePositionsDragStartedTrigger,
    TriggerKind::MovePositionsDragStartedTrigger
);
trigger_struct!(
    MovePositionsSelectionCompletedTrigger,
    TriggerKind::MovePositionsSelectionCompletedTrigger
);
trigger_struct!(
    MovePositionsSelectionStartedTrigger,
    TriggerKind::MovePositionsSelectionStartedTrigger
);
trigger_struct!(
    MovePositionsStartedTrigger,
    TriggerKind::MovePositionsStartedTrigger
);
trigger_struct!(PanCompletedTrigger, TriggerKind::PanCompletedTrigger);
trigger_struct!(PanStartedTrigger, TriggerKind::PanStartedTrigger);
trigger_struct!(
    PlacePositionsCanceledTrigger,
    TriggerKind::PlacePositionsCanceledTrigger
);
trigger_struct!(
    PlacePositionsCompletedTrigger,
    TriggerKind::PlacePositionsCompletedTrigger
);
trigger_struct!(
    PlacePositionsStartedTrigger,
    TriggerKind::PlacePositionsStartedTrigger
);
trigger_struct!(
    RotateAroundCenterCompletedTrigger,
    TriggerKind::RotateAroundCenterCompletedTrigger
);
trigger_struct!(
    RotateAroundCenterRotationCompletedTrigger,
    TriggerKind::RotateAroundCenterRotationCompletedTrigger
);
trigger_struct!(
    RotateAroundCenterRotationStartedTrigger,
    TriggerKind::RotateAroundCenterRotationStartedTrigger
);
trigger_struct!(
    RotateAroundCenterSelectionCompletedTrigger,
    TriggerKind::RotateAroundCenterSelectionCompletedTrigger
);
trigger_struct!(
    RotateAroundCenterSelectionStartedTrigger,
    TriggerKind::RotateAroundCenterSelectionStartedTrigger
);
trigger_struct!(
    RotateAroundCenterStartedTrigger,
    TriggerKind::RotateAroundCenterStartedTrigger
);
trigger_struct!(
    ScaleAroundDancerCompletedTrigger,
    TriggerKind::ScaleAroundDancerCompletedTrigger
);
trigger_struct!(
    ScaleAroundDancerDragCompletedTrigger,
    TriggerKind::ScaleAroundDancerDragCompletedTrigger
);
trigger_struct!(
    ScaleAroundDancerDragStartedTrigger,
    TriggerKind::ScaleAroundDancerDragStartedTrigger
);
trigger_struct!(
    ScaleAroundDancerSelectionCompletedTrigger,
    TriggerKind::ScaleAroundDancerSelectionCompletedTrigger
);
trigger_struct!(
    ScaleAroundDancerSelectionStartedTrigger,
    TriggerKind::ScaleAroundDancerSelectionStartedTrigger
);
trigger_struct!(
    ScaleAroundDancerStartedTrigger,
    TriggerKind::ScaleAroundDancerStartedTrigger
);
trigger_struct!(
    ScalePositionsCompletedTrigger,
    TriggerKind::ScalePositionsCompletedTrigger
);
trigger_struct!(
    ScalePositionsDragCompletedTrigger,
    TriggerKind::ScalePositionsDragCompletedTrigger
);
trigger_struct!(
    ScalePositionsDragStartedTrigger,
    TriggerKind::ScalePositionsDragStartedTrigger
);
trigger_struct!(
    ScalePositionsSelectionCompletedTrigger,
    TriggerKind::ScalePositionsSelectionCompletedTrigger
);
trigger_struct!(
    ScalePositionsSelectionStartedTrigger,
    TriggerKind::ScalePositionsSelectionStartedTrigger
);
trigger_struct!(
    ScalePositionsStartedTrigger,
    TriggerKind::ScalePositionsStartedTrigger
);
trigger_struct!(ZoomCompletedTrigger, TriggerKind::ZoomCompletedTrigger);
trigger_struct!(ZoomStartedTrigger, TriggerKind::ZoomStartedTrigger);
