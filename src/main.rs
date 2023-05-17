use num_cpus;
use sha256::digest;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::{fs, println};

fn get_file_as_byte_vec(filename: &String, size: u32) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");

    if metadata.len() <= size as u64 {
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        return buffer;
    } else {
        let mut buffer = vec![0; size as usize];
        f.read(&mut buffer).expect("buffer overflow");
        buffer
    }
}

fn main() {
    let number_of_cpus = num_cpus::get();
    let bytes_to_read = 16384;
    let paths = fs::read_dir("/Users/ariel/Movies").unwrap();
    let mut files_hashmap: HashMap<String, Vec<String>> = HashMap::new();

    let mut count = 0;
    for path in paths {
        let unwrapped_path = path.unwrap();
        if !unwrapped_path.path().is_dir() {
            let bytes = get_file_as_byte_vec(
                &unwrapped_path.path().to_str().unwrap().to_string(),
                bytes_to_read,
            );

            let hex_string = hex::encode(bytes);
            let digest = digest(hex_string);

            // digest acts as our key
            if files_hashmap.contains_key(&digest) {
                let file_array = files_hashmap.get_mut(&digest).unwrap();
                file_array.push(unwrapped_path.path().to_str().unwrap().to_string());
            } else {
                files_hashmap.insert(
                    digest.clone(),
                    vec![unwrapped_path.path().to_str().unwrap().to_string()],
                );
            }
        };
        count += 1;
    }
    println!("files count: {}", count);
    println!("files_hashmap count: {}", files_hashmap.len());
    println!("Hello, world! {} cpus", number_of_cpus);
    println!("files_hashmap: {:?}", files_hashmap);

    for (key, value) in files_hashmap.iter() {
        if value.len() > 5 {
            println!("key: {}, value: {:?}\n\n", key, value);
        }
    }
}
