use super::*;

pub fn encrypt(file_path: String, key_path: String) -> Result<(), String> {
    let to_encrypt = read(file_path.clone()).unwrap();
    let secret = read(key_path).unwrap();
    let secret: Result<Secret, serde_json::Error> = serde_json::from_slice(&secret);

    match secret {
        Ok(secret) => match AES::new_from_slices(&secret.decode_key(), &secret.decode_iv()) {
            Ok(cipher) => {
                let pos = to_encrypt.len();

                let mut buffer = vec![0u8; pos + pos];
                buffer[..pos].copy_from_slice(&to_encrypt);
                let encrypted_data = cipher.encrypt(&mut buffer, pos).unwrap();

                write(file_path, base64::encode(encrypted_data)).unwrap();
                Ok(())
            }
            Err(err) => return Err(err.to_string()),
        },
        Err(err) => return Err(err.to_string()),
    }
}
