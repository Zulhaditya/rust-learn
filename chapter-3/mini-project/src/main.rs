use core::f32;
use std::io::{self, Write};

fn main() {
    println!("\n ======== Mini project chapter-3 ========== \n");

    loop {
        // menu utama
        println!("\n1. Konversi suhu");
        println!("2. Generate angka fibonacci");
        println!("3. Deret Aritmatika");
        println!("4. Hitung Body Mass Index (BMI)");
        println!("5. Keluar\n");
        oneline_input();

        let mut pilih_menu = String::new();

        io::stdin()
            .read_line(&mut pilih_menu)
            .expect("Gagal membaca input!");

        // konversi string ke integer
        let pilih_menu: u32 = match pilih_menu.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // control flow
        if pilih_menu == 1 {
            println!("\nFitur konversi suhu:\n");
            println!("1. Celcius -> Fahrenheit");
            println!("2. Fahrenheit -> Celcius");
            println!("3. Celcius ->Kelvin");
            println!("4. Kelvin -> Celcius");
            println!("5. Fahrenheit -> Kelvin");
            println!("6. Kelvin -> Fahrenheit");
            println!("7. Kembali ke menu utama\n");
            oneline_input();

            let mut pilih_konversi = String::new();

            io::stdin()
                .read_line(&mut pilih_konversi)
                .expect("Gagal membaca data!");

            let pilih_konversi: u32 = match pilih_konversi.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if pilih_konversi == 1 {
                print!("Input celcius: ");
                io::stdout().flush().expect("Gagal menampilkan prompt!");

                let mut suhu = String::new();
                io::stdin()
                    .read_line(&mut suhu)
                    .expect("Gagal membaca data!");

                match suhu.trim().parse::<i32>() {
                    Ok(suhu) => println!("{} celcius = {} fahrenheit\n", suhu, c_to_f(suhu)),
                    Err(_) => println!("Error: masukkan angka yang valid\n"),
                };
            } else if pilih_konversi == 2 {
                print!("Input fahrenheit: ");
                io::stdout().flush().expect("Gagal menampilkan prompt!");

                let mut suhu = String::new();
                io::stdin()
                    .read_line(&mut suhu)
                    .expect("Gagal membaca data!");

                match suhu.trim().parse::<i32>() {
                    Ok(suhu) => println!("{} fahrenheit = {} celcius\n", suhu, f_to_c(suhu)),
                    Err(_) => println!("Error: masukkan angka yang valid\n"),
                };
            } else if pilih_konversi == 3 {
                print!("Input celcius: ");
                io::stdout().flush().expect("Gagal menampilkan prompt!");

                let mut suhu = String::new();
                io::stdin()
                    .read_line(&mut suhu)
                    .expect("Gagal membaca data!");

                match suhu.trim().parse::<f32>() {
                    Ok(suhu) => println!("{} celcius = {} kelvin\n", suhu, c_to_k(suhu)),
                    Err(_) => println!("Error: masukkan angka yang valid\n"),
                };
            } else if pilih_konversi == 4 {
                print!("Input kelvin: ");
                io::stdout().flush().expect("Gagal menampilkan prompt!");

                let mut suhu = String::new();
                io::stdin()
                    .read_line(&mut suhu)
                    .expect("Gagal membaca data!");

                match suhu.trim().parse::<f32>() {
                    Ok(suhu) => println!("{} kelvin = {} celcius\n", suhu, k_to_c(suhu)),
                    Err(_) => println!("Error: masukkan angka yang valid\n"),
                };
            } else if pilih_konversi == 5 {
                print!("Input fahrenheit: ");
                io::stdout().flush().expect("Gagal menampilkan prompt!");

                let mut suhu = String::new();
                io::stdin()
                    .read_line(&mut suhu)
                    .expect("Gagal membaca data!");

                match suhu.trim().parse::<f32>() {
                    Ok(suhu) => println!("{} fahrenheit = {} kelvin\n", suhu, f_to_k(suhu)),
                    Err(_) => println!("Error: masukkan angka yang valid\n"),
                };
            } else if pilih_konversi == 6 {
                print!("Input kelvin: ");
                io::stdout().flush().expect("Gagal menampilkan prompt!");

                let mut suhu = String::new();
                io::stdin()
                    .read_line(&mut suhu)
                    .expect("Gagal membaca data!");

                match suhu.trim().parse::<f32>() {
                    Ok(suhu) => println!("{} kelvin = {} fahrenheit\n", suhu, k_to_f(suhu)),
                    Err(_) => println!("Error: masukkan angka yang valid\n"),
                };
            } else {
                continue;
            }
        } else if pilih_menu == 2 {
            print!("Input angka: ");
            io::stdout().flush().expect("Gagal menampilkan prompt!");

            let mut angka_fibonacci = String::new();

            io::stdin()
                .read_line(&mut angka_fibonacci)
                .expect("Gagal membaca data!");

            match angka_fibonacci.trim().parse::<u32>() {
                Ok(angka_fibonacci) => println!(
                    "Fibonacci ke-{} adalah {}",
                    angka_fibonacci,
                    fibonacci(angka_fibonacci)
                ),
                Err(_) => println!("Error: masukkan angka yang valid\n"),
            };
        } else if pilih_menu == 3 {
            // Input Suku Pertama (a)
            print!("Suku pertama: ");
            io::stdout().flush().unwrap();
            let mut a = String::new();
            io::stdin().read_line(&mut a).unwrap();
            let a: i32 = a.trim().parse().unwrap();

            // Input Selisih Angka (b)
            print!("Selisih angka: ");
            io::stdout().flush().unwrap();
            let mut b = String::new();
            io::stdin().read_line(&mut b).unwrap();
            let b: i32 = b.trim().parse().unwrap();

            // Input Jumlah Suku (n)
            print!("Jumlah suku: ");
            io::stdout().flush().unwrap();
            let mut n = String::new();
            io::stdin().read_line(&mut n).unwrap();
            let n: i32 = n.trim().parse().unwrap();

            println!("\nSuku ke {} adalah {}", n, suku_ke_n(a, b, n));
            println!("Jumlah {} suku pertama: {}", n, jumlah_n_suku(a, b, n));
        } else if pilih_menu == 4 {
            println!("Keluar dari program");
            break;
        }
    }
}

fn oneline_input() {
    print!("Pilih: ");
    io::stdout().flush().expect("Gagal menampilkan prompt!");
}

fn c_to_f(c: i32) -> i32 {
    (c * 9 / 5) + 32
}

fn f_to_c(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn c_to_k(c: f32) -> f32 {
    c + 273.15
}

fn k_to_c(k: f32) -> f32 {
    k - 273.15
}

fn f_to_k(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0 + 273.15
}

fn k_to_f(k: f32) -> f32 {
    (k - 273.15) * 9.0 / 5.0 + 32.0
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn suku_ke_n(a: i32, b: i32, n: i32) -> i32 {
    a + (n - 1) * b
}

fn jumlah_n_suku(a: i32, b: i32, n: i32) -> i32 {
    n * (2 * a + (n - 1) * b) / 2
}
