/// Enum to specify the operation type
#[derive(Clone, Copy)]
pub enum BooleanOperation {
    Any,
    AnyLazy,
    All,
    AllLazy,
}

impl BooleanOperation {
    /// Combines the current result with the new value based on the operation type
    pub fn combine(self, current: bool, next: bool) -> bool {
        match self {
            BooleanOperation::Any => current | next,
            BooleanOperation::AnyLazy => current || next,
            BooleanOperation::All => current & next,
            BooleanOperation::AllLazy => current && next,
        }
    }

    /// Initial value for the operation type
    pub fn initial(self) -> bool {
        match self {
            BooleanOperation::Any | BooleanOperation::AnyLazy => false,
            BooleanOperation::All | BooleanOperation::AllLazy => true,
        }
    }
}