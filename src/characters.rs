use crate::args::Args;
use crate::types::BuiltInCharSet;

pub fn process_characters(args: &mut Args) {
    // If the flag indicates it, use all ASCII characters
    if let Some(dicts) = &args.dicts {
        args.chars.clear();
        for dict in dicts {
            match dict {
                BuiltInCharSet::All => {
                    args.chars
                        .push_str(&(32..=126).map(|c| c as u8 as char).collect::<String>());
                }
                BuiltInCharSet::Symbols => {
                    args.chars.push_str(" @!¡+=-:.'");
                }
                // There is no good monospace font with braille characters included by default
                // Might work on this later
                // BuiltInCharSet::Braille => {
                //     args.chars.push_str("⠁⠂⠃⠄⠅⠆⠇⠈⠉⠊⠋⠌⠍⠎⠏⠐⠑⠒⠓⠔⠕⠖⠗⠘⠙⠚⠛⠜⠝⠞⠟⠠⠡⠢⠣⠤⠥⠦⠧⠨⠩⠪⠫⠬⠭⠮⠯⠰⠱⠲⠳⠴⠵⠶⠷⠸⠹⠺⠻⠼⠽⠾⠿ ");
                // }
                BuiltInCharSet::Blocks => {
                    args.chars.push_str(" █▓▒░");
                }
                BuiltInCharSet::BlocksAll => {
                    args.chars.push_str(" ▀▁▂▃▄▅▆▇█▉▊▋▌▍▎▐░▒▓▔▕▖▗▘▙▚▛▜▝▞▟▏");
                }
                BuiltInCharSet::Box => {
                    args.chars.push_str("─│┌┐└┘├┤┬┴┼");
                }
                BuiltInCharSet::BoxAll => {
                    args.chars.push_str("─│┌┐└┘├┤┬┴┼╱╲╳╭╮╰╯");
                }
                BuiltInCharSet::BoxDouble => {
                    args.chars.push_str("═║╔╗╚╝╠╣╦╩╬");
                }
                BuiltInCharSet::BoxDoubleAll => {
                    args.chars.push_str("═║╔╗╚╝╠╣╦╩╬╱╲╳╭╮╰╯");
                }
                BuiltInCharSet::Nerd => {
                    args.chars.push_str(" ");
                }
                BuiltInCharSet::Math => {
                    args.chars.push_str(" ±×÷≈≠≤≥∞∑∏√∫∂∆∇");
                }
                BuiltInCharSet::Numbers => {
                    args.chars.push_str(" 0123456789");
                }
                BuiltInCharSet::Letters => {
                    args.chars
                        .push_str(" ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");
                }
            }
        }
    }

    // Add the additional characters
    args.chars.push_str(&args.add_chars);

    // Remove the excluded characters
    args.chars = args
        .chars
        .chars()
        .filter(|c| !args.except.contains(*c))
        .collect();
}
