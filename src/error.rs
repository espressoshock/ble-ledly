use thiserror::Error;

#[derive(Error, Debug)]
pub enum CapabilityError {
    #[error("Invalid peripheral reference")]
    Error,
}

/// Errors related to BLE communication
#[derive(Error, Debug)]
pub enum BluetoothError {
    #[error("The selected default bluetooth adapter [0] is invalid")]
    InvalidBluetoothAdapter,

    #[error("Invalid peripheral reference")]
    InvalidPeripheralReference,

    #[error("Unable to unpack peripheral properties")]
    InvalidPeriperipheralProperty,

    #[error("No write characteristic found that satisfy target value")]
    NotFoundTargetCharacteristic,

    #[error(transparent)]
    InternalError(#[from] btleplug::Error),

    #[error("Invalid or absent characteristic")]
    InvalidCharacteristic,
}
