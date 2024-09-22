pub struct GrepFlags {
    pub reverse: bool,
    pub lines: bool,
    pub count: bool,
    pub ignore_case: bool,
    pub all_files: bool,
}

impl GrepFlags {
    pub fn new() -> Self {
        Self {
            reverse: false,
            lines: false,
            count: false,
            ignore_case: false,
            all_files: false,
        }
    }
}
