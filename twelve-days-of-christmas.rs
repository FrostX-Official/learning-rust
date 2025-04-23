fn first_line_day(day: u8) {
    let day_str: &str = match day {
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
    };

    println!("On the {day_str} day of Christmas, my true love sent to me");
}

fn gift(gift_index: u8) {
    let gift_str = match gift_index {
        1 => "A partridge in a pear tree",
        2 => "Two turtle doves and",
        3 => "Three french hens",
        4 => "Four calling birds",
        5 => "Five golden rings",
        6 => "Six geese a-laying",
        7 => "Seven swans a-swimming",
        8 => "Eight maids a-milking",
        9 => "Nine ladies dancing",
        10 => "Ten lords a-leaping",
        11 => "Eleven pipers piping",
        12 => "Twelve drummers drumming",
        _ => "",
    };

    println!("{gift_str}");
}

fn main() {
    println!("TWELVE DAYS OF CHRISTMAS:");
    println!("");

    for day in 1..13 {
        first_line_day(day);
        for gift_day in (1..(day + 1)).rev() {
            gift(gift_day);
        }
        println!(" ");
    }
}
