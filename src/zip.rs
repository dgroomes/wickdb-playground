/// This represents characteristics about the area defined by a ZIP code.
/// For example, it includes population and the containing geographies like the city and state names.
pub struct ZipArea {
    // Note that the highest ZIP code is 99950 (Ketchikan, Alaska). u16 is too small (65,536) so we
    // need to use u32.
    pub code: u32,
    pub city: String,
    pub state: String,
    // u32 (4 billion) is a reasonable upper limit for the population at a ZIP code!
    pub pop: u32,
}
