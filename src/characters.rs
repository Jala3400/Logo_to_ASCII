use crate::config::ImageConfig;
use crate::types::BuiltInCharSet;

pub fn process_characters(config: &mut ImageConfig) {
    // If the flag indicates it, use all ASCII characters
    if let Some(dicts) = config.dicts.clone() {
        config.chars.clear();
        for dict in dicts {
            match dict {
                BuiltInCharSet::Default => {
                    config.chars.push_str("8dbqp'·. ");
                }
                BuiltInCharSet::All => {
                    config
                        .chars
                        .push_str(&(32..=126).map(|c| c as u8 as char).collect::<String>());
                }
                BuiltInCharSet::Symbols => {
                    config.chars.push_str(" @!¡+=-:.'");
                }
                // There is no good monospace font with braille characters included by default
                // Might work on this later
                // BuiltInCharSet::Braille => {
                //     config.chars.push_str("⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿ ");
                // }
                BuiltInCharSet::Blocks => {
                    config.chars.push_str(" █▓▒░");
                }
                BuiltInCharSet::BlocksAll => {
                    config.chars.push_str(" ▀▁▂▃▄▅▆▇█▉▊▋▌▍▎▐░▒▓▔▕▖▗▘▙▚▛▜▝▞▟▏");
                }
                BuiltInCharSet::Box => {
                    config.chars.push_str("─│┌┐└┘├┤┬┴┼");
                }
                BuiltInCharSet::BoxAll => {
                    config.chars.push_str("─│┌┐└┘├┤┬┴┼╱╲╳╭╮╰╯");
                }
                BuiltInCharSet::BoxDouble => {
                    config.chars.push_str("═║╔╗╚╝╠╣╦╩╬");
                }
                BuiltInCharSet::BoxDoubleAll => {
                    config.chars.push_str("═║╔╗╚╝╠╣╦╩╬╱╲╳╭╮╰╯");
                }
                BuiltInCharSet::Nerd => {
                    config.chars.push_str(" ");
                }
                BuiltInCharSet::Math => {
                    config.chars.push_str(" ±×÷≈≠≤≥∞∑∏√∫∂∆∇");
                }
                BuiltInCharSet::Numbers => {
                    config.chars.push_str(" 0123456789");
                }
                BuiltInCharSet::Letters => {
                    config
                        .chars
                        .push_str(" ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
                }
            }
        }
    }

    // Add the additional characters
    config.chars.push_str(&config.add_chars);

    // Remove the excluded characters
    config.chars = config
        .chars
        .chars()
        .filter(|c| !config.except.contains(*c))
        .collect();
}
