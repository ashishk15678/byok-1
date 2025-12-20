#[macro_export]
macro_rules! info {
    ($($expr:expr),*) => {
      {  use colored::Colorize;

        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("{} {} {}", timestamp.bold(), "[INFO]  : ".cyan(), format!("{}", $($expr),*).cyan());
      }
    };
}

#[macro_export]
macro_rules! error {
    ($($expr:expr),*) => {
       { use colored::Colorize;

        // Get the current local time and format it
        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("{} {} {}", timestamp.bold() , "[ERROR] : ".red(), format!("{:?}", $($expr),*).red());
       }
    };
}

#[macro_export]
macro_rules! warn {
    ($($expr:expr),*) => {
{        use colored::Colorize;

        // Get the current local time and format it
        let now = chrono::Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        println!("{} {} {}", timestamp.bold(), "[WARN]  : ".yellow(), format!("{:?}", $($expr),*).yellow());
       }
    };
}

#[macro_export]
macro_rules! debug {
    ($($expr:expr),*) => {
{
    use crate::config::DEBUG;
        if DEBUG {
            use colored::Colorize;

            // Get the current local time and format it
            let now = chrono::Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

            println!("{} {} {}", timestamp.bold(), "[DEBUG] : ".blue(), format!("{:?}", $($expr),*).blue());
        }
    }
    };
}
