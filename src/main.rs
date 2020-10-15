mod rs_go;
fn main() {
    let doggo = rs_go::Doggo::initialize();
    println!(
        "hello, my name is {}, my age is {}",
        doggo.get_name(),
        *doggo.get_age()
    );
    println!(
        "i have friend, a {}. his name is {} and he's {} years old",
        doggo.get_friend(),
        doggo.get_friend_name(),
        *doggo.get_friend_age()
    );
    println!("awesome!")
}
