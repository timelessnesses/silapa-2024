use std::{fmt::Debug, io::Write, str::FromStr};

pub fn input<T: FromStr>(prompt: Option<impl AsRef<str>>) -> T
where
    <T as FromStr>::Err: Debug,
{
    if let Some(prompt) = prompt {
        print!("{}", prompt.as_ref());
        std::io::stdout().flush().unwrap();
    }
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().parse().unwrap()
}
