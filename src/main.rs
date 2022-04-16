mod crypto_test;

fn main() {
    println!("(e)ncrypt data or (d)ecrypt data? ");

    let mut answer = String::new();

    std::io::stdin().read_line(&mut answer).expect("not allowed");

    let mut raw_str = String::new();

    if &answer[0..1] == "e" {
        println!("you have chosen encryption");
        println!("What would you like to encrypt? Please type it here");
        std::io::stdin().read_line(&mut raw_str).expect("errorroro");
        crypto_test::encrypt_data(&raw_str[..].as_bytes());
    } else if &answer[0..1] == "d" {
        println!("you have chosen decryption");
        println!("Decrypting...");
        crypto_test::decrypt_file("./test-encrypted.txt", "einszweieinszwei".as_bytes());
    } else {
        println!("You have to type e or d, try again..");
    }
   
   
}
