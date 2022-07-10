fn main() {
    for day in 1..13 {

        let ordinal_suffix = ordinal_suffix_of(day);
        println!("On the {day}{ordinal_suffix} day of Christmas, my true love gave to me:");

        for gift in (0..day).rev() {
            match gift {
                0 => if day > 1 { println!("And a patridge in a pear tree"); } else { println!("A patridge in a pear tree"); },
                1 => println!("Two turtledoves"),
                2 => println!("Three French hens"),
                3 => println!("Four mockingbirds"),
                4 => println!("Five golden rings"),
                5 => println!("Six geese a-laying"),
                6 => println!("Seven swans a-swimming"),
                7 => println!("Eight maids a-milking"),
                8 => println!("Nine ladies dancing"),
                9 => println!("Ten lords a-leaping"),
                10 => println!("Eleven pipers piping"),
                _ => println!("Twelve drummers drumming"),
            }
        }

        println!("---")
    }
}

fn ordinal_suffix_of(day: u32) -> &'static str {
    match day {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}
