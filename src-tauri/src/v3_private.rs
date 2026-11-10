use argon2::{Algorithm,Argon2,Params,Version};
use aes_gcm::{Aes256Gcm,KeyInit,Nonce,aead::{Aead,Payload}};
#[tauri::command]
pub fn unlock_v3_poi(password:String)->Result<serde_json::Value,String>{
 let b=include_bytes!("v3_private.bin");
 if &b[..4]!=b"DPV3" {return Err("Invalid encrypted resource".into());}
 let mut key=[0u8;32];
 let argon=Argon2::new(Algorithm::Argon2id,Version::V0x13,Params::new(65536,3,1,Some(32)).map_err(|_|"KDF configuration error")?);
 argon.hash_password_into(password.as_bytes(),&b[4..20],&mut key).map_err(|_|"Key derivation error")?;
 let cipher=Aes256Gcm::new_from_slice(&key).map_err(|_|"Cipher initialization error")?;
 let result=cipher.decrypt(Nonce::from_slice(&b[20..32]),Payload{msg:&b[32..],aad:b"DAO_V3_POI"}).map_err(|_|"Sai mật khẩu")?;
 key.fill(0);
 serde_json::from_slice(&result).map_err(|_|"Invalid POI data".into())
}
