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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LayoutStatus {
    Ok,
    Inconsistencies,
    WontDisplay,
}

impl LayoutStatus {
    pub fn and(status1: LayoutStatus, status2: LayoutStatus) -> LayoutStatus {
        match (status1, status2) {
            (LayoutStatus::Ok, LayoutStatus::Ok) => LayoutStatus::Ok,
            (LayoutStatus::WontDisplay, _) | (_, LayoutStatus::WontDisplay) => {
                LayoutStatus::WontDisplay
            }
            (LayoutStatus::Inconsistencies, _) | (_, LayoutStatus::Inconsistencies) => {
                LayoutStatus::Inconsistencies
            }
        }
    }
}

/// The response a widget returns to its parent after a request.
/// It contains the size the widget will take inside the attributed space,
/// and the status of each dimension.
#[derive(Copy, Clone)]
pub struct LayoutResponse {
    pub size: (f32, f32),
    pub status: (LayoutStatus, LayoutStatus),
}

#[cfg(test)]
mod tests {
    use crate::core::*;
    #[test]
    fn layout_status_1() {
        assert_eq!(
            LayoutStatus::Ok,
            LayoutStatus::and(LayoutStatus::Ok, LayoutStatus::Ok)
        )
    }

    #[test]
    fn layout_status_2() {
        assert_eq!(
            LayoutStatus::Inconsistencies,
            LayoutStatus::and(LayoutStatus::Inconsistencies, LayoutStatus::Inconsistencies)
        );
        assert_eq!(
            LayoutStatus::Inconsistencies,
            LayoutStatus::and(LayoutStatus::Ok, LayoutStatus::Inconsistencies)
        );
        assert_eq!(
            LayoutStatus::Inconsistencies,
            LayoutStatus::and(LayoutStatus::Inconsistencies, LayoutStatus::Ok)
        )
    }

    #[test]
    fn layout_status_3() {
        assert_eq!(
            LayoutStatus::WontDisplay,
            LayoutStatus::and(LayoutStatus::WontDisplay, LayoutStatus::WontDisplay)
        );
        assert_eq!(
            LayoutStatus::WontDisplay,
            LayoutStatus::and(LayoutStatus::Inconsistencies, LayoutStatus::WontDisplay)
        );
        assert_eq!(
            LayoutStatus::WontDisplay,
            LayoutStatus::and(LayoutStatus::WontDisplay, LayoutStatus::Inconsistencies)
        );
        assert_eq!(
            LayoutStatus::WontDisplay,
            LayoutStatus::and(LayoutStatus::Ok, LayoutStatus::WontDisplay)
        );
        assert_eq!(
            LayoutStatus::WontDisplay,
            LayoutStatus::and(LayoutStatus::WontDisplay, LayoutStatus::Ok)
        );
    }
}
