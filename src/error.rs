#[derive(Debug)]
pub enum Error {
    DatabaseError(String),
    MqttError(String),
    WebError(String),
    ExcelError(String),
}
impl Error {
    pub fn to_string(&self) -> String {
        match self {
            Self::DatabaseError(str)=>format!("database error {}",str),
            Self::MqttError(str) => format!("mqtt error {}",str),
            Self::WebError(str) => format!("web error {}",str),
            Self::ExcelError(str) => format!("excel error {}",str),
        }        
    }
}