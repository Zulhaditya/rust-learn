fn main() {
    // mutable = bisa diubah nilainya
    // secara default variabel adalah immutable
    let mut x = 5;
    println!("Nilai dari x: {x}");
    x = 6;
    println!("Nilai dari x: {x}");

    // constant = pasti immutable tidak bisa diubah dengan mut keyword
    // penamaan constant selalu uppercase
    const TIGA_JAM_DALAM_DETIK: u32 = 60 * 60 * 3;
    println!("Waktu sekarang: {TIGA_JAM_DALAM_DETIK}");

    // shadowing = variabel bisa menggunakan nama yang sama
    let x = 7;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Nilai x di innerscope adalah: {x}");
    }

    println!("Nilai x sekarang adalah: {x}");

    let spasi = "    ";
    let spasi = spasi.len();
    println!("Jumlah spasi: {spasi}");
}
