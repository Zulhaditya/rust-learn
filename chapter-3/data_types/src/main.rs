fn main() {
    let angka_1 = 10;
    let angka_2 = 20;

    let hasil = angka_1 + angka_2;
    println!("Hasil penjumlahan: {hasil}");

    let pengurangan = 121.5 - 39.42;
    println!("Hasil pengurangan: {pengurangan}");

    let pembagian = 300 / 5;
    println!("Hasil pembagian: {pembagian}");

    let perkalian = 2898 * 72;
    println!("Hasil perkalian: {perkalian}");

    let modulo = 120 % 30;
    println!("Hasil modulo: {modulo}");

    let t = true;
    let f: bool = false;

    println!("Nilai t: {t}");
    println!("Nilai f: {f}");

    // char menggunakan single quote membedakannya dengan string
    let c = 'z';
    let z: char = 'Z';
    let emoji_kucing = '😻';
    println!("{} {} {}", c, z, emoji_kucing);

    // tuple = gabungan number dengan tipe data yang berbeda
    // length tuple fix, jika sudah dibuat maka tidak bisa ditambah atau dikurangi
    let tup: (i32, f64, u8) = (500, 7.9, 1);

    // destructuring. simbol _ digunakan untuk variabel yang opsional
    let (_tup1, tup2, _tup3) = tup;
    println!("Nilai dari tup2: {tup2}");

    let nilai: (i32, f64, u8) = (500, 6.4, 1);
    let lima_ratus = nilai.0;
    let enam_koma_empat = nilai.1;
    let satu = nilai.2;

    let test_nilai = lima_ratus * 10;
    println!(
        "Hasil dari test nilai: {} - {} - {}",
        test_nilai, enam_koma_empat, satu
    );

    // array = tipe data harus sama dan fix length
    let bulan = [
        "Januari",
        "Februari",
        "Maret",
        "April",
        "Mei",
        "Juni",
        "Juli",
        "Agustus",
        "September",
        "Oktober",
        "November",
        "Desember",
    ];

    println!("{}", bulan[0]);

    let lima_array: [i32; 5] = [1, 2, 3, 4, 5];
    let first = lima_array[0];
    let second = lima_array[1];

    println!("{first}");
    println!("{second}");
}
