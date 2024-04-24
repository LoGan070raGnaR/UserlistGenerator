use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn generate_root_list_lowercase(wordlist: &str) -> Vec<String> {
    let mut names = Vec::new();
    let file = File::open(wordlist).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(name) = line {
            names.push(name.trim().to_lowercase());
        }
    }
    names
}

fn lowercase_transformations(names: &[String]) {
    for line in names {
        let parts: Vec<&str> = line.split_whitespace().collect();
        println!("{}", parts[0]); // James Howlett -> james
        println!("{}", parts[1]); // James Howlett -> howlett
        println!("{}.{}", &parts[0][0..1], parts[1]); // James Howlett -> j.howlett
        println!("{}-{}", &parts[0][0..1], parts[1]); // James Howlett -> j-howlett
        println!("{}_{}", &parts[0][0..1], parts[1]); // James Howlett -> j_howlett
        println!("{}+{}", &parts[0][0..1], parts[1]); // James Howlett -> j+howlett
        println!("{}{}", &parts[0][0..1], parts[1]); // James Howlett -> jhowlett
        println!("{}{}", parts[0], parts[1]); // James Howlett -> jameshowlett
        println!("{}{}", parts[1], parts[0]); // James Howlett -> howlettjames
        println!("{}.{}", parts[0], parts[1]); // James Howlett -> james.howlett
        println!("{}.{}", parts[1], parts[0]); // James Howlett -> howlett.james
    }
}

fn uppercase_transformations(names: &[String]) {
    for line in names {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let first_word = parts[0];
        let second_word = parts[1];
        println!("{}", first_word.to_uppercase()); // James Howlett -> JAMES
        println!("{}", second_word.to_uppercase()); // James Howlett -> HOWLETT
        println!("{}.{}", &first_word[0..1], second_word.to_uppercase()); // James Howlett -> J.Howlett
        println!("{}_{}", &first_word[0..1], second_word.to_uppercase()); // James Howlett -> J_Howlett
        println!("{}-{}", &first_word[0..1], second_word.to_uppercase()); // James Howlett -> J-Howlett
        println!("{}{}", &first_word[0..1], second_word.to_uppercase()); // James Howlett -> JHowlett
        println!("{}{}", first_word.to_uppercase(), second_word.to_uppercase()); // James Howlett -> JAMESHOWLETT
        println!("{}{}", second_word.to_uppercase(), first_word.to_uppercase()); // James Howlett -> HOWLETTJAMES
        println!("{}", first_word.to_uppercase()); // James Howlett -> JAMES
        println!("{}", second_word.to_uppercase()); // James Howlett -> HOWLETT
        println!("{}{}", first_word.to_uppercase(), second_word.to_uppercase()); // James Howlett -> JAMESHOWLETT
    }
}

fn print_help() {
    println!("╔╗ ╔╗           ╔╗        ╔╗ ╔═══╗                     ╔╗        ");
    println!("║║ ║║           ║║       ╔╝╚╗║╔═╗║                    ╔╝╚╗       ");
    println!("║║ ║║╔══╗╔══╗╔═╗║║ ╔╗╔══╗╚╗╔╝║║ ╚╝╔══╗╔═╗ ╔══╗╔═╗╔══╗ ╚╗╔╝╔══╗╔═╗");
    println!("║║ ║║║══╣║╔╗║║╔╝║║ ╠╣║══╣ ║║ ║║╔═╗║╔╗║║╔╗╗║╔╗║║╔╝╚ ╗║  ║║ ║╔╗║║╔╝");
    println!("║╚═╝║╠══║║║═╣║║ ║╚╗║║╠══║ ║╚╗║╚╩═║║║═╣║║║║║║═╣║║ ║╚╝╚╗ ║╚╗║╚╝║║║ ");
    println!("╚═══╝╚══╝╚══╝╚╝ ╚═╝╚╝╚══╝ ╚═╝╚═══╝╚══╝╚╝╚╝╚══╝╚╝ ╚═══╝ ╚═╝╚══╝╚╝  ");
    println!("                                                                   ");
    println!("                                                                   ");
    println!("Usage: ./userlistgenerator -w <wordlist> [-u]");
    println!("Options:");
    println!("  -w, --wordlist <FILE>     Specify path to the wordlist");
    println!("  -u, --uppercase           Also produce uppercase permutations. Disabled by default");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help();
        return;
    }

    let mut wordlist = "";
    let mut uppercase = false;

    for i in 1..args.len() {
        match args[i].as_str() {
            "-w" | "--wordlist" => {
                if i + 1 < args.len() {
                    wordlist = &args[i + 1];
                } else {
                    println!("Error: Missing argument for wordlist");
                    return;
                }
            }
            "-u" | "--uppercase" => uppercase = true,
            _ => {}
        }
    }

    let names = generate_root_list_lowercase(wordlist);

    lowercase_transformations(&names);

    if uppercase {
        uppercase_transformations(&names);
    }
}
