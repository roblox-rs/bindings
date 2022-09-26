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

    pub fn push(&mut self, str: &str) {
        self.write(str);
        self.indent_level += 1;
    }

    pub fn pop(&mut self, str: &str) {
        if self.indent_level == 0 {
            panic!("Failed to add");
        }
        self.indent_level -= 1;
        self.write(str);
    }

    pub fn write(&mut self, str: &str) {
        self.stream.push_str(&"\t".repeat(self.indent_level.into()));
        self.stream.push_str(str);
        self.stream.push('\n');
    }

    pub fn write_split(&mut self, str: &str) {
        for s in str.split('\n') {
            self.write(s);
        }
    }
}

macro_rules! note {
    ($name:ident) => {
        $name.write("")
    };
	($name:ident, $($tts:tt)*) => {
		$name.write_split(&format!($($tts)*))
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

pub(crate) use note;
pub(crate) use pull;
pub(crate) use push;
