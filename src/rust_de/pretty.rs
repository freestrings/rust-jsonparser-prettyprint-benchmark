use serde_json::de::StrRead;
use serde_json::Deserializer;
use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

use rust_de::JsonProcessor;
use rust_de::Visitor;

use pretty_printer;

pub fn pretty_print(json: &str) -> std::result::Result<(), serde_json::Error> {
    let deserializer: Deserializer<StrRead> = serde_json::Deserializer::from_str(json);
    PrettyPrinterProcessor::process(deserializer, pretty_printer::PrettyPrint::new())
}

struct PrettyPrinterProcessor;

impl<'a, V> JsonProcessor<'a, StrRead<'a>, V> for PrettyPrinterProcessor where V: Visitor {}

impl Visitor for pretty_printer::PrettyPrint {
    fn visit_null(&mut self) {
        self.writeln("null".to_string());
    }

    fn visit_bool(&mut self, value: &bool) {
        self.writeln(format!("{}", value));
    }

    fn visit_number(&mut self, value: &Number) {
        self.writeln(format!("{}", value));
    }

    fn visit_string(&mut self, value: &String) {
        self.writeln(format!("\"{}\"", value));
    }

    fn visit_array(&mut self, value: &Vec<Value>) {
        self.writeln("[".to_string());
        self.inc_indent();
        let mut idx = 0_usize;
        for v in value {
            self.indent_on();
            if idx > 0 {
                self.write(",".to_string());
                self.indent_off();
            }
            self.visit_value(&v);
            self.indent_on();
            idx += 1;
        }
        self.dec_indent();
        self.writeln("]".to_string());
    }

    fn visit_object(&mut self, value: &Map<String, Value>) {
        self.writeln("{".to_string());
        self.inc_indent();
        let mut idx = 0_usize;
        for k in value.keys() {
            self.indent_on();
            if idx > 0 {
                self.write(",".to_string());
                self.indent_off();
            }
            self.write(format!("\"{}\" : ", k));
            match value.get(k) {
                Some(v) => {
                    self.indent_off();
                    self.visit_value(&v);
                    self.indent_on();
                }
                None => self.visit_null(),
            }
            idx += 1;
        }
        self.dec_indent();
        self.writeln("}".to_string());
    }
}