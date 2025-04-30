#[derive(Debug)]
pub enum SuperMarketError {
    CommandNotFound,
    NoItemInStock,
    CriticalError,
    NoSuchItem,
    PositionAlreadyFilled,
}

impl SuperMarketError {
    pub fn to_str(&self) -> &'static str {
        match self {
            SuperMarketError::CommandNotFound => "Command not found",
            SuperMarketError::NoItemInStock => "No item in stock",
            SuperMarketError::CriticalError => "This should not happen",
            SuperMarketError::PositionAlreadyFilled => "Position already filled",
            SuperMarketError::NoSuchItem => "Product does not exist",
        }
    }
}