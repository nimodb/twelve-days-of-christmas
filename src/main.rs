fn main() {
    println!("Twelve Days of Christmas Song");
    print_lyrics();
}

fn print_lyrics() {
    for day in 1..=12 {
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            get_ordinal(day)
        );
        print_gifts(day);
        println!();
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

fn print_gifts(day: u8) {
    for current_day in (1..=day).rev() {
        match current_day {
            12 => println!("12 Drummers Drumming"),
            11 => println!("11 Pipers Piping"),
            10 => println!("10 Lords a Leaping"),
            9 => println!("9 Ladies Dancing"),
            8 => println!("8 Maids a Milking"),
            7 => println!("7 Swans a Swimming"),
            6 => println!("6 Geese a Laying"),
            5 => println!("5 Golden Rings"),
            4 => println!("4 Calling Birds"),
            3 => println!("3 French Hens"),
            2 => println!("2 Turtle Doves"),
            1 => {
                if day == 1 {
                    println!("A Partridge in a Pear Tree");
                } else {
                    println!("Add a Partridge in a Pear Tree");
                }
            }
            _ => (),
        }
    }
}
