# UserlistGenerator

UserlistGenerator is a Rust tool for generating lists of usernames to be used in brute-force attacks. It creates permutations of names from a wordlist file, providing lowercase and optional uppercase variations. This tool is especially useful in conjunction with [Kerbrute](https://github.com/ropnop/kerbrute).

## Compiling the Rust Script
To compile the Rust script, run the following command:
```bash
rustc userlistgenerator.rs
```

## Using the Compiled Executable
After compiling the Rust script, you can use the generated executable as follows:
```bash
./userlistgenerator -w <wordlist> [-u]
```

### Options
- `-w, --wordlist <FILE>`: Specify the path to the wordlist file.
- `-u, --uppercase`: Also produce uppercase permutations. Disabled by default.

### Example wordlist

```markdown
James Howlett
Ip Man
```

### Example output with lowercase permutations only

```bash
./userlistgenerator -w username.lst
```

```markdown
james
howlett
j.howlett
j-howlett
j_howlett
j+howlett
jhowlett
jameshowlett
howlettjames
james.howlett
howlett.james
ip
man
i.man
i-man
i_man
i+man
iman
ipman
manip
ip.man
man.ip
```

### Example output with lowercase and uppercase permutations

```bash
./userlistgenerator -w username.lst -u
```

```markdown
james
howlett
j.howlett
j-howlett
j_howlett
j+howlett
jhowlett
jameshowlett
howlettjames
james.howlett
howlett.james
ip
man
i.man
i-man
i_man
i+man
iman
ipman
manip
ip.man
man.ip
JAMES
HOWLETT
j.HOWLETT
j_HOWLETT
j-HOWLETT
jHOWLETT
JAMESHOWLETT
HOWLETTJAMES
JAMES
HOWLETT
JAMESHOWLETT
IP
MAN
i.MAN
i_MAN
i-MAN
iMAN
IPMAN
MANIP
IP
MAN
IPMAN
```

## What is Kerbrute?
[Kerbrute](https://github.com/ropnop/kerbrute) is a tool used for performing Kerberos pre-authentication brute-force attacks. It's commonly used for testing password policies and assessing security risks.

## Credits
This project was inspired by the [username_generator](https://github.com/shroudri/username_generator) project written in Python.