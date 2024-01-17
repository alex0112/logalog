// mod question;
// use question::Question;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::Write;

fn main() {
    let nums: [u32; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    let prefixes: [&str; 5] = ["", "K", "M", "G", "T"];

    let mut streak: u8 = 0;

    loop {
        let (question, answer): (String, u32) = gen_question(&nums, &prefixes);

        println!("{streak} in a row CORRECT");
        print!("{question} ");
        io::stdout().flush().expect("Unable to flush output buffer");

        let mut user_in = String::new();

        io::stdin()
            .read_line(&mut user_in)
            .expect("Could not read from STDIN");

        let user_in = user_in.trim().parse::<u32>();

        match user_in {
            Ok(user_ans) => {
                if user_ans == answer {
                    streak += 1;
                    println!("Correct!\n")
                } else {
                    streak = 0;
                    println!("Nope.")
                }
            }
            Err(_e) => {
                println!("Please enter a valid number as your answer.");
                continue;
            }
        }
    }
}

fn gen_question(nums: &[u32; 10], prefixes: &[&str; 5]) -> (String, u32) {
    let prefix: &str = prefixes.choose(&mut thread_rng()).unwrap();
    //    let prefix_pow: u32 = power_from_prefix(prefix);
    let num: u32 = *nums.choose(&mut thread_rng()).unwrap();

    let qst: String = format!("log {}{} = ", num, prefix);

    let ans: u32 = 42;

    (qst, ans)
}

fn power_from_prefix(prefix: &str) -> u32 {
    todo!()
}

fn power_from_u32(num: u32) -> u32 {
    todo!()
}

// struct GameState {
//     streak: u32,
//     currentQuestion: Question,
// }
