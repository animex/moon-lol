use bitflags::bitflags;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LayerTransitionBehavior {
    Unaffected,
    TurnInvisibleDoesNotMatchNewLayerFilter,
    TurnVisibleDoesMatchNewLayerFilter,
}

impl From<u8> for LayerTransitionBehavior {
    fn from(value: u8) -> Self {
        match value {
            1 => LayerTransitionBehavior::TurnInvisibleDoesNotMatchNewLayerFilter,
            2 => LayerTransitionBehavior::TurnVisibleDoesMatchNewLayerFilter,
            _ => LayerTransitionBehavior::Unaffected,
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct EnvironmentVisibility: u8 {
        const NoLayer = 0;
        const Layer1 = 1 << 0;
        const Layer2 = 1 << 1;
        const Layer3 = 1 << 2;
        const Layer4 = 1 << 3;
        const Layer5 = 1 << 4;
        const Layer6 = 1 << 5;
        const Layer7 = 1 << 6;
        const Layer8 = 1 << 7;
        const AllLayers = 255;
    }
}
