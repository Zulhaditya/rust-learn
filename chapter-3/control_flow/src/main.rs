fn main() {
    println!("Learn control flow");

    let angka = 7;

    if angka < 5 {
        println!("{angka} kecil dari 5");
    } else {
        println!("{angka} lebih besar dari 5");
    }

    let angka_utama = 6;

    if angka_utama % 2 == 0 {
        println!("Angka bisa dibagi 2");
    } else if angka_utama % 4 == 0 {
        println!("Angka bisa dibagi 4");
    } else if angka_utama % 6 == 0 {
        println!("Angka bisa dibagi 6");
    } else {
        println!("Angka tidak bisa dibagi dengan 2, 4, dan 6!");
    }

    let kondisi = true;
    let angka_sekunder = if kondisi { 5 } else { 6 };
    println!("Nilai dari angka sekunder: {angka_sekunder}");

    // loops = akan dieksekusi terus sampai diberhentikan (break)
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Hasilnya: {result}");

    // nested loop
    let mut counter = 0;
    'counting_up: loop {
        println!("count = {counter}");
        let mut sisa = 10;

        loop {
            println!("tersisa = {sisa}");
            if sisa == 9 {
                break;
            }

            if counter == 2 {
                break 'counting_up;
            }

            sisa -= 1;
        }
        counter += 1;
    }

    println!("Akhir perhitungan = {counter}");

    // while loop
    let mut gaji = 10;
    while gaji != 5 {
        println!("{gaji}!");

        gaji -= 1;
    }

    println!("GAJIAN CUK!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("Nilai array: {}", a[index]);

        index += 1;
    }

    // for loop
    let b = [100, 200, 300, 400, 500];

    for element in b {
        println!("Nilai array: {element}");
    }

    for numbers in (1..5).rev() {
        println!("{numbers}");
    }
    println!("LIFTOFF");
}
