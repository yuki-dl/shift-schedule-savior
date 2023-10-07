/*
朝、昼、夜
- 必要人数
- 時間の定義
    - 朝 7:00~13:00
    - 昼 13:00~17:00
    - 夜 17:00~22:00
*/

#[derive(Debug, Clone, PartialEq)]
pub enum TimeFrame {
    Morning,
    Afternoon,
    Evening,
    Full1,
    Full2,
}
impl TimeFrame {
    pub fn get(&self) -> String {
        match self {
            TimeFrame::Morning => "朝".to_string(),
            TimeFrame::Afternoon => "昼".to_string(),
            TimeFrame::Evening => "夜".to_string(),
            TimeFrame::Full1 => "朝昼".to_string(),
            TimeFrame::Full2 => "昼夜".to_string(),
        }
    }
    pub fn shift(&self) -> TimeFrame {
        let new = match self {
            TimeFrame::Morning => TimeFrame::Afternoon,
            TimeFrame::Afternoon => TimeFrame::Evening,
            TimeFrame::Evening => TimeFrame::Full1,
            TimeFrame::Full1 => TimeFrame::Full2,
            TimeFrame::Full2 => TimeFrame::Morning,
        };
        new
    }
    pub fn to_signal(&self) -> u8 {
        match self {
            TimeFrame::Morning => 2,
            TimeFrame::Afternoon => 3,
            TimeFrame::Evening => 4,
            TimeFrame::Full1 => 5,
            TimeFrame::Full2 => 6,
        }
    }
}
