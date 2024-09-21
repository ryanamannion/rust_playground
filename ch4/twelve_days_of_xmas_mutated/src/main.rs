// # print the 12 days of Christmas lyrics

fn suffix(n: i32) -> String {
    match n {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th"
    }.to_owned()
}

fn main() {
    let mut verse_num: i32 = 1;
    loop {
        if verse_num == 13 {
            break
        }
        let suff: String = suffix(verse_num);
        let mut lyric: String = String::from(
            format!("On the {verse_num}{suff} day of Christmas my true love gave to me:")
        );
        for day in (1..=verse_num).rev() {
            let given: &str = match day {
                1 => "a partridge in a pear tree",
                2 => "two turtle doves",
                3 => "three French hens",
                4 => "four calling birds",
                5 => "five golden rings",
                6 => "six geese a-laying",
                7 => "seven swans a-swimming",
                8 => "eight maids a-milking",
                9 => "nine ladies dancing",
                10 => "ten lords a-leaping",
                11 => "eleven pipers piping",
                12 => "twelve drummers drumming",
                _ => "something else??"
            };
            let sep: &str = if day == verse_num {
                " "
            } else if day == 1{
                ", and "
            } else {
                ", "
            };
            lyric.push_str(sep);
            lyric.push_str(given);
        };
        println!("{lyric}");
        verse_num += 1;
    }
}
