fn main() {
    println!("Twelve Days of Christmas Song");
    print_lyrics();
}

fn print_lyrics() {
    for day in 1..=12 {
        println!("On the {} day of christmas...", get_ordinal(day));
    }
}

fn get_ordinal(day: u8) -> &'static str {
    match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    }
}
