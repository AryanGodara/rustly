use std::collections::HashMap;

// Import the rand crate
use rand::{self, Rng};
// Import sha256
// use digest::Digest;
// use sha2::Sha256;

#[derive(Debug)]
struct Record {
    url: String,
    shortened_url: String,
}

// fn create_hash<D: Digest>(msg: &str, mut hasher: D) -> String
// where
//     D: Digest,
//     digest::Output<D>: std::fmt::LowerHex,
// {
//     hasher.update(msg);
//     let res = hasher.finalize();
//     format!("{:x}", res)
// }

fn generate_shortened_url(map: &mut HashMap<String,String>, url: &str) -> Record {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    
    // Run the loop for n times
    // let x = (0..length); // Range<usize>
    // let hasher = Sha256::default();
    // hasher.update(url);
    // let res = hasher.finalize();
    // let shortened_url = format!("{:x}", res);

    let shortened_url = (0..url.len())
        .map(|_| { // Closure with no arguments
            let index = rand::thread_rng().gen_range(0..CHARSET.len()-1);
            return CHARSET[index] as char // Convert u8 to char
        })
        .collect::<String>(); // Collect the iterator into a String

    // Insert the shortened url into the map
    map.insert(shortened_url.clone(), url.to_string());

    Record {
        url: url.to_string() ,
        shortened_url ,
    }
}

fn main() {
    let mut map: HashMap<String, String> = HashMap::new();

    let url = "https://www.google.com";
    let shortened_url = generate_shortened_url(&mut map, url);
    let shortened = shortened_url.shortened_url.clone();
    let res = map.get(&shortened).unwrap();
    println!("{} => {}", shortened, res);
    assert_eq!(res, &shortened_url.url);
}
