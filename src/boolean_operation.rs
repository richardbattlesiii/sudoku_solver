/// Enum to specify the operation type when combining multiple operations (for conciseness).
#[derive(Clone, Copy)]
pub enum BooleanOperation {
    Or,
    OrLazy,
    And,
    AndLazy,
}

impl BooleanOperation {
    /// Initial value for the operation type.
    pub fn initial(self) -> bool {
        match self {
            BooleanOperation::Or | BooleanOperation::OrLazy => false,
            BooleanOperation::And | BooleanOperation::AndLazy => true,
        }
    }

    /// Combines the current result with the new value based on the operation type.
    pub fn combine(self, current: bool, next: bool) -> bool {
        match self {
            BooleanOperation::Or => current | next,
            BooleanOperation::OrLazy => current || next,
            BooleanOperation::And => current & next,
            BooleanOperation::AndLazy => current && next,
        }
    }
}