fn main() {
    println!("Twelve Days of Christmas Song");
    print_lyrics();
}

fn print_lyrics() {
    for day in 1..=12 {
        println!("On the {day} day of christmas...");
    }
}
