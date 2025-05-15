// import library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Tebak angka!");

    let angka_rahasia = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input angka tebakan kamu: ");

        // buat variabel untuk menyimpan user input
        // variabel di rust bersifat immutable: value tidak berubah
        let mut tebakan = String::new(); // String::new() akan return nilai string

        io::stdin()
            .read_line(&mut tebakan)
            .expect("Gagal membaca data!");

        // konversi string ke integer
        let tebakan: u32 = match tebakan.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Kamu menebak: {}", tebakan);

        // bandingkan angka
        match tebakan.cmp(&angka_rahasia) {
            Ordering::Less => println!("Terlalu kecil!"),
            Ordering::Greater => println!("Terlalu besar!"),
            Ordering::Equal => {
                println!("Kamu menang!");
                break;
            }
        }
    }
}
