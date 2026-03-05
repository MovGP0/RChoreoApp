#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChoreoDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoreoInfoState {
    pub choreo_name: String,
    pub choreo_subtitle: String,
    pub choreo_comment: String,
    pub choreo_date: ChoreoDate,
    pub choreo_variation: String,
    pub choreo_author: String,
    pub choreo_description: String,
    pub choreo_transparency: f64,
}

impl Default for ChoreoInfoState {
    fn default() -> Self {
        Self {
            choreo_name: "Lorem ipsum dolor sit amet".to_string(),
            choreo_subtitle: "consectetur adipiscing elit".to_string(),
            choreo_comment: String::new(),
            choreo_date: ChoreoDate::default(),
            choreo_variation: String::new(),
            choreo_author: String::new(),
            choreo_description: String::new(),
            choreo_transparency: 0.0,
        }
    }
}
