#[derive(Clone)]
enum ComparisonFlag {
    Less,
    Greater,
    Equal
}

#[derive(Clone)]
pub struct FlagState {
    comparison: ComparisonFlag,
    overflow: bool
}

impl FlagState {
    pub fn is_less(&self) -> bool {
        matches!(self.comparison, ComparisonFlag::Less)
    }

    pub fn set_less(&mut self) {
        self.comparison = ComparisonFlag::Less;
    }

    pub fn is_greater(&self) -> bool {
        matches!(self.comparison, ComparisonFlag::Greater)
    }

    pub fn set_greater(&mut self) {
        self.comparison = ComparisonFlag::Greater;
    }

    pub fn is_equal(&self) -> bool {
        matches!(self.comparison, ComparisonFlag::Equal)
    }

    pub fn set_equal(&mut self) {
        self.comparison = ComparisonFlag::Equal;
    }

    pub fn is_overflow(&self) -> bool {
        self.overflow
    }

    pub fn set_overflow(&mut self) {
        self.overflow = true;
    }

    pub fn unset_overflow(&mut self) {
        self.overflow = false;
    }
}
