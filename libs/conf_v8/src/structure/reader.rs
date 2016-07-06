
use file_system;

use structure::toc::TOC;
use structure::block::Block;
use structure::header::Header;

/// Возвращает область данных по данным заголовка
pub fn get_region<'a>(data: &'a Vec<u8>, h: &Header) -> &'a [u8] {

    debug!("Region");

    let start_pos = h.region_position() as usize;
    let end_pos = (h.region_position() + h.region_size()) as usize;

    let buffer = &data[start_pos..end_pos];

    debug!("-Region");

    return buffer;
}

/// Получить все данные блока по данным заголовка
pub fn get_block(data: &Vec<u8>, h: &Header) -> Vec<u8> {

    debug!("Reading regions of the block.");

    let mut block: Vec<u8> = Vec::new();
    let mut header = h.clone();

    loop {
        block.extend_from_slice(&get_region(data, &header));

        match header.next_header_position() {
            Some(pos) => header = Header::from_cf(data, pos),
            None => break,
        }
    }

    debug!("-Reading regions of the block.");

    return block;
}

/// Получить коллекцию блоков на основании данных конфигурационного файла
pub fn from_cf(data: &Vec<u8>) -> Vec<Block> {

    info!("Read configuration file");

    let toc = match TOC::from_cf(&data) {
        None => {
            error!("Bad file format. Required format: *.cf.");
            panic!("Bad file format. Required format: *.cf.");
        }
        Some(v) => v,
    };

    let mut retval: Vec<Block> = Vec::new();

    for address in toc.addresses() {

        let header_attr = Header::from_cf(&data, address.attr_header_pos());
        let header_data = Header::from_cf(&data, address.data_header_pos());

        let block = Block::from_cf(&data, &header_attr, &header_data);
        retval.push(block);
    }

    info!("-Read configuration file");

    return retval;
}

/// Получить коллекцию блоков на основании ранее распакованных данных конфигурационного файла
pub fn from_file(path_to_dir: &String) -> Vec<Block> {

    info!("Read files");

    let mut retval: Vec<Block> = Vec::new();

    for (_, path) in &file_system::files_in_dir(path_to_dir) {
        retval.push(Block::from_file(&path));
    }

    info!("-Read files");

    return retval;
}
