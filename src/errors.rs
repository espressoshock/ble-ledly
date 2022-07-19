use thiserror::Error;

#[derive(Error, Debug)]
pub enum BluetoothError {
    #[error("the selected default bluetooth adapter [0] is invalid")]
    InvalidBluetoothAdapter,

    #[error(transparent)]
    InternalError(#[from] btleplug::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error("unable to unpack peripheral properties")]
    InvalidPeriperipheralProperty,

    #[error("invalid peripheral reference during service discovery")]
    InvalidPeripheralReference,

    #[error("no write characteristic found that satisfy target value")]
    NotFoundTargetCharacteristic,

    #[error("Invalid characteristic")]
    InvalidCharacteristic,
}

#[derive(Error, Debug)]
pub enum LightControlError {
    #[error("the supplied value is not within the appropriate range: [{0}]")]
    InvalidRange(String),

    #[error("the supplied value is not valid")]
    InvalidValue,
}
