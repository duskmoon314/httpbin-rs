use base64::alphabet;
use base64::engine::general_purpose::{self, STANDARD, STANDARD_NO_PAD, URL_SAFE, URL_SAFE_NO_PAD};
use base64::engine::GeneralPurpose;
use base64::Engine;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Base64Engine {
    Standard,
    StandardNoPad,
    UrlSafe,
    UrlSafeNoPad,
    Bcrypt,
    BinHex,
    Crypt,
    ImapMutf7,
    Custom,
}

pub struct Base64Config {
    pub alphabet: String,
    pub pad: bool,
}

#[derive(Error, Debug)]
pub enum Base64Error {
    #[error("unknown engine: {0}")]
    UnknownEngine(String),
    #[error("the config is empty")]
    EmptyConfig,
    #[error("the alphabet is invalid: {0}")]
    InvalidAlphabet(#[from] alphabet::ParseAlphabetError),
    #[error("the base64 string is invalid: {0}")]
    InvalidBase64(#[from] base64::DecodeError),
}

/// Encode a `&[u8]` to a base64 string
pub fn encode(
    data: &[u8],
    engine: Base64Engine,
    config: Option<Base64Config>,
) -> Result<String, Base64Error> {
    let encoded = match engine {
        Base64Engine::Standard => STANDARD.encode(data),
        Base64Engine::StandardNoPad => STANDARD_NO_PAD.encode(data),
        Base64Engine::UrlSafe => URL_SAFE.encode(data),
        Base64Engine::UrlSafeNoPad => URL_SAFE_NO_PAD.encode(data),
        Base64Engine::Bcrypt => BCRYPT.encode(data),
        Base64Engine::BinHex => BIN_HEX.encode(data),
        Base64Engine::Crypt => CRYPT.encode(data),
        Base64Engine::ImapMutf7 => IMAP_MUTF7.encode(data),
        Base64Engine::Custom => {
            if config.is_none() {
                return Err(Base64Error::EmptyConfig);
            }

            let config = config.unwrap();

            let alphabet = alphabet::Alphabet::new(&config.alphabet)?;

            let engine = GeneralPurpose::new(
                &alphabet,
                if config.pad {
                    general_purpose::PAD
                } else {
                    general_purpose::NO_PAD
                },
            );

            engine.encode(data)
        }
    };

    Ok(encoded)
}

/// Decode a base64 string to a `Vec<u8>`
pub fn decode(
    data: &str,
    engine: Base64Engine,
    config: Option<Base64Config>,
) -> Result<Vec<u8>, Base64Error> {
    let decoded = match engine {
        Base64Engine::Standard => STANDARD.decode(data)?,
        Base64Engine::StandardNoPad => STANDARD_NO_PAD.decode(data)?,
        Base64Engine::UrlSafe => URL_SAFE.decode(data)?,
        Base64Engine::UrlSafeNoPad => URL_SAFE_NO_PAD.decode(data)?,
        Base64Engine::Bcrypt => BCRYPT.decode(data)?,
        Base64Engine::BinHex => BIN_HEX.decode(data)?,
        Base64Engine::Crypt => CRYPT.decode(data)?,
        Base64Engine::ImapMutf7 => IMAP_MUTF7.decode(data)?,
        Base64Engine::Custom => {
            if config.is_none() {
                return Err(Base64Error::EmptyConfig);
            }

            let config = config.unwrap();

            let alphabet = alphabet::Alphabet::new(&config.alphabet)?;

            let engine = GeneralPurpose::new(
                &alphabet,
                if config.pad {
                    general_purpose::PAD
                } else {
                    general_purpose::NO_PAD
                },
            );

            engine.decode(data)?
        }
    };

    Ok(decoded)
}

const BCRYPT: GeneralPurpose = GeneralPurpose::new(&alphabet::BCRYPT, general_purpose::NO_PAD);
const BIN_HEX: GeneralPurpose = GeneralPurpose::new(&alphabet::BIN_HEX, general_purpose::NO_PAD);
const CRYPT: GeneralPurpose = GeneralPurpose::new(&alphabet::CRYPT, general_purpose::NO_PAD);
const IMAP_MUTF7: GeneralPurpose =
    GeneralPurpose::new(&alphabet::IMAP_MUTF7, general_purpose::NO_PAD);
