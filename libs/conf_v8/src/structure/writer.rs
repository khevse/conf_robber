


use structure::toc::TOC;
use structure::block::Block;
use structure::header::Header;

/// Получить данные для конфигурационного файла
pub fn inflate_cf(blocks: &Vec<Block>) -> Vec<u8> {

    info!("Inflate to the configuration file.");

    let mut toc = TOC::new();
    let mut cf_data: Vec<u8> = Vec::new();

    for block in blocks {
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

    info!("-Inflate to the configuration file.");

    return cf;
}

// Распаковать блоки и записать их файлы
pub fn deflate_to_files(blocks: &Vec<Block>, path_to_dir: &String) {

    info!("Deflate to files.");

    for block in blocks {
        trace!("Write block to the file: {}", block.id());
        block.write_to_file(path_to_dir);
    }

    info!("-Deflate to files.");
}
