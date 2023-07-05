use std::fs;

#[derive(Debug)]
pub struct StorageItem {
    /// Size of the reserved space.
    /// In the file format, the first byte tells if the storage is
    /// reserved or not so the max storage is actually 15^2
    pub size: u16,
    /// Offset to the start of the storage space
    pub offset: u16,
    /// Possibly initialised data bytes.
    /// If data is initialised, the lenght needs to match `size`
    pub init_data: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct Storage {
    /// Length of the storage bytes, defined by first 2 bytes
    pub total_size: u16,
    /// Storage items
    pub items: Vec<StorageItem>,
}

impl Storage {
    fn load(data: &[u8]) -> Self {
        let total_size = u16::from_le_bytes([data[0], data[1]]);
        let mut bytes = &data[2..total_size as usize + 2];

        let mut items: Vec<StorageItem> = Vec::new();
        while !bytes.is_empty() {
            let size = u16::from_le_bytes([bytes[0], bytes[1]]);
            // We get the real size by removing the init_data flag
            let rsize = size & 0x7fff;
            // Add the variable stack address offset
            let offset = u16::from_le_bytes([bytes[2], bytes[3]]) + (u16::MAX / 2);
            // If the init_data flag is set
            let init_data = if size & 0x8000 == 0x8000 {
                Some(bytes[4..rsize as usize + 4].into())
            } else {
                None
            };

            if 4 + rsize as usize > bytes.len() {
                break;
            }

            // If the init_data flag is set
            bytes = if size & 0x8000 == 0x8000 {
                &bytes[4 + rsize as usize..]
            } else {
                &bytes[4..]
            };

            items.push(StorageItem {
                offset,
                init_data,
                size: rsize,
            })
        }

        Self { items, total_size }
    }
}

#[derive(Debug)]
pub struct SmolFile {
    pub storage: Storage,
    /// Instruction start address
    pub main_start: u16,
    pub instructions: Vec<u8>,
}

impl SmolFile {
    pub fn save(self, path: &str) {
        let mut storage_bytes: Vec<u8> = Vec::new();

        // Storage
        storage_bytes.extend(self.storage.total_size.to_le_bytes().iter());
        for item in self.storage.items {
            storage_bytes.extend(item.size.to_le_bytes().iter());
            storage_bytes.extend(item.offset.to_le_bytes().iter());
            if let Some(data) = item.init_data {
                storage_bytes.extend(data.iter());
            }
        }

        // start addess
        storage_bytes.extend(self.main_start.to_le_bytes().iter());

        // instructions
        storage_bytes.extend(self.instructions.iter());

        fs::write(path, storage_bytes).unwrap();
    }

    pub fn load(path: &str) -> Self {
        let file_bytes = fs::read(path).unwrap();
        let storage_size = u16::from_le_bytes([file_bytes[0], file_bytes[1]]) as usize;
        let storage = Storage::load(&file_bytes);
        let main_start = u16::from_le_bytes([file_bytes[storage_size + 2], file_bytes[storage_size + 3]]);
        let instructions: Vec<u8> = file_bytes[storage_size + 4..].into();

        Self {
            storage,
            main_start,
            instructions,
        }
    }
}
