use crate::blockstream::{BTransaction, Input, Output, Utxo};
use crate::{BitcoinAddress, BitcoinAmount, BitcoinPrivateKey, BitcoinWallet, CryptoTx, Error};
use bitcoin::{ScriptBuf, TxIn, TxOut, Witness};
use std::str::FromStr;

/// Represents any Bitcoin transaction in WalletD.
#[derive(Clone)]
pub struct BitcoinTx {
    tx: bitcoin::Transaction,
    signers_addresses: Vec<String>,
}

/// Contains the specifications for a preparing a Bitcoin transaction.
#[derive(Clone)]
pub struct BitcoinTxParameters {
    send_amount: BitcoinAmount,
    to_public_address: String,
    fee_sat_per_byte: f64,
    utxo_available: Vec<Utxo>,
    change_address: bitcoin::Address,
    inputs_available_tx_info: Vec<BTransaction>,
    network: bitcoin::Network,
}

impl TryFrom<Input> for TxIn {
    type Error = Error;
    fn try_from(input: Input) -> Result<Self, Self::Error> {
        let mut witness_slices: Vec<Vec<u8>> = Vec::new();
        for witness in input.witness {
            let bytes = hex::decode(witness)?;
            witness_slices.push(bytes);
        }

        Ok(Self {
            previous_output: bitcoin::OutPoint::new(
                bitcoin::Txid::from_str(&input.txid)?,
                input.vout,
            ),
            script_sig: ScriptBuf::from_hex(&input.scriptsig)?,
            sequence: bitcoin::Sequence::from_consensus(input.sequence),
            witness: Witness::from_slice(witness_slices.as_slice()),
        })
    }
}

impl TryFrom<Output> for TxOut {
    type Error = Error;
    fn try_from(output: Output) -> Result<Self, Self::Error> {
        Ok(Self {
            value: output.value,
            script_pubkey: ScriptBuf::from_hex(&output.scriptpubkey)?,
        })
    }
}

impl TryFrom<BTransaction> for BitcoinTx {
    type Error = Error;
    fn try_from(tx: BTransaction) -> Result<Self, Self::Error> {
        let mut tx_inputs = vec![];
        for input in tx.vin {
            tx_inputs.push(TxIn::try_from(input)?);
        }
        let mut tx_outputs = vec![];

        Ok(Self {
            tx: bitcoin::Transaction {
                version: tx.version,
                lock_time: bitcoin::absolute::LockTime::from_consensus(tx.locktime),
                input: tx_inputs,
                output: tx_outputs,
            },
            signers_addresses: vec![],
        })
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitcoinSigningParameters {
    private_key: BitcoinPrivateKey,
}

impl CryptoTx for BitcoinTx {
    type ErrorType = Error;
    type CryptoAmount = BitcoinAmount;
    type TxParameters = BitcoinTxParameters;
    type SigningParameters = BitcoinSigningParameters;

    fn prepare_tx(tx_parameters: &Self::TxParameters) -> Result<Self, Self::ErrorType> {
        let receiver_view_wallet = BitcoinAddress::from_public_address(
            &tx_parameters.to_public_address,
            tx_parameters.network,
        )?;
        let prepared = BitcoinWallet::prepare_transaction(
            tx_parameters.fee_sat_per_byte,
            &tx_parameters.utxo_available,
            &tx_parameters.inputs_available_tx_info,
            &tx_parameters.send_amount,
            &receiver_view_wallet,
            tx_parameters.change_address.clone(),
        )?;

        let tx: BitcoinTx = prepared.0.try_into()?;

        todo!()
    }

    fn sign_tx(
        &self,
        signing_parameters: &BitcoinSigningParameters,
    ) -> Result<Self, Self::ErrorType> {
        unimplemented!()
    }

    fn validate_tx(&self) -> Result<(), Self::ErrorType> {
        unimplemented!()
    }
}
