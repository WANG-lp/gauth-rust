use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use ring::{digest, hmac};
use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use std::time::SystemTime;
use std::env;
use std::path::PathBuf;

struct OTP {}

impl OTP {
    fn decode_base32(secret: &str) -> Option<Vec<u8>> {
        match base32::decode(base32::Alphabet::RFC4648 { padding: true }, secret) {
            Some(decode_res) => Some(decode_res),
            _ => None,
        }
    }
    pub fn get_code(secret: &str) -> Option<String> {
        let message = (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / 30) as u32;
        let key = OTP::decode_base32(secret)?;
        let mut message_body = vec![];

        message_body.write_u64::<BigEndian>(message as u64).ok();

        let s_key = hmac::SigningKey::new(&digest::SHA1, key.as_ref());
        let signature = hmac::sign(&s_key, &message_body);
        let hash = signature.as_ref();

        let offset = hash[hash.len() - 1] & 0x0F;
        let mut truncated_hash = (&hash[offset as usize..(offset + 4) as usize]).to_vec();
        truncated_hash[0] = truncated_hash[0] & 0x7f;

        let mut rdr = Cursor::new(truncated_hash);
        let mut code: u32 = rdr.read_u32::<BigEndian>().unwrap();

        code = code % 1_000_000u32;
        let mut code_str = code.to_string();
        for i in 0..(6 - code_str.len()) {
            code_str.insert(i, '0');
        }
        Some(code_str)
    }
}

fn main() {
    // Create a path to the desired file
    let args: Vec<String> = env::args().collect();
    let mut path = dirs::home_dir().unwrap();
    path.push(".config");
    path.push("gauth.csv");
    if args.len() > 1{
        path = PathBuf::from(&args[1]);
    }
    let path = path.as_path();
    let display = path.display();
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b':')
        .from_reader(file);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.unwrap();
        let name = &record[0].trim();
        let secret = &record[1].trim();
        let otop = OTP::get_code(secret);
        println!("{}: {}", name, otop.unwrap());
    }
    println!(
        "Time left: {}s",
        30 - (SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            % 30)
    );
}
