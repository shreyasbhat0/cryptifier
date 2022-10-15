use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    key: String,
    inital_value: String,
}

impl Secret {
    pub fn new(out_path: String) -> Result<(), Error> {
        let mut key = [0u8; 16];
        thread_rng().fill_bytes(&mut key[..]);

        let mut inital_value = [0u8; 16];
        thread_rng().fill_bytes(&mut inital_value[..]);

        let secret = Secret {
            key: base64::encode(key),
            inital_value: base64::encode(inital_value),
        };

        match to_vec(&secret) {
            Ok(secret) => match File::create(out_path) {
                Ok(mut file) => match file.write_all(&secret) {
                    Ok(()) => {
                        println!("Secret Generated Successfully");
                        Ok(())
                    }
                    Err(err) => {
                        return Err(err);
                    }
                },
                Err(err) => {
                    panic!("fail to create a file {}", err)
                }
            },
            Err(err) => {
                panic!("failed to serilaize {}", err)
            }
        }
    }

    pub fn decode_key(&self) -> Vec<u8> {
        base64::decode(&self.key).unwrap()
    }

    pub fn decode_iv(&self) -> Vec<u8> {
        base64::decode(&self.inital_value).unwrap()
    }
}
