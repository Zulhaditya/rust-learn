fn main() {
    println!("Hello, world!");
    another_function(10, 'Z');

    let x = five();
    println!("Nilai dari x: {x}");

    let y = seven(6);
    println!("Nilai dari y + 1 = {y}");
}

// function = posisinya bisa dimana saja
// parameter wajib diberikan type
fn another_function(x: i32, unit_label: char) {
    println!("Nilai dari x: {x} dan unit_label: {unit_label}");

    // statement = instruksi yang menjalankan sebuah aksi dan tidak
    //             mengembalikan nilai dan diakhiri dengan (;)
    // expression = mengembalikan nilai dan tidak diakhiri dengan (;)

    let y = {
        let x = 3;
        x + 1
    };

    println!("Nilai y: {y}");
}

// fungsi yang mengembalikan nilai (expression)
fn five() -> i32 {
    5
}

fn seven(y: i32) -> i32 {
    y + 1
}
