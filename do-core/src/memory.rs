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

#[cfg(test)]
mod tests {
    use crate::memory::Memory;
    use crate::Error;

    #[test]
    fn test_memory_store_load() -> Result<(), Error> {
        let mut memory = Memory::new(4096);

        memory.store(0x100, 0xf)?;
        assert_eq!(memory.load(0x100)?, 0xf);

        Ok(())
    }

    #[test]
    fn test_memory_overflow() -> Result<(), Error> {
        let mut memory = Memory::new(4096);

        assert!(memory.store(0x2000, 0xf).is_err());

        Ok(())
    }
}
