#![allow(dead_code)]
use sodiumoxide::crypto::secretbox;
use std::fs;
use std::path::Path;
use anyhow::Context;

const KEY_PATH: &str = "/etc/ola/secret.key";

pub fn ensure_key() -> anyhow::Result<secretbox::Key> {
    sodiumoxide::init().map_err(|_| anyhow::anyhow!("sodium init failed"))?;
    if Path::new(KEY_PATH).exists() {
        let b = fs::read(KEY_PATH).context("reading key")?;
        if b.len() != secretbox::KEYBYTES { return Err(anyhow::anyhow!("bad key length")); }
        Ok(secretbox::Key::from_slice(&b).unwrap())
    } else {
        anyhow::bail!("Secret key missing at {}. Create it as root during install.", KEY_PATH);
    }
}

pub fn save_secure(path: &str, data: &[u8]) -> anyhow::Result<()> {
    let key = ensure_key()?;
    let nonce = secretbox::gen_nonce();
    let cipher = secretbox::seal(data, &nonce, &key);
    // store nonce + ciphertext
    let mut out = nonce.0.to_vec();
    out.extend_from_slice(&cipher);
    fs::write(path, &out).context("writing secure file")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = fs::metadata(path) {
            let mut perm = metadata.permissions();
            perm.set_mode(0o600);
            fs::set_permissions(path, perm).ok();
        }
    }
    Ok(())
}

pub fn load_secure(path: &str) -> anyhow::Result<Vec<u8>> {
    let key = ensure_key()?;
    let b = fs::read(path).context("reading secure file")?;
    if b.len() < secretbox::NONCEBYTES { return Err(anyhow::anyhow!("file too short")); }
    let (nonce_bytes, cipher) = b.split_at(secretbox::NONCEBYTES);
    let mut nonce_arr = [0u8; secretbox::NONCEBYTES];
    nonce_arr.copy_from_slice(nonce_bytes);
    let nonce = secretbox::Nonce(nonce_arr);
    let plain = secretbox::open(&cipher, &nonce, &key).map_err(|_| anyhow::anyhow!("decryption failed"))?;
    Ok(plain)
}
