use pest::error::Error;
use pest::Parser;
use pest_grammars::json::*;

use pest_json::JsonHandler;
use pest_json::JsonTokenWalker;
use pretty_printer;

pub fn pretty_print(json: &str) -> std::result::Result<(), Error<Rule>> {
    let json = JsonParser::parse(Rule::json, json)?.next()
        .expect(format!("Invalid json format {:?}", json).as_str());
    PrettyPrinterWalker::walk(json, &mut pretty_printer::PrettyPrint::new());
    Ok(())
}

struct PrettyPrinterWalker;

impl<E> JsonTokenWalker<E> for PrettyPrinterWalker where E: JsonHandler {}

impl JsonHandler for pretty_printer::PrettyPrint {
    fn json_start(&mut self) {}

    fn json_end(&mut self) {}

    fn object_start(&mut self) {
        self.writeln("{".to_string());
        self.inc_indent();
    }

    fn object_end(&mut self) {
        self.dec_indent();
        self.writeln("}".to_string());
    }

    fn object_key(&mut self, idx: usize, key: String) {
        self.indent_on();
        if idx > 0 {
            self.write(",".to_string());
            self.indent_off();
        }
        self.write(key);
        self.indent_off();
        self.write(" : ".to_string());
    }

    fn object_value_start(&mut self, _idx: usize, _key: String) {
        self.indent_off();
    }

    fn object_value_end(&mut self, _idx: usize, _key: String) {
        self.indent_on();
    }

    fn array_start(&mut self) {
        self.writeln("[".to_string());
        self.inc_indent();
    }

    fn array_end(&mut self) {
        self.dec_indent();
        self.writeln("]".to_string());
    }

    fn array_value_start(&mut self, idx: usize) {
        self.indent_on();
        if idx > 0 {
            self.write(",".to_string());
            self.indent_off();
        }
    }

    fn array_value_end(&mut self, _idx: usize) {
        self.indent_on();
    }

    fn string_value(&mut self, value: String) {
        let mut buf = String::new();
        let mut chars = value.chars();

        #[derive(PartialEq)]
        enum State {
            UnicodeCandidate,
            Unicode(char),
            Char(char),
            Str(String),
            None,
        };

        let mut state: State = State::None;

        loop {
            let reference = chars.by_ref();

            match reference.next() {
                Some(ch) => {
                    state = match ch {
                        '\\' => State::UnicodeCandidate,
                        'u' if state == State::UnicodeCandidate => {
                            let chunk: String = reference.take(4).collect();
                            if chunk.len() < 4 {
                                State::Str(chunk)
                            } else {
                                match u32::from_str_radix(&chunk[..], 16).ok().and_then(std::char::from_u32) {
                                    Some(ch) => State::Unicode(ch),
                                    None => State::Str(chunk)
                                }
                            }
                        }
                        '/' if state == State::UnicodeCandidate => {
                            State::Char(ch)
                        }
                        _ if state == State::UnicodeCandidate => {
                            State::Str(format!("\\{}", ch))
                        }
                        _ => State::Char(ch)
                    }
                }
                None => break
            }

            match &state {
                State::Char(ch) | State::Unicode(ch) => {
                    buf.push(*ch);
                }
                State::Str(ch) => {
                    buf.push_str(ch);
                }
                _ => {}
            }
        }
        buf.push('\n');
        self.write(buf);
    }

    fn number_value(&mut self, value: f64) {
        self.writeln(format!("{}", value));
    }

    fn bool_value(&mut self, value: bool) {
        self.writeln(format!("{}", value));
    }

    fn null_value(&mut self) {
        self.writeln("null".to_string());
    }
}