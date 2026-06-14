use crate::core::config::ImageConfig;
use crate::core::types::BuiltInCharSet;

impl ImageConfig {
    pub fn get_processed_chars(&self) -> String {
        let mut chars = String::new();

        // If the flag indicates it, use all ASCII characters
        if let Some(dicts) = &self.dicts {
            for dict in dicts {
                match dict {
                    BuiltInCharSet::Default => {
                        chars.push_str("8dbqp'·. ");
                    }
                    BuiltInCharSet::All => {
                        chars.push_str(&(32..=126).map(|c| c as u8 as char).collect::<String>());
                    }
                    BuiltInCharSet::Symbols => {
                        chars.push_str(" @!¡+=-:.'");
                    }
                    // There is no good monospace font with braille characters included by default
                    // Might work on this later
                    // BuiltInCharSet::Braille => {
                    //     chars.push_str("⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠩⠩");
                    // }
                    BuiltInCharSet::Blocks => {
                        chars.push_str(" █▓▒░");
                    }
                    BuiltInCharSet::BlocksAll => {
                        chars.push_str(" ▀▁▂▃▄▅▆▇█▉▊▋▌▍▎▐░▒▓▔▕▖▗▘▙▚▛▜▝▞▟▏");
                    }
                    BuiltInCharSet::Box => {
                        chars.push_str("─│┌┐└┘├┤┬┴┼");
                    }
                    BuiltInCharSet::BoxAll => {
                        chars.push_str("─│┌┐└┘├┤┬┴┼╱╲╳╭╮╰╯");
                    }
                    BuiltInCharSet::BoxDouble => {
                        chars.push_str("═║╔╗╚╝╠╣╦╩╬");
                    }
                    BuiltInCharSet::BoxDoubleAll => {
                        chars.push_str("═║╔╗╚╝╠╣╦╩╬╱╲╳╭╮╰╯");
                    }
                    BuiltInCharSet::Nerd => {
                        chars.push_str(" ");
                    }
                    BuiltInCharSet::Math => {
                        chars.push_str(" ±×÷≈≠≤≥∞∑∏√∫∂∆∇");
                    }
                    BuiltInCharSet::Numbers => {
                        chars.push_str(" 0123456789");
                    }
                    BuiltInCharSet::Letters => {
                        chars.push_str(" ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
                    }
                }
            }
        } else {
            chars.push_str(&self.chars);
        }

        // Add the additional characters
        chars.push_str(&self.add_chars);

        // Remove the excluded characters
        chars = chars
            .chars()
            .filter(|c| !self.except.contains(*c))
            .collect();

        chars
    }
}
