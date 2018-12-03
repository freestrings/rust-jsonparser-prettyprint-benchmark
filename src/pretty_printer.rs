pub struct PrettyPrint {
    depth: usize,
    apply_indent: bool,
}

impl PrettyPrint {
    pub fn new() -> Self {
        PrettyPrint {
            depth: 0,
            apply_indent: true,
        }
    }

    pub fn get_indent(&self) -> usize {
        if self.apply_indent == true { self.depth * 4 } else { 0 }
    }

    pub fn inc_indent(&mut self) {
        self.depth += 1;
    }

    pub fn dec_indent(&mut self) {
        self.depth -= 1;
    }

    pub fn indent_on(&mut self) {
        self.apply_indent = true;
    }

    pub fn indent_off(&mut self) {
        self.apply_indent = false;
    }

    pub fn writeln(&self, msg: String) {
        println!("{space:>indent$}{value}", space = "", indent = self.get_indent(), value = msg);
    }

    pub fn write(&self, msg: String) {
        print!("{space:>indent$}{value}", space = "", indent = self.get_indent(), value = msg);
    }
}