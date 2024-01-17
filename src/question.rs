// #[derive(PartialEq, Debug)]
// enum Unit {
//     Blank,
//     K,
//     M,
//     G,
//     T,
// }

// pub struct Question {
//     num: u32,
//     unit: Unit,
//     prompt: String,
// }

// impl Question {
//     pub fn new(num: u32, unit: Unit) -> Self {
//         let prompt: String = generate_prompt(num, unit);

//         Question {
//             num,
//             unit,
//             prompt: generate_prompt(num, unit),
//         }
//     }

//     pub fn ask(self, prompter: fn() -> u32) -> bool {
//         todo!()
//     }

//     fn generate_prompt(num: u32, unit: Unit) -> String {
//         todo!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_new() {
//         assert_eq!(Question::new(42, Unit::K).num, 42);
//         assert_eq!(Question::new(42, Unit::K).unit, Unit::K);
//     }

//     #[test]
//     fn test_ask() {
//         // Test correct case
//         let question: Question = Question::new(42, Unit::K);
//         let user_in: fn() -> u32 = || 42u32;

//         // Test incorrect case
//     }
// }
