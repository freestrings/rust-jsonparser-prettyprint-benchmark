pub mod pretty;

use serde_json::Deserializer;
use serde_json::Value;
use serde_json::Map;

trait JsonProcessor<'a, R, V>
    where
        R: serde_json::de::Read<'a>,
        V: Visitor
{
    fn process(deserializer: Deserializer<R>, mut visitor: V) -> std::result::Result<(), serde_json::Error> {
        let mut iter = deserializer.into_iter::<Value>();
        loop {
            let value = iter.next();
            if value.is_none() {
                break;
            }
            match value.unwrap() {
                Ok(v) => visitor.visit_value(&v),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

trait Visitor {
    fn visit_value(&mut self, value: &Value) {
        match value {
            Value::Null => self.visit_null(),
            Value::Bool(v) => self.visit_bool(v),
            Value::Number(v) => self.visit_number(v),
            Value::String(v) => self.visit_string(v),
            Value::Array(v) => self.visit_array(v),
            Value::Object(v) => self.visit_object(v),
        }
    }
    fn visit_null(&mut self);
    fn visit_bool(&mut self, value: &bool);
    fn visit_number(&mut self, value: &serde_json::Number);
    fn visit_string(&mut self, value: &String);
    fn visit_array(&mut self, value: &Vec<Value>);
    fn visit_object(&mut self, value: &Map<String, Value>);
}