use std::io;
use std::io::BufRead;
use std::io::Write;
use urlencoding::encode;
use clap::{Arg, App};


fn sbs(template: &str, subs: &str) -> String {
    template.replace("{}", subs)
}

fn encode_line(text: &str, template: &str) -> String {
    let s = sbs(template, text);
    encode(&s)
}

fn print_line(line: &str) {
    io::stdout().lock().write(format!("{}\n", line).as_bytes()).unwrap();
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
        let l = line.unwrap();
        let formatted = encode_line(&l, template);

        print_line(&formatted);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{sbs, encode_line};

    #[test]
    fn format_works_correctly() {
        assert_eq!(sbs("one {} three", "two"), "one two three");
        assert_eq!(sbs("one {{}} three", "two"), "one {two} three");
        assert_eq!(sbs("one {{}} {} three", "two"), "one {two} two three");
        assert_eq!(sbs("one{{}}three", "two"), "one{two}three");
    }

    #[test]
    fn encode_line_works_correctly() {
        assert_eq!(encode_line("empty", "Hello Günter"), "Hello%20G%C3%BCnter");
        assert_eq!(encode_line("не пусто", "Hello Günter/{}"), "Hello%20G%C3%BCnter%2F%D0%BD%D0%B5%20%D0%BF%D1%83%D1%81%D1%82%D0%BE");
    }
}