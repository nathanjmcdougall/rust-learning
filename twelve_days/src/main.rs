/*
https://en.wikipedia.org/wiki/The_Twelve_Days_of_Christmas_(song)
*/

struct SongDay {
    // Getting an error about named lifetime, so using 'static here.
    // This just means these string slices live for the entire duration of the program,
    // which is true since they are hardcoded.
    day: &'static str,
    gift: &'static str,
}

const DAYS: [&SongDay; 12] = [
    &SongDay {
        day: "first",
        gift: "a partridge in a pear tree",
    },
    &SongDay {
        day: "second",
        gift: "two turtle doves",
    },
    &SongDay {
        day: "third",
        gift: "three french hens",
    },
    &SongDay {
        day: "fourth",
        gift: "four calling birds",
    },
    &SongDay {
        day: "fifth",
        gift: "five gold rings",
    },
    &SongDay {
        day: "sixth",
        gift: "six geese a laying",
    },
    &SongDay {
        day: "seventh",
        gift: "seven swans a swimming",
    },
    &SongDay {
        day: "eighth",
        gift: "eight maids a milking",
    },
    &SongDay {
        day: "ninth",
        gift: "nine ladies dancing",
    },
    &SongDay {
        day: "tenth",
        gift: "ten lords a leaping",
    },
    &SongDay {
        day: "eleventh",
        gift: "eleven pipers piping",
    },
    &SongDay {
        day: "twelfth",
        gift: "twelve drummers drumming",
    },
];

fn main() {
    for day_index in 0..DAYS.len() {
        let day = &DAYS[day_index];
        println!(
            "On the {} day of Christmas my true love sent to me",
            day.day
        );

        for gift_index in (0..=day_index).rev() {
            let gift = &DAYS[gift_index].gift;
            if gift_index == 0 && day_index != 0 {
                println!("And {}.", gift);
            } else if gift_index == 0 && day_index == 0 {
                println!("{}.", capitalize_gift(gift));
            } else {
                println!("{},", capitalize_gift(gift));
            }
        }

        println!();
    }
}

fn capitalize_gift(gift: &str) -> String {
    let mut chars = gift.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
