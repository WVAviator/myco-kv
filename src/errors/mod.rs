#[derive(PartialEq, Debug)]
pub enum TransactionError {
    UnknownCommand(String),
    MissingKey,
    InvalidKey(String),
    MissingValue,
    InvalidValue(String),
    LogWriteFail(String),
    LogReadFail(String),
    LogLoadFail,
    KeyNotFound(String),
    RestoreError(String),
    OperationFailure(String),
    InternalError,
    SerializationFailure,
    MissingCommand,
    InvalidExpiration(i64),
}

impl TransactionError {
    pub fn message(&self) -> String {
        match self {
            TransactionError::UnknownCommand(command) => {
                format!("{}: Command {} not recognized", self.get_code(), command)
            }
            TransactionError::MissingKey => {
                format!("{}: Missing key", self.get_code())
            }
            TransactionError::InvalidKey(key) => {
                format!("{}: Invalid key \"{}\"", self.get_code(), key)
            }
            TransactionError::MissingValue => {
                format!("{}: Missing value", self.get_code())
            }
            TransactionError::InvalidValue(value) => {
                format!("{}: Invalid value {}", self.get_code(), value)
            }
            TransactionError::LogWriteFail(statement) => {
                format!(
                    "{}: Failed to write statement to log - {}",
                    self.get_code(),
                    statement
                )
            }
            TransactionError::LogReadFail(line) => {
                format!("{}: Failed to read line {} from log", self.get_code(), line)
            }
            TransactionError::LogLoadFail => {
                format!("{}: Failed to load logfile", self.get_code())
            }
            TransactionError::KeyNotFound(key) => {
                format!("{}: Key {} not found", self.get_code(), key)
            }
            TransactionError::RestoreError(message) => {
                format!(
                    "{}: Unable to restore database from logs - {}",
                    self.get_code(),
                    message
                )
            }
            TransactionError::OperationFailure(message) => {
                format!(
                    "{}: Unable to complete operation - {}",
                    self.get_code(),
                    message
                )
            }
            TransactionError::InternalError => {
                format!("{}: An internal server error occurred", self.get_code(),)
            }
            TransactionError::SerializationFailure => {
                format!("{}: Unable to serialize request into JSON", self.get_code())
            }
            TransactionError::MissingCommand => {
                format!("{}: Invalid command", self.get_code())
            }
            TransactionError::InvalidExpiration(timestamp) => {
                format!(
                    "{}: Invalid expiration {}",
                    self.get_code(),
                    timestamp.to_string()
                )
            }
        }
    }

    fn get_code(&self) -> String {
        match self {
            TransactionError::UnknownCommand(_) => String::from("E01"),
            TransactionError::MissingKey => String::from("E02"),
            TransactionError::InvalidKey(_) => String::from("E03"),
            TransactionError::MissingValue => String::from("E04"),
            TransactionError::InvalidValue(_) => String::from("E05"),
            TransactionError::LogWriteFail(_) => String::from("E06"),
            TransactionError::LogReadFail(_) => String::from("E07"),
            TransactionError::LogLoadFail => String::from("E08"),
            TransactionError::KeyNotFound(_) => String::from("E09"),
            TransactionError::RestoreError(_) => String::from("E10"),
            TransactionError::OperationFailure(_) => String::from("E11"),
            TransactionError::InternalError => String::from("E12"),
            TransactionError::SerializationFailure => String::from("E13"),
            TransactionError::MissingCommand => String::from("E14"),
            TransactionError::InvalidExpiration(_) => String::from("E15"),
        }
    }
}
