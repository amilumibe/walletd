use crate::{Error, EthereumAmount};
use walletd_coin_core::CryptoTx;

pub struct EthTxParameters;
pub struct EthSigningParameters;

/// Represents a transaction on the Ethereum blockchain.
#[derive(Debug, Clone)]
pub struct EthTx;

impl CryptoTx for EthTx {
    type ErrorType = Error;
    type TxParameters = EthTxParameters;
    type CryptoAmount = EthereumAmount;
    type SigningParameters = EthSigningParameters;

    fn prepare_tx(_tx_parameters: &Self::TxParameters) -> Result<Self, Self::ErrorType> {
        unimplemented!()
    }

    fn sign_tx(
        &self,
        _signing_parameters: &Self::SigningParameters,
    ) -> Result<Self, Self::ErrorType> {
        unimplemented!()
    }

    fn validate_tx(&self) -> Result<(), Self::ErrorType> {
        unimplemented!()
    }
}
