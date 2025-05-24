struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct PersegiPanjang {
    panjang: u32,
    lebar: u32,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Zulhaditya"),
        email: String::from("zulhaditya@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("Inayah");
    println!("{}", user1.email);

    build_user(String::from("Ackxle"), String::from("ackxle@email.com"));

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };
    //
    // // struct with tuple
    // struct Warna(i32, i32, i32);
    // struct Point(i32, i32, i32);
    //
    // let black = Warna(0, 0, 0);
    // let origin = Point(0, 0, 0);

    // let width = 30;
    // let height = 50;
    //
    // println!("Luas persegi panjang: {}", area(width, height));

    let persegi_panjang_tuple = (30, 50);

    println!(
        "Luas persegi panjang dengan tuple: {}",
        area_tuple(persegi_panjang_tuple)
    );

    let persegi_panjang_struct = PersegiPanjang {
        panjang: 10,
        lebar: 30,
    };

    println!(
        "Luas persegi panjang dengan struct: {}",
        area_struct(&persegi_panjang_struct)
    );
}

// fungsi dengan tuple
fn area_tuple(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

// fungsi dengan struct
fn area_struct(persegi_panjang: &PersegiPanjang) -> u32 {
    persegi_panjang.panjang * persegi_panjang.lebar
}
