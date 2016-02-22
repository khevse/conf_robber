
use utils;
use cf::header::Header;
use cf::block::Block;
use cf::toc::TOC;

pub static GROUP_BLOKS_FLAG: [u8; 4] = [0xFF, 0xFF, 0xFF, 0x7F]; // маркер группы &conv::int32_to_bytes(i32::max_value())
pub static DEFAULT_BLOCK_SIZE: i32 = 512; // Размер блока данных по умолчанию

// Возвращает область данных по данным заголовка
pub fn get_region<'a>(data: &'a Vec<u8>, h: &Header) -> &'a [u8] {
    let start_pos = h.region_position() as usize;
    let end_pos = (h.region_position() + h.region_size()) as usize;

    return &data[start_pos..end_pos];
}

// Получить все данные блока по данным заголовка
pub fn get_block(data: &Vec<u8>, h: &Header) -> Vec<u8> {

    let mut block: Vec<u8> = Vec::new();
    let mut header = h.clone();

    loop {
        block.extend_from_slice(&get_region(data, &header));

        match header.next_header_position() {
            Some(pos) => header = Header::from_cf(data, pos),
            None => break,
        }
    }

    return block;
}

// Конфигурация
pub struct CF {
    blocks: Vec<Block>, // блоки конфигурации
}

impl CF {
    // Получить объект на основании данных конфигурационного файла
    pub fn from_cf(data: &Vec<u8>) -> CF {

        info!("Read configuration file");

        let toc = match TOC::from_cf(&data) {
            None => {
                error!("Bad file format. Required format: *.cf.");
                panic!("Bad file format. Required format: *.cf.");
            }
            Some(v) => v,
        };

        let mut cf = CF { blocks: Vec::new() };

        for address in toc.addresses() {

            let header_attr = Header::from_cf(&data, address.attr_header_pos());
            let header_data = Header::from_cf(&data, address.data_header_pos());

            let block = Block::from_cf(&data, &header_attr, &header_data);
            cf.add_block(block);
        }

        info!("-Read configuration file");

        return cf;
    }

    // Получить объект на основании ранее распакованных данных конфигурационного файла
    pub fn from_file(path_to_dir: &String) -> CF {

        info!("Read files");

        let mut cf = CF { blocks: Vec::new() };
        let list = utils::fs::files_in_dir(path_to_dir);
        for path in &list[..] {
            cf.add_block(Block::from_file(&path));
        }

        info!("-Read files");

        return cf;
    }

    // Получить данные для конфигурационного файла
    pub fn for_cf(&self) -> Vec<u8> {

        let mut toc = TOC::new();
        let mut cf_data: Vec<u8> = Vec::new();

        for block in &self.blocks {
            let (attrs, data) = block.for_cf();

            let mut block_for_cf: Vec<u8> = Vec::new();
            block_for_cf.extend_from_slice(&Header::for_cf(attrs.len()));
            block_for_cf.extend_from_slice(&attrs);

            let data_header_pos = block_for_cf.len() as i32;

            block_for_cf.extend_from_slice(&Header::for_cf(data.len()));
            block_for_cf.extend_from_slice(&data);

            let header_attr = Header::from_cf(&block_for_cf, 0);
            let header_data = Header::from_cf(&block_for_cf, data_header_pos);

            toc.add(&header_attr, &header_data);
            cf_data.append(&mut block_for_cf);
        }

        let mut cf = toc.for_cf();
        cf.append(&mut cf_data);

        return cf;
    }

    // Распаковать блоки и записать их файлы
    pub fn deflate(&mut self, path_to_dir: &String) {

        info!("Write to files.");

        for block in &mut self.blocks {
            trace!("Write block to the file: {}", block.name());
            block.write_to_file(path_to_dir);
        }

        info!("-Write to files.");
    }

    // Добавить новый блок в конфигурацию
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    // Получить префикс конфигурационного файла
    pub fn prefix() -> Vec<u8> {

        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(&GROUP_BLOKS_FLAG);                                // маркер группы
        data.extend_from_slice(&utils::conv::int32_to_bytes(DEFAULT_BLOCK_SIZE)); // размер блока по умолчанию
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);                        // unknown
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);                        // unknown

        return data;
    }
}
