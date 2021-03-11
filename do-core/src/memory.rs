use crate::Error;

pub struct Memory {
    size: usize,
    memory: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Memory {
            size,
            memory: vec![0; size],
        }
    }

    pub fn load(&self, address: u16) -> Result<u8, Error> {
        if address > self.size as u16 {
            return Err(Error::MemoryOverflow(address));
        }

        Ok(self.memory[address as usize])
    }

    pub fn store(&mut self, address: u16, value: u8) -> Result<(), Error> {
        if address > self.size as u16 {
            return Err(Error::MemoryOverflow(address));
        }

        self.memory[address as usize] = value;

        Ok(())
    }
}
