use std::io;
use std::io::BufRead;
use std::io::Write;
use urlencoding::encode;
use clap::{Arg, App};


fn format(template: &str, subs: &str) -> String {
    template.replace("{}", subs)
}

fn main() -> io::Result<()> {
    let matches = App::new("urlencode")
                          .version("0.1.0")
                          .author("Vadim Polulyakh <bavadim@gmail.com>")
                          .about("perform url encoding to strings in stdin")
                          .arg(Arg::with_name("template")
                               .short("t")
                               .long("template")
                               .value_name("STRING")
                               .help("write this string to stdout for each line in stdin and substitute url encoded string from stdin instead {}"))
                          .get_matches();

    let template = matches.value_of("template").unwrap_or("{}");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let s = encode(&line.unwrap());
        let formatted = format(template, &s) + "\n";

        io::stdout().lock().write(formatted.as_bytes())?;
    }

    Ok(())
}
