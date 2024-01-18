use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::Write;

fn main() {
    let nums: [u32; 10] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    let prefixes: [&str; 5] = ["", "K", "M", "G", "T"];

    println!("##### Logarithm quizzer #####\n");
    quiz(&nums, &prefixes);
}

fn quiz(nums: &[u32; 10], prefixes: &[&str; 5]) {
    let mut streak: u8 = 0;

    'new_question: loop {
        let (question, answer): (String, u32) = gen_question(nums, prefixes);

        println!("{streak} in a row CORRECT");
        io::stdout().flush().expect("Unable to flush output buffer");

        // If user input is invalid, don't throw the question away. Ask again.
        'input: loop {
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
                        println!("Correct!\n");
                        continue 'new_question;
                    } else {
                        streak = 0;
                        println!("Nope. Correct answer was {answer}\n");
                        break 'input;
                    }
                }
                Err(_e) => {
                    println!("Please enter a valid number as your answer.");
                    continue 'input;
                }
            }
        }
    }
}

fn gen_question(nums: &[u32; 10], prefixes: &[&str; 5]) -> (String, u32) {
    let prefix: &str = prefixes.choose(&mut thread_rng()).unwrap();
    let prefix_pow: u32 = power_from_prefix(prefix);
    let num: u32 = *nums.choose(&mut thread_rng()).unwrap();

    let qst: String = format!("log {}{} =", num, prefix);
    let ans: u32 = prefix_pow + num.ilog2();

    (qst, ans)
}

fn power_from_prefix(prefix: &str) -> u32 {
    match prefix {
        "" => 0u32,
        "K" => 10u32,
        "M" => 20u32,
        "G" => 30u32,
        "T" => 40u32,
        _ => unreachable!(),
    }
}
