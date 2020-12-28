/// Define an objective for the layout on
/// a specific coordinate.
/// - If `Maximize` is indicated on a dimension, then
/// the widget should maximize the space it takes
/// along it.
/// - If `Minimize` is indicated on a dimension, then
/// the widget should minimize the space it takes
/// along it.
/// - If `None` is indicated on a dimension, then
/// the widget has no constraint along that dimension.
pub enum Objective {
    Maximize,
    Minimize,
    None,
}

/// A layout query is used when a parent widget
/// requests the layout of a child. The parent indicates
/// the space it grants it along the X and Y axes,
/// and the objectives associated.
/// The space is contained in a `Option`. `Some(x)` represents
/// a finite amount of space, and `None` represents an infinite
/// space.
pub struct LayoutQuery {
    pub available_space: (Option<f32>, Option<f32>),
    pub objectives: (Objective, Objective),
}

/// The status of the layout along a dimension.
/// It allows to indicate to the parent if
/// the constraint or the space given on a dimension
/// where good, unsufficient or impossible to satisfy.
/// This enum is then useful for the parent to determine
/// which constraint was problematic, and adapt it if it can.
pub enum LayoutStatus {
    Ok,
    Inconsistencies,
    WontDisplay,
}

/// The response a widget returns to its parent after a request.
/// It contains the size the widget will take inside the attributed space,
/// and the status of each dimension.
pub struct LayoutResponse {
    pub size: (f32, f32),
    pub status: (LayoutStatus, LayoutStatus),
}
