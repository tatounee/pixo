type AllCases = bool;

#[derive(Debug)]
pub enum FlipMode {
    Recto,
    Verso,
    Random(AllCases),
}

impl FlipMode {
    pub fn is_all_cases(&self) -> bool {
        if let Self::Random(all_cases) = self {
            *all_cases
        } else {
            false
        }
    }
}
