use crypto::aes;
use crypto::buffer::RefReadBuffer;
use crypto::buffer::*;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;


const MSG_L : usize = 16;
// Learning: For aes-128 the Key and IV have to be of length 16 (in bytes).

// &[u8] means as much as byte array.
pub fn encrypt_data(data: &[u8]) {
    let key = "einszweieinszwei".as_bytes();
    let iv = "myivmyivmyivmyiv".as_bytes();

    println!("Bytes to be encrypted: {:?}", data);

    let mut encryptor = aes::cbc_encryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        crypto::blockmodes::PkcsPadding,
    );

    let mut write_buf = [0; MSG_L];

    let mut input_buffer_ref = RefReadBuffer::new(data);
    let mut output_buffer_ref = crypto::buffer::RefWriteBuffer::new(&mut write_buf);
    let output_f = fs::File::create("./test-encrypted.txt");
    let mut buffered_file_writer = BufWriter::new(output_f.unwrap());

    loop {
        let encrypted_data = encryptor.encrypt(&mut input_buffer_ref, &mut output_buffer_ref, true);

        buffered_file_writer
            .write(&output_buffer_ref.take_read_buffer().take_remaining())
            .unwrap();
        match encrypted_data {
            Ok(crypto::buffer::BufferResult::BufferUnderflow) => break,
            Ok(_) => {}
            Err(err) => panic!("{:?}", err),
        }
    }
}

pub fn decrypt_file(fp: &str, key: &[u8]) {
    let input_f = fs::File::open(fp).unwrap();
    let mut f_reader = BufReader::new(input_f);
    let mut r_buffer = [0; MSG_L];
    let mut write_buf = [0; MSG_L];
    
    let iv = "myivmyivmyivmyiv".as_bytes();
        let mut decryptor = aes::cbc_decryptor(
            aes::KeySize::KeySize128,
            key,
            iv,
            crypto::blockmodes::PkcsPadding,
        );

        let output_f = fs::File::create("decrypted.txt").unwrap();
        let mut output_writer = BufWriter::new(output_f);

    loop {
        f_reader.read(&mut r_buffer).unwrap();
        println!("read bytes from file: {:?}", &r_buffer);

        let mut input_buffer_ref = RefReadBuffer::new(&r_buffer);
        let mut output_buffer_ref = RefWriteBuffer::new(&mut write_buf);

        let dec_result = decryptor.decrypt(&mut input_buffer_ref, &mut output_buffer_ref, true);
        output_writer.write(&output_buffer_ref.take_read_buffer().take_remaining()).unwrap();
        match dec_result {
            Ok(crypto::buffer::BufferResult::BufferUnderflow) => break,
            Err(err) => panic!("{:?}", err),
            Ok(_) => {}
        }
    }
        

}
