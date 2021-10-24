use std::env;
use std::process::Command;

const FOREGROUND: &str = env!("COLOR_FG");
const BACKGROUND: &str = env!("COLOR_BG");
const ACCENT: &str = env!("COLOR_A");

fn main() {
    match Command::new("bemenu")
        .args(&[
              "--ignorecase",
              "--list=10", "--bottom",
              "--fn", "Noto Sans 15",
              "-p", "",
              "--tb", ACCENT,    // Title background.
              "--tf", BACKGROUND,    // Title foreground.
              "--fb", ACCENT,    // Filter background.
              "--ff", BACKGROUND,    // Filter foreground
              "--nb", BACKGROUND,    // Normal background.
              "--nf", FOREGROUND,    // Normal foreground.
              "--hb", ACCENT,    // Highlighted background.
              "--hf", BACKGROUND,    // Highlighted foreground.
              "--sb", ACCENT,    // Selected background.
              "--sf", BACKGROUND    // Selected foreground.
        ])
        .status() {
            Err(why) => panic!("Error running dmenu's process: {}", why),
            Ok(_) => ()
	};
}
