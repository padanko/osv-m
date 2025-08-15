use std::io::Write;

use sha2::{Digest, Sha512};

use chrono::Local;
use rand::random_range;

use crate::SETTING;

pub fn epoch_time() -> String {
    Local::now().timestamp().to_string()
}

pub fn random_integer(digit_count: usize) -> String {
    let mut digits: Vec<u8> = Vec::new();

    for _ in 0..digit_count {
        digits.push(random_range(0..=9));
    }

    digits
        .iter().map(|digit| digit.to_string())
        .collect::<Vec<String>>().join("")
}

pub fn generate_id_from_char_and_length(seed: Option<&str>, length: usize, charset: &str) -> String {

    let charset: Vec<char> = charset.chars().collect();
    let length = length;

    let seed = match seed {
        Some(seed) => seed.to_string(),
        None => {
            // seedがNoneならランダム生成
            let mut buffer = String::new();
            
            for _ in 0..length {
                let index = random_range(0..charset.len());

                if let Some(char_) = charset.get(index) {
                    buffer.push(*char_);
                }
            }

            return buffer;
        }
    };

    let mut hasher = Sha512::new();

    let _ = hasher.write(seed.as_bytes());

    match hasher.flush() {
        Ok(()) => {
            let bytes_hash = hasher.finalize().to_vec();
            let mut count = 0;

            let mut buffer = String::new();

            for _ in 0..bytes_hash.len() {
                
                if let (Some(upper), Some(lower)) = (bytes_hash.get(count), bytes_hash.get(count+1)) {
                    let upper = (*upper) as u16;
                    let lower = (*lower) as u16;

                    let index = (upper << 8) | lower;

                    if let Some(char_) = charset.get(index as usize % charset.len()) {
                        buffer.push(*char_);
                    } else {
                        buffer.push('?');
                    }

                    count += 2;
                }

                if count/2 > length {
                    break;
                }

            }

            buffer
        },
        Err(_) => {
            String::from("???")
        }
    }

}

//////////////////// これ以降は用途別 ////////////////////

pub fn generate_topic_id() -> String {
    match SETTING.topic_id_digcount {
        Some(digit_count) => {
            random_integer(digit_count)
        },
        None => {
            epoch_time()
        }
    }
}


pub fn generate_user_view_id(ip_addr: &str) -> String {
    let date = &Local::now().format("%Y-%m-%d").to_string();
    let prefix = &SETTING.id_raw_prefix;
    let length = SETTING.id_length;
    let charset = &SETTING.id_charset;

    generate_id_from_char_and_length(
        Some(&format!("{} {} {}", prefix, date, ip_addr)),
        length,
        charset
    )
}