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
}

impl TransactionError {
    pub fn message(&self) -> String {
        match self {
            TransactionError::UnknownCommand(command) => {
                format!("{}: Command {} not recognized", get_code(self), command)
            }
            TransactionError::MissingKey => {
                format!("{}: Missing key", get_code(self))
            }
            TransactionError::InvalidKey(key) => {
                format!("{}: Invalid key \"{}\"", get_code(self), key)
            }
            TransactionError::MissingValue => {
                format!("{}: Missing value", get_code(self))
            }
            TransactionError::InvalidValue(value) => {
                format!("{}: Invalid value {}", get_code(self), value)
            }
            TransactionError::LogWriteFail(statement) => {
                format!(
                    "{}: Failed to write statement to log - {}",
                    get_code(self),
                    statement
                )
            }
            TransactionError::LogReadFail(line) => {
                format!("{}: Failed to read line {} from log", get_code(self), line)
            }
            TransactionError::LogLoadFail => {
                format!("{}: Failed to load logfile", get_code(self))
            }
            TransactionError::KeyNotFound(key) => {
                format!("{}: Key {} not found", get_code(self), key)
            }
            TransactionError::RestoreError(message) => {
                format!(
                    "{}: Unable to restore database from logs - {}",
                    get_code(self),
                    message
                )
            }
            TransactionError::OperationFailure(message) => {
                format!(
                    "{}: Unable to complete operation - {}",
                    get_code(self),
                    message
                )
            }
            TransactionError::InternalError(message) => {
                format!(
                    "{}: An internal server error occurred - {}",
                    get_code(self),
                    message
                )
            }
            TransactionError::SerializationFailure => {
                format!("{}: Unable to serialize request into JSON", get_code(self))
            }
        }
    }

    fn get_code(&self) -> String {
        match self {
            TransactionError::UnknownCommand(_) => String::from("E01"),
            TransactionError::MissingKey => Stirng::from("E02"),
            TransactionError::InvalidKey(_) => String::from("E03"),
            TransactionError::MissingValue => String::from("E04"),
            TransactionError::InvalidValue(_) => String::from("E05"),
            TransactionError::LogWriteFail(_) => String::from("E06"),
            TransactionError::LogReadFail(_) => String::from("E07"),
            TransactionError::LogLoadFail => String::from("E08"),
            TransactionError::KeyNotFound(_) => String::from("E09"),
            TransactionError::RestoreError(_) => String::from("E10"),
            TransactionError::OperationFailure(_) => String::from("E11"),
            TransactionError::InternalError(_) => String::from("E12"),
            TransactionError::SerializationFailure => String::from("E13"),
        }
    }
}
