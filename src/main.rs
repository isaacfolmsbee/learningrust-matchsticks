use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Matchsticks! If you're running this you know the rules :D\n");

    println!("Choose a number (min 10):");

    let mut max_matches = String::new();
    let max_matches: u32 = loop {
        io::stdin()
            .read_line(&mut max_matches)
            .expect("Failed to read line");

        let max_matches: u32 = max_matches.trim().parse().expect("You must enter a number!");

        if max_matches >= 10 {
            break max_matches;
        } else {
            println!("Minimum 10. Enter another number:")
        }
    };

    let mut matches_left: u32 = if max_matches == 10 {
        10
    } else {
        rand::thread_rng().gen_range(10..max_matches)
    };

    while matches_left > 0 {
        println!("There are {} matches left", matches_left);

        let matches_chose: u32 = loop {
            println!("Choose 1-3 matches to remove:");

            let mut matches_chose = String::new();

            io::stdin()
                .read_line(&mut matches_chose)
                .expect("Failed to read line");

            let matches_chose: u32 = matches_chose
                .trim()
                .parse()
                .expect("You must enter a number!");

            if matches_chose >= 1 && matches_chose <= 3 {
                break matches_chose;
            }
        };

        if matches_chose >= matches_left {
            println!("You removed the last match and lost :(");
            break;
        } else {
            matches_left -= matches_chose;

            println!(
                "You removed {} matches, there are {} left.",
                matches_chose, matches_left
            );
        }

        if matches_left == 1 {
            println!("You won! The bot grabbed the last match");
            break;
        }

        let bot_choice: u32 = if (matches_left - 1) % 4 == 0 {
            rand::thread_rng().gen_range(1..3)
        } else {
            (matches_left - 1) % 4
        };

        matches_left -= bot_choice;
        println!(
            "The bot removed {} matches, there are {} left.",
            bot_choice, matches_left
        );
    }
}
