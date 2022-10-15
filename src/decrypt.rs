use super::*;

pub fn decrypt(encrypted_file_path: String, secret: String) -> Result<(), String> {
    let encrypted_content = read(encrypted_file_path.clone()).unwrap();
    let secret = read(secret).unwrap();

    let secret: Result<Secret, serde_json::Error> = serde_json::from_slice(&secret);

    match secret {
        Ok(secret) => match AES::new_from_slices(&secret.decode_key(), &secret.decode_iv()) {
            Ok(cipher) => {
                let mut buffer = base64::decode(encrypted_content).unwrap();
                let decrypted_data = cipher.decrypt(&mut buffer).unwrap();

                println!("{}", str::from_utf8(decrypted_data).unwrap());

                write(encrypted_file_path, str::from_utf8(decrypted_data).unwrap()).unwrap();

                Ok(())
            }
            Err(err) => return Err(err.to_string()),
        },
        Err(err) => return Err(err.to_string()),
    }
}
