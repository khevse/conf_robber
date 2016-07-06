
use conv;
use meta_data;
use structure;
use structure::block::Block;
use settings::Settings;

use {GROUP_BLOKS_FLAG, DEFAULT_BLOCK_SIZE};

/// Конфигурация
pub struct CF {
    blocks: Vec<Block>, // блоки конфигурации
}

impl CF {
    pub fn new(blocks: Vec<Block>) -> CF {
        return CF { blocks: blocks };
    }

    /// Добавить новый блок в конфигурацию
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    /// Получить префикс конфигурационного файла
    pub fn prefix() -> Vec<u8> {

        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(&GROUP_BLOKS_FLAG);                         // маркер группы
        data.extend_from_slice(&conv::int32_to_bytes(DEFAULT_BLOCK_SIZE)); // размер блока по умолчанию
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);                 // unknown
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);                 // unknown

        return data;
    }

    /// Получить объект на основании данных конфигурационного файла
    pub fn from_cf(data: &Vec<u8>) -> CF {
        return CF::new(structure::reader::from_cf(data));
    }

    /// Получить объект на основании ранее распакованных данных конфигурационного файла
    pub fn from_file(path_to_dir: &String) -> CF {
        return CF::new(structure::reader::from_file(path_to_dir));
    }

    /// Получить данные для конфигурационного файла
    pub fn for_cf(&self) -> Vec<u8> {
        return structure::writer::inflate_cf(&self.blocks);
    }

    /// Распаковать блоки и записать их файлы
    pub fn deflate_to_files(&self, path_to_dir: &String) {
        structure::writer::deflate_to_files(&self.blocks, path_to_dir);
    }

    pub fn filter(&mut self, settings_xml: &String) {
        let settings = Settings::new(&settings_xml);
        meta_data::writer::filter(&mut self.blocks, &settings);
    }
}
