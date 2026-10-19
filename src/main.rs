use std::fs;

fn main() {
    output_file_in_chunks("messages.txt", 8);
}

fn output_file_in_chunks(file_name: &str, chunk_size: usize) {
    let file_contents = fs::read(file_name);
    let mut buffer = vec![0u8; chunk_size];
    let mut buffer_index = 0;

    for byte in file_contents.unwrap() {
        buffer[buffer_index] = byte;
        buffer_index += 1;
        if buffer_index == chunk_size {
            buffer_index = 0;
            println!("{}", std::str::from_utf8(&buffer).unwrap());
        }
    }
}
