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


//////// これ以降はユーザーID /////////

pub fn generate_user_id(ip_addr: &str) -> String {

    let charset: Vec<char> = (&SETTING.id_charset).chars().collect();
    let length = SETTING.id_length;
    let prefix = &SETTING.id_raw_prefix;

    let date = chrono::Local::now().format("%Y/%m/%d").to_string();

    let id_raw = format!("{} {} {}", prefix, ip_addr, &date);

    let mut hasher = Sha512::new();

    let _ = hasher.write(id_raw.as_bytes());

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

                    if let Some(p) = charset.get(index as usize % charset.len()) {
                        buffer.push(*p);
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