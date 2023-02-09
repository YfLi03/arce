use log::warn;
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyIvInit};

use crate::api::err;
use crate::api::config::GlobalConfig;

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;

const MAX_CONTENT: usize = 262144;

// Convert u8s to hexs
fn array2string(enc: &[u8]) -> Result<String, err::Error> {
    let mut s = String::new();
    for i in 0..enc.len() {
        s += &format!("{:x}{:x}", enc[i] / 16, enc[i] % 16)
    };
    Ok(s)
}



pub fn encrypt(content: String, password: String) -> Result<String, err::Error> {
    let config = GlobalConfig::global();

    // Right now the same iv is used for every article
    // A better way is to generate per article
    if let None = config.iv {
        warn!("IV not set. Cannot Encrypt Articles.");
        return Err(err::Error{
            reason: err::Reason::Internal,
            message: String::from("IV unavailable in the config file")
        })
    }
    let mut iv = [0x0; 16];
    iv.copy_from_slice(&config.iv.clone().unwrap().as_bytes());

    // Generate the key from Password with some simple method
    // Maybe i should switch to PBKDF2 in the future
    let mut key = [0x42; 16];
    let mut loc = 0;
    for i in 0..16 {
        key[i] = password.bytes().nth(loc).unwrap();
        loc = ( loc + 1 ) % password.len();
    }

    let mut buf = [0x0; MAX_CONTENT];
    let bytes = content.as_bytes();
    buf[..content.len()].copy_from_slice(bytes);

    let encrypted = Aes128CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, content.len())
        .unwrap();

    Ok(array2string(encrypted)?)
}