/// Exits the program
///
/// This macro calls exit if running in normal mode, and panics if called in a test.
/// This is because tests require an explicit panic, whereas a user who typed an argument wrong
/// does not want to see a panic.
#[macro_export]
macro_rules! exit {
    ($x: literal) => {
        #[cfg(test)]
        panic!();
        #[cfg(not(test))]
        std::process::exit($x);
    };
    () => {
        #[cfg(test)]
        panic!();
        #[cfg(not(test))]
        std::process::exit(0);
    };
}

#[macro_export]
macro_rules! unwrap_option {
    ($input: expr, $error_message: expr, $($log_message: expr),+) => {
        {
            if $input.is_none() {
                let log_message = format!($($log_message),+);
                let log_message = format!("{} | {}:{}", log_message, file!(), line!());
                log::error!("{}", log_message);
                println!("{}", console::style($error_message).red().bold());
                exit!(1);
            }

            $input.unwrap()
        }
    };

    ($input: expr, $error_message: expr) => {
        {
            if $input.is_none() {
                let log_message = format!("{} | {}:{}", $error_message.clone(), file!(), line!());
                log::error!("{}", log_message.clone());
                println!("{}", console::style($error_message).red().bold());
                exit!(1);
            }

            $input.unwrap()
        }
    };
}

#[macro_export]
macro_rules! unwrap_result {
    ($input: expr, $error_message: expr, $($log_message: expr),+) => {
        {
            if $input.is_err() {
                let log_message = format!($($log_message),+);
                let log_message = format!("{} | {}:{}", log_message.clone(), file!(), line!());
                log::error!("{}", log_message);
                println!("{}", console::style($error_message).red().bold());
                exit!(1);
            }

            $input.unwrap()
        }
    };

    ($input: expr, $error_message: expr) => {
        {
            if $input.is_err() {
                let log_message = format!("{} | {}:{}", $error_message, file!(), line!());
                log::error!("{}", log_message);
                println!("{}", console::style($error_message).red().bold());
                exit!(1);
            }

            $input.unwrap()
        }
    };
}
