pub enum OrderSide {
    Buy,
    Sell,
}

impl OrderSide {
    pub fn as_str(&self) -> String {
        match self {
            Self::Buy => "BUY".to_string(),
            Self::Sell => "SELL".to_string(),
        }
    }
}

pub enum OrderType {
    Limit,
    Market,
    LimitMaker,
    ImmediateOrCancel,
    FillOrKill,
}

impl OrderType {
    pub fn as_str(&self) -> String {
        match self {
            Self::Limit => "LIMIT".to_string(),
            Self::Market => "MARKET".to_string(),
            Self::LimitMaker => "LIMIT_MAKER".to_string(),
            Self::ImmediateOrCancel => "IMMEDIATE_OR_CANCEL".to_string(),
            Self::FillOrKill => "FILL_OR_KILL".to_string(),
        }
    }
}

pub enum OrderStatus {
    New,
    Filled,
    PartiallyFilled,
    Canceled,
    PartiallyCanceled,
}

impl OrderStatus {
    pub fn as_str(&self) -> String {
        match self {
            Self::New => "NEW".to_string(),
            Self::Filled => "FILLED".to_string(),
            Self::PartiallyFilled => "PARTIALLY_FILLED".to_string(),
            Self::Canceled => "CANCELED".to_string(),
            Self::PartiallyCanceled => "PARTIALLY_CANCELED".to_string(),
        }
    }
}

pub enum Permissions {
    Spot,
    Futures,
}

impl Permissions {
    pub fn as_str(&self) -> String {
        match self {
            Self::Spot => "SPOT".to_string(),
            Self::Futures => "FUTURES".to_string(),
        }
    }
}

pub enum KlineInterval {
    m1,
    m5,
    m15,
    m30,
    m60,
    h4,
    d1,
    W1,
    M1,
}

impl KlineInterval {
    pub fn as_str(&self) -> String {
        match self {
            Self::m1 => "1m".to_string(),
            Self::m5 => "5m".to_string(),
            Self::m15 => "15m".to_string(),
            Self::m30 => "30m".to_string(),
            Self::m60 => "60m".to_string(),
            Self::h4 => "4h".to_string(),
            Self::d1 => "1d".to_string(),
            Self::W1 => "1W".to_string(),
            Self::M1 => "1M".to_string(),
        }
    }
}

pub enum DepositStatus {
    Small,
    TimeDelay,
    LargeDelay,
    Pending,
    Success,
    Auditing,
    Rejected,
}

impl DepositStatus {
    pub fn as_str(&self) -> String {
        match self {
            Self::Small => 1.to_string(),
            Self::TimeDelay => 2.to_string(),
            Self::LargeDelay => 3.to_string(),
            Self::Pending => 4.to_string(),
            Self::Success => 5.to_string(),
            Self::Auditing => 6.to_string(),
            Self::Rejected => 7.to_string(),
        }
    }
}

pub enum WithdrawStatus {
    Apply,
    Auditing,
    Wait,
    Processing,
    WaitPackaging,
    WaitConfirm,
    Success,
    Failed,
    Cancel,
    Manual,
}

impl WithdrawStatus {
    pub fn as_str(&self) -> String {
        match self {
            Self::Apply => 1.to_string(),
            Self::Auditing => 2.to_string(),
            Self::Wait => 3.to_string(),
            Self::Processing => 4.to_string(),
            Self::WaitPackaging => 5.to_string(),
            Self::WaitConfirm => 6.to_string(),
            Self::Success => 7.to_string(),
            Self::Failed => 8.to_string(),
            Self::Cancel => 9.to_string(),
            Self::Manual => 10.to_string(),
        }
    }
}

pub enum AccountType {
    Spot,
    Futures,
}

impl AccountType {
    pub fn as_str(&self) -> String {
        match self {
            Self::Spot => "SPOT".to_string(),
            Self::Futures => "FUTURES".to_string(),
        }
    }
}
