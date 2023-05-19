use num_cpus;
use sha256::digest;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::thread;

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

fn get_files_from_directory(directory_path: &str) -> Vec<String> {
    fs::read_dir(directory_path)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if e.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    path.into_os_string().into_string().ok()
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn search() -> HashMap<String, Vec<String>> {
    println!("Searching...");
    let number_of_cpus = num_cpus::get();
    let bytes_to_read = 16384 * 4;

    let paths = get_files_from_directory("/Users/ariel/Movies");

    let files_hashmap: Arc<Mutex<HashMap<String, Vec<String>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let shared_paths = Arc::new(Mutex::new(paths));

    let thread_handles: Vec<_> = (0..number_of_cpus)
        .map(|_| {
            let shared_strings = Arc::clone(&shared_paths); // Clone the Arc to move into the thread
            let hashmap = Arc::clone(&files_hashmap);
            thread::spawn(move || {
                loop {
                    let string_opt;
                    {
                        let mut strings = shared_strings.lock().unwrap(); // Acquire the lock to access the list
                        if strings.is_empty() {
                            break; // Exit the loop if the list is empty
                        }
                        string_opt = strings.pop(); // Get the next string from the list
                    }

                    if let Some(string) = string_opt {
                        let bytes = get_file_as_byte_vec(&string, bytes_to_read);
                        let hex_string = hex::encode(bytes);
                        let digest = digest(hex_string);

                        let mut map = hashmap.lock().unwrap();
                        if map.contains_key(&digest) {
                            let file_array = map.get_mut(&digest).unwrap();
                            file_array.push(string);
                        } else {
                            map.insert(digest.clone(), vec![string]);
                        }
                    }
                }
            })
        })
        .collect();

    for handle in thread_handles {
        handle.join().unwrap();
    }

    return Arc::try_unwrap(files_hashmap)
        .unwrap()
        .into_inner()
        .unwrap();
}
