use std::fs;

fn basic_main(program_name: &str) -> String {
    String::from(format!(
        "fn main() {{
    println!(\"TODO: implement {program_name}\");
}}",
    ))
}

fn cargo_insert(program_name: &str) -> std::io::Result<()> {
    let mut cargo_toml = fs::read("../Cargo.toml")?;

    let mut contents = format!(
        "\n[[bin]]
name=\"{program_name}\"
path=\"src/bin/{program_name}/main.rs\"\n"
    )
    .as_bytes()
    .to_vec();

    cargo_toml.append(&mut contents);

    fs::write("../Cargo.toml", cargo_toml)?;
    Ok(())
}

fn main() {
    let commands = "arch b2sum base32 base64 basename basenc cat chcon chgrp chmod chown chroot cksum comm coreutils cp csplit cut date dd df dir dircolors dirname du echo env expand expr factor false fmt fold groups head hostid hostname id install join kill link ln logname ls md5sum mkdir mkfifo mknod mktemp mv nice nl nohup nproc numfmt od paste pathchk pinky pr printenv printf ptx pwd readlink realpath rm rmdir runcon seq sha1sum sha224sum sha256sum sha384sum sha512sum shred shuf sleep sort split stat stdbuf stty sum sync tac tail tee test timeout touch tr true truncate tsort tty uname unexpand uniq unlink uptime users vdir wc who whoami yes";
    let commands = commands.split(" ");

    for command in commands {
        let res = fs::create_dir(format!("./bin/{command}"));
        if res.is_err() {
            println!("error: Could not create dir '{command}': {:?}", res.err());
            continue;
        }

        let res = fs::write(format!("./bin/{}/main.rs", command), basic_main(command));
        if res.is_err() {
            println!(
                "error: Could not create main file '{command}/main.rs': {:?}",
                res.err()
            );
        }

        let res = cargo_insert(command);
        if res.is_err() {
            println!(
                "error: Could not append Cargo.toml for '{command}': {:?}",
                res.err()
            );
        }
    }
}
