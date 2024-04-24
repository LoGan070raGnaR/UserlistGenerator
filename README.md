# UserlistGenerator

UserlistGenerator is a Rust tool for generating lists of usernames to be used in brute-force attacks. It creates permutations of names from a wordlist file, providing lowercase and optional uppercase variations. This tool is especially useful for testing password strength with tools like [Kerbrute](https://github.com/ropnop/kerbrute).

## Features
- Generate usernames from a wordlist file
- Produce lowercase and optional uppercase permutations
- Ideal for testing password strength with tools like Kerbrute

## Usage
```bash
./userlistgenerator -w <wordlist> [-u]
```

### Options
- `-w, --wordlist <FILE>`: Specify the path to the wordlist file.
- `-u, --uppercase`: Also produce uppercase permutations. Disabled by default.

## Example
```bash
./userlist_generator -w example_wordlist.lst
```

## What is Kerbrute?
[Kerbrute](https://github.com/ropnop/kerbrute) is a tool used for performing Kerberos pre-authentication brute-force attacks. It's commonly used for testing password policies and assessing security risks.

## Credits
This project was inspired by the [username_generator](https://github.com/shroudri/username_generator) project written in Python.

