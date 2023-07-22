#[repr(u8)]
pub enum ProjectType {
    Pet = 1,
    Startup = 2,
    Research = 3,
}

#[repr(u8)]
pub enum ProjectStatus {
    InSearch = 1,
    InProgress = 2,
    Ended = 3
}

#[repr(u8)]
pub enum PositionType {
    Backend = 1,
    Frontend = 2,
    Manager = 3,
    Tester = 4,
    DataScientist = 5,
    Designer = 6,
    Analyst = 7,
    SEO = 8,
    Devops = 9
}