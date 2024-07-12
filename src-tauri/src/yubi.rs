use std::str::FromStr;

use rsa::{
    pkcs1v15::EncryptingKey,
    pkcs8::{der::Encode, DecodePublicKey},
    traits::RandomizedEncryptor,
    RsaPublicKey,
};
use yubikey::{
    piv::{self, SlotId},
    Context, Serial, YubiKey,
};

use crate::AppState;

#[tauri::command]
pub fn list_yk() -> Result<Vec<String>, String> {
    let mut context = Context::open().map_err(|e| e.to_string())?;
    let readers = context.iter().map_err(|e| e.to_string())?;

    let mut connected_serials: Vec<String> = vec![];

    for reader in readers {
        let yubikey = reader.open().map_err(|e| e.to_string())?;
        let serial = yubikey.serial();
        connected_serials.push(serial.clone().to_string());
    }

    Ok(connected_serials)
}

#[tauri::command]
pub fn select_yubikey(state: tauri::State<'_, AppState>, serial: String) {
    *state.selected_serial.lock().unwrap() = Some(serial);
}

#[tauri::command]
pub fn encrypt_input(state: tauri::State<'_, AppState>, input: String) -> Result<String, String> {
    let state_serial = &*state.selected_serial.lock().unwrap();

    match state_serial {
        Some(serial_string) => {
            let serial = Serial::from_str(&serial_string).unwrap();
            let mut yubikey = YubiKey::open_by_serial(serial).map_err(|e| e.to_string())?;
            let target_slot = SlotId::KeyManagement;
            let slot_meta = piv::metadata(&mut yubikey, target_slot).unwrap();
            if let Some(key_info) = slot_meta.public {
                let key_der = &key_info.to_der().unwrap();
                let public_key = RsaPublicKey::from_public_key_der(&key_der).unwrap();
                let pkcs1v15 = EncryptingKey::new(public_key);
                let data_bytes = input.as_bytes();
                let encrypted_data = pkcs1v15
                    .encrypt_with_rng(&mut rand::thread_rng(), data_bytes)
                    .unwrap();
                // set managed state
                *state.encrypted_input.lock().unwrap() = Some(encrypted_data.clone());
                let loss_res = String::from_utf8_lossy(&encrypted_data).to_string();
                return Ok(loss_res);
            }
        }
        None => {
            return Err("Serial is not selected".into());
        }
    }

    Err("Not completed".into())
}

#[tauri::command]
pub fn decrypt_message(state: tauri::State<'_, AppState>, pin: String) -> Result<String, String> {
    let encrypted_state = &*state.encrypted_input.lock().unwrap();
    match encrypted_state {
        Some(encrypted_data) => {
            if let Some(stored_serial) = &*state.selected_serial.lock().unwrap() {
                let serial = Serial::from_str(&stored_serial).unwrap();
                let mut yubikey = YubiKey::open_by_serial(serial).unwrap();
                yubikey
                    .verify_pin(pin.as_bytes())
                    .map_err(|e| e.to_string())?;
                let decrypted_data = piv::decrypt_data(
                    &mut yubikey,
                    encrypted_data,
                    piv::AlgorithmId::Rsa2048,
                    SlotId::KeyManagement,
                )
                .unwrap();
                // Removing PKCS1 v1.5 padding manually
                if decrypted_data.len() < 11
                    || decrypted_data[0] != 0x00
                    || decrypted_data[1] != 0x02
                {
                    return Err("Decryption error: invalid PKCS1 v1.5 padding".into());
                }

                let mut i = 2;
                while i < decrypted_data.len() && decrypted_data[i] != 0 {
                    i += 1;
                }
                if i >= decrypted_data.len() {
                    return Err("Decryption error: invalid PKCS1 v1.5 padding".into());
                }

                let decrypted_content = &decrypted_data[(i + 1)..];
                let lossy_res = String::from_utf8_lossy(decrypted_content).to_string();
                return Ok(lossy_res);
            }
        }
        None => {
            return Err("Encrypted State Not Found".into());
        }
    }
    Err("Operation failed".into())
}
