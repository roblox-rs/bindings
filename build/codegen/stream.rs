pub struct Stream {
    pub stream: String,
    indent_level: u8,
}

impl Stream {
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            stream: String::new(),
        }
    }

    pub fn prereq(prereqs: &mut Vec<String>, cb: impl FnOnce(&mut Stream)) {
        let mut stream = Stream::new();
        cb(&mut stream);
        prereqs.push(stream.stream);
    }

    pub fn expression(cb: impl FnOnce(&mut Stream)) -> String {
        let mut stream = Stream::new();
        cb(&mut stream);
        stream.stream
    }

    pub fn push(&mut self, str: &str) {
        self.write_line(str);
        self.indent_level += 1;
    }

    pub fn pop(&mut self, str: &str) {
        if self.indent_level == 0 {
            panic!("Failed to add");
        }
        self.indent_level -= 1;
        self.write_line(str);
    }

    pub fn write(&mut self, str: &str) {
        for (i, text) in str.split('\n').enumerate() {
            if i > 0 {
                self.stream.push('\n');
                self.stream.push_str(&"\t".repeat(self.indent_level.into()));
            }

            self.stream.push_str(text);
        }
    }

    pub fn write_line(&mut self, str: &str) {
        for text in str.split('\n') {
            if !self.stream.is_empty() {
                self.stream.push('\n');
            }

            self.stream.push_str(&"\t".repeat(self.indent_level.into()));
            self.stream.push_str(text);
        }
    }
}

macro_rules! note {
    ($name:ident in $list:ident) => {
        if !$list.is_empty() {
            note!($name, "{}", $list.join("\n"));
        }
    };
    ($name:ident) => {
        $name.write_line("")
    };
	($name:ident, $($tts:tt)*) => {
		$name.write_line(&format!($($tts)*))
	}
}

macro_rules! push {
	($name:ident, $($tts:tt)*) => {
		$name.push(&format!($($tts)*))
	}
}

macro_rules! pull {
	($name:ident, $($tts:tt)*) => {
		$name.pop(&format!($($tts)*))
	}
}

macro_rules! write_to {
    ($name:ident, $($tts:tt)*) => {
        $name.write(&format!($($tts)*))
    }
}

pub(crate) use note;
pub(crate) use pull;
pub(crate) use push;
pub(crate) use write_to;
