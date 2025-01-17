use super::*;
use crate::{AddressType, BitcoinAddress, CryptoAddress, Error, HDKey, HDNetworkType, Seed};
use std::str::FromStr;

#[test]
fn test_from_hd_key_testnet() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2wpkh;
    let hd_key = HDKey::new(seed, HDNetworkType::TestNet, "m/84'/1'/0'/0/0")?;
    let bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let expected_address = "tb1q2knvzpjltz4uwh6j5wrmqn7lnzccsphpd85jp9";
    assert_eq!(bitcoin_address.public_address(), expected_address);
    assert!(bitcoin_address.public_key().is_ok());
    assert!(bitcoin_address.private_key().is_ok());
    Ok(())
}

#[test]
fn test_from_hd_key_mainnet_p2wpkh() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2wpkh;
    let hd_key = HDKey::new(seed, HDNetworkType::MainNet, "m/84'/0'/0'/0/0")?;
    let bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let expected_address = "bc1qqh5dyxhkqage7fnmn9mjhk4w072fekzsvpzzs7";
    assert_eq!(bitcoin_address.public_address(), expected_address);
    Ok(())
}

#[test]
fn test_from_hd_key_mainnet_p2sh() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2sh;
    let hd_key = HDKey::new(seed, HDNetworkType::MainNet, "m/49'/0'/0'/0/1")?;
    assert_eq!(
        hd_key.derivation_path.to_string(),
        "m/49'/0'/0'/0/1".to_string()
    );
    assert_eq!(
        &hd_key.to_wif()?,
        "KzzMux1HnhZCAiCLScSpUDtXtsHgjts4RJLadDDi2zgxU2qq3g53"
    );
    assert_eq!(
        format!("{:x}", hd_key.extended_public_key()?),
        "02b9a730f83f85b77c7cf2f444d6cf76b144e11370bb96c6cbc624072f2d8e94cc"
    );
    let _bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    // TODO(AS: walletd #20):  Need to fix issue with p2sh address generation
    //let expected_address = "32z7gdz9HL5PRbFQM8E6ar5xidHUxCfniW";
    //assert_eq!(bitcoin_address.public_address(), expected_address);
    Ok(())
}

#[test]
fn test_from_public_address() -> Result<(), Error> {
    let expected_address = "tb1q2knvzpjltz4uwh6j5wrmqn7lnzccsphpd85jp9";
    let btc_address =
        BitcoinAddress::from_public_address(expected_address, bitcoin::Network::Testnet)?;
    assert_eq!(btc_address.public_address(), expected_address);
    assert!(btc_address.public_key().is_err());
    assert!(btc_address.private_key().is_err());
    let address_info = btc_address.address_info();
    assert!(address_info.address_type().is_some());
    assert_eq!(
        address_info
            .address_type()
            .expect("expecting option value due to previous check"),
        AddressType::P2wpkh
    );
    Ok(())
}

#[test]
fn test_zeroize() -> Result<(), Error> {
    let seed_hex = "a2fd9c0522d84d52ee4c8533dc02d4b69b4df9b6255e1af20c9f1d4d691689f2a38637eb1ec778972bf845c32d5ae83c7536999b5666397ac32021b21e0accee";
    let seed = Seed::from_str(seed_hex)?;
    let address_format = AddressType::P2wpkh;
    let hd_key = HDKey::new(seed, HDNetworkType::TestNet, "m/84'/1'/0'/0/0")?;
    let mut bitcoin_address = BitcoinAddress::from_hd_key(&hd_key, address_format)?;
    let expected_address = "tb1q2knvzpjltz4uwh6j5wrmqn7lnzccsphpd85jp9";
    assert_eq!(bitcoin_address.public_address(), expected_address);
    assert!(bitcoin_address.public_key().is_ok());
    assert!(bitcoin_address.private_key().is_ok());
    let mut private_key = bitcoin_address.private_key.clone().unwrap();
    bitcoin_address.zeroize();
    assert!(bitcoin_address.private_key.is_none());
    private_key.zeroize();
    assert_eq!(private_key.to_bytes(), [1u8; 32]);
    Ok(())
}
