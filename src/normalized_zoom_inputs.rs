/// Holds normalized zoom inputs constructed from raw events.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NormalizedZoomInputs {
    pub(crate) pinch: f32,
    pub(crate) wheel: f32,
}

impl NormalizedZoomInputs {
    /// Apply sensitivity scalers to the inputs and return a final zoom delta to apply.
    pub(crate) fn apply_sensitivity(&self, wheel_sensitivity: f32, pinch_sensitivity: f32) -> f32 {
        self.pinch * pinch_sensitivity + self.wheel * wheel_sensitivity
    }

    /// True when no input.
    pub(crate) fn is_empty(self) -> bool {
        self.pinch == 0. && self.wheel == 0.
    }
}
