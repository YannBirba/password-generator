use derivative::Derivative;
use rand::{thread_rng, Rng};
use std::{
    fmt::Error,
    fs::File,
    io::{stdin, stdout, Write},
    time::SystemTime,
};

fn read_user_input() -> String {
    let mut user_input = String::new();

    print!("Password generator ('help' to see options, 'quit' to quit) : ");

    let _ = stdout().flush();

    stdin()
        .read_line(&mut user_input)
        .expect("Did not enter a correct string :(");

    if let Some('\n') = user_input.chars().next_back() {
        user_input.pop();
    }
    if let Some('\r') = user_input.chars().next_back() {
        user_input.pop();
    }

    return user_input;
}

fn parse_user_input(user_input: String) -> Result<(Password, i32, bool, FileFormat), Error> {
    let mut password = Password::default();
    let mut nb_of_passwords = 1;
    let mut save_in_file = false;
    let mut file_format = FileFormat::Csv;

    let splited_user_input: Vec<&str> = user_input.split(" ").collect();

    let mut user_inputs = Vec::new();

    for ui in splited_user_input {
        if !ui.is_empty() && ui != "" {
            user_inputs.push(ui);
        }
    }

    for ui in user_inputs {
        let splited_ui: Vec<&str> = ui.split("=").collect();

        let key = splited_ui[0];
        let mut value: &str = "";
        if splited_ui.len() > 1 {
            value = splited_ui[1];
        }

        match key {
            "file" | "f" => {
                save_in_file = true;
                match value {
                    "txt" => {
                        file_format = FileFormat::Txt;
                    }
                    "csv" => {
                        file_format = FileFormat::Csv;
                    }
                    _ => {
                        file_format = FileFormat::Csv;
                    }
                }
            }
            "number_of_passwords" | "nop" | "n" => {
                let number_of_passwords = value.parse::<i32>().unwrap();
                nb_of_passwords = number_of_passwords;
                if nb_of_passwords > 25 {
                    save_in_file = true;
                }
            }
            "size" | "s" | "sz" => {
                let size = value.parse::<usize>().unwrap();
                password.size = size;
            }
            "!use_numbers" | "!n" | "!num" => {
                password.use_numbers = false;
            }
            "use_numbers" | "un" | "num" => {
                let use_numbers = value.parse::<bool>().unwrap();
                password.use_numbers = use_numbers;
            }
            "!use_uppercase" | "!up" | "!u" => {
                password.use_uppercase = false;
            }
            "use_uppercase" | "up" | "u" => {
                let use_uppercase = value.parse::<bool>().unwrap();
                password.use_uppercase = use_uppercase;
            }
            "!use_special_caracters" | "!sc" | "!c" => {
                password.use_special_caracters = false;
            }
            "use_special_caracters" | "sc" | "c" => {
                let use_special_caracters = value.parse::<bool>().unwrap();
                password.use_special_caracters = use_special_caracters;
            }
            _ => (),
        }
    }

    return Ok((password, nb_of_passwords, save_in_file, file_format));
}

enum FileFormat {
    Txt,
    Csv,
}

fn save_password_in_file(file: &mut File, index: i32, password: String, format: &FileFormat) {
    match format {
        FileFormat::Txt => {
            file.write_all(format!("\nPassword {} :\n{}\n", index + 1, password).as_bytes())
                .unwrap();

            file.sync_all().unwrap();

            return;
        }
        FileFormat::Csv => {
            file.write_all(format!("{},{}\n", index + 1, password).as_bytes())
                .unwrap();

            file.sync_all().unwrap();

            return;
        }
    }
}

fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

fn generate_password() {
    let user_input = read_user_input();

    match user_input.as_str() {
        "!quit" | "quit" | "q" => {
            std::process::exit(0);
        }
        "!help" | "help" | "h" => {
            println!(
                "
|-----------------------------------------------------------------------|
| size=15 (to set the size of the password)                             |
| use_numbers=true (to use numbers in the password)                     |
| use_uppercase=true (to use uppercase in the password)                 |
| use_special_caracters=true (to use special caracters in the password) |
|-----------------------------------------------------------------------|
"
            );
            return;
        }
        _ => (),
    }
    let parsed_user_input = parse_user_input(user_input).unwrap();

    if parsed_user_input.2 && parsed_user_input.1 > 1 {
        let mut file: File;
        let filename = format!("passwords_{}", get_sys_time_in_secs());
        let filefullname: String;
        match parsed_user_input.3 {
            FileFormat::Txt => {
                filefullname = format!("{}.txt", filename);
                file = File::create(filefullname).unwrap();
            }
            FileFormat::Csv => {
                filefullname = format!("{}.csv", filename);
                file = File::create(filefullname).unwrap();
                file.write_all("Id,Password\n".as_bytes()).unwrap();
                file.sync_all().unwrap();
            }
        }
        for i in 0..parsed_user_input.1 {
            let password = Password::generate(&parsed_user_input.0);

            match password {
                Ok(password) => {
                    save_password_in_file(&mut file, i, password, &parsed_user_input.3);
                }
                Err(error) => println!("Error: {}", error),
            }
        }
    } else {
        for i in 0..parsed_user_input.1 {
            let password = Password::generate(&parsed_user_input.0);

            match password {
                Ok(password) => {
                    if parsed_user_input.1 > 1 {
                        println!("\nPassword {} :\n\n{}\n", i + 1, password);
                    } else {
                        println!("\nYour password is :\n\n{}\n", password);
                    }
                }
                Err(error) => println!("Error: {}", error),
            }
        }
    }
}

fn main() {
    let mut user_input = "r".to_string();

    while user_input.contains("r") || user_input.contains("R") {
        user_input = String::new();
        generate_password();
        print!("(r) restart | (anything) to leave: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut user_input)
            .expect("Did not enter a correct string :(");
    }
}
#[derive(Derivative)]
#[derivative(Debug, Default)]
struct Password {
    #[derivative(Default(value = "15"))]
    size: usize,
    #[derivative(Default(value = "true"))]
    use_numbers: bool,
    #[derivative(Default(value = "true"))]
    use_uppercase: bool,
    #[derivative(Default(value = "true"))]
    use_special_caracters: bool,
}

impl Password {
    pub fn generate(password: &Self) -> Result<String, Error> {
        const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
        const NUMBERS: &str = "0123456789";
        const SPECIAL_CARACTERS: &str = "!@#$%^&*()_+";
        const UPPERCASES: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        let mut chartset = String::new();

        if password.use_numbers {
            chartset.push_str(NUMBERS);
        }
        if password.use_uppercase {
            chartset.push_str(UPPERCASES);
        }
        if password.use_special_caracters {
            chartset.push_str(SPECIAL_CARACTERS);
        }

        chartset.push_str(LETTERS);

        let chartset = chartset.as_bytes();

        let generated_password: String;

        let mut rng = thread_rng();

        generated_password = (0..password.size)
            .map(|_| {
                let idx = rng.gen_range(0..chartset.len());
                chartset[idx] as char
            })
            .collect();

        return Ok(generated_password);
    }
}
