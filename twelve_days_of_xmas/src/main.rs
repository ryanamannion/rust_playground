// # print the 12 days of Christmas lyrics

// My python brain REALLLLY wanted to do this with string concatenation but i just could not figure
// out the ownership part of it. I have not gotten to that part of The Book yet, but I will
// eventually and maybe I will come back and re-implement this

fn suffix(n: i32) -> String {
    // can't do   -> &str      without "missing lifetime specifier", what's that?
    match n {
        1 => "st".to_owned(),   // not sure I really get why I need to do all this
        2 => "nd".to_owned(),
        3 => "rd".to_owned(),
        _ => "th".to_owned()
    }
}

// NB it seems like I needed to ^ "do all of [that]" ^ because of ownership and temporary variables?
// that is coming later in the book, so for now I did it all with printing

fn main() {
    let mut verse_num: i32 = 1;
    loop {
        if verse_num == 13 {
            break
        }
        print!("On the {verse_num}{} day of Christmas my true love gave to me:", suffix(verse_num));
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
            print!("{}{}", sep, given)
        };
        verse_num += 1;
        println!("\n===============================")
    }
}
