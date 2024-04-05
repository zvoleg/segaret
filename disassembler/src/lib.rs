use std::io::Write;
use std::io::Error;
use std::fs::File;
use std::collections::HashMap;

pub struct Disassembler {
    file_name: String,
    buffer: HashMap<u32, String>,
}

impl Disassembler {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: String::from(file_name),
            buffer: HashMap::new(),
        }
    }

    pub fn push_instruction(&mut self, address: u32, instruction: String) {
        self.buffer.insert(address, instruction);
    }

    pub fn save(&self) -> Result<(), Error>  {
        let mut file = File::create(&self.file_name)?;
        let mut disassembling = self.buffer.iter().map(|i| (*i.0, i.1.clone())).collect::<Vec<(u32, String)>>();
        disassembling.sort_by(|a, b| a.0.cmp(&b.0));
        let disassembling = disassembling.iter().map(|v| format!("{:08X}: {}", v.0, v.1)).collect::<Vec<String>>();
        let disassembling_str = disassembling.join("\n");
        file.write_all(disassembling_str.as_bytes())?;
        Ok(())
    }
}