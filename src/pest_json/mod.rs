use pest::iterators::Pair;
use pest_grammars::json::*;

pub mod pretty;

trait JsonTokenWalker<E> where E: JsonHandler {
    fn walk_json(pair: Pair<Rule>, handler: &mut E) {
        handler.json_start();
        for p in pair.into_inner() {
            Self::walk(p, handler);
        }
        handler.json_end();
    }
    fn walk_object(pair: Pair<Rule>, handler: &mut E) {
        handler.object_start();
        let mut idx = 0_usize;
        for p in pair.into_inner() {
            let mut inner = p.into_inner();
            let key = match inner.next() {
                Some(key) => key.as_str().to_string(),
                None => String::new()
            };
            handler.object_key(idx, key.clone());
            match inner.next() {
                Some(p) => {
                    handler.object_value_start(idx, key.clone());
                    Self::walk(p, handler);
                    handler.object_value_end(idx, key);
                }
                None => panic!("Invalid json: {:?}", inner)
            };
            idx += 1;
        }
        handler.object_end();
    }
    fn walk_array(pair: Pair<Rule>, handler: &mut E) {
        handler.array_start();
        let mut idx = 0_usize;
        for p in pair.into_inner() {
            handler.array_value_start(idx);
            Self::walk(p, handler);
            handler.array_value_end(idx);
            idx += 1;
        }
        handler.array_end();
    }
    fn walk_value(pair: Pair<Rule>, handler: &mut E) {
        let mut inner = pair.into_inner();
        match inner.next() {
            Some(p) => Self::walk(p, handler),
            None => panic!("Invalid value: {:?}", inner),
        }
    }
    fn walk(pair: Pair<Rule>, handler: &mut E) {
        match pair.as_rule() {
            Rule::json => Self::walk_json(pair, handler),
            Rule::object => Self::walk_object(pair, handler),
            Rule::array => Self::walk_array(pair, handler),
            Rule::value => Self::walk_value(pair, handler),
            Rule::string => handler.string_value(pair.as_str().to_string()),
            Rule::number => handler.number_value(match pair.as_str().parse::<f64>() {
                Ok(key) => key,
                Err(_e) => panic!("Not a number: {:?}", pair)
            }),
            Rule::bool => handler.bool_value(match pair.as_str().parse::<bool>() {
                Ok(key) => key,
                Err(_e) => panic!("Not a boolean: {:?}", pair)
            }),
            Rule::null => handler.null_value(),
            Rule::EOI => handler.json_end(),
            _other => unreachable!()
        }
    }
}

trait JsonHandler: Sized {
    fn json_start(&mut self);
    fn json_end(&mut self);
    fn object_start(&mut self);
    fn object_end(&mut self);
    fn object_key(&mut self, idx: usize, key: String);
    fn object_value_start(&mut self, idx: usize, key: String);
    fn object_value_end(&mut self, idx: usize, key: String);
    fn array_start(&mut self);
    fn array_end(&mut self);
    fn array_value_start(&mut self, idx: usize);
    fn array_value_end(&mut self, idx: usize);
    fn string_value(&mut self, value: String);
    fn number_value(&mut self, value: f64);
    fn bool_value(&mut self, value: bool);
    fn null_value(&mut self);
}