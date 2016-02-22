
use cf::conf::{CF, GROUP_BLOKS_FLAG, get_block};
use cf::header::{Header, is_header, check_control_characters};
use cf::block_address::BlockAddress;

// Оглавление конфигурационного файла или группы блоков
pub struct TOC {
    addresses: Vec<BlockAddress>, // Адреса блоков
}

impl TOC {
    // Создать новое оглавление
    pub fn new() -> TOC {
        return TOC { addresses: Vec::new() };
    }

    // Получить объект оглавления, на основании данных конфигурационного файла
    pub fn from_cf(data: &Vec<u8>) -> Option<TOC> {

        trace!("Init table of contents");

        let reval = match find_toc(data) {
            None => None,
            Some(toc_header) => {
                return Some(TOC { addresses: read_toc(data, &toc_header) });
            }
        };

        trace!("-Init table of contents: {}.", reval.is_some());

        return reval;
    }

    // Возвращает адреса оглавления
    pub fn addresses(&self) -> &Vec<BlockAddress> {
        return &self.addresses;
    }

    // Добавить новый адрес
    pub fn add(&mut self, attr_header: &Header, data_header: &Header) {
        let attrs_header_pos = match self.addresses.last() {
            Some(v) => v.next_block_position(),
            None => 0,
        };

        let data_header_pos = attrs_header_pos + Header::size() + attr_header.full_region_size();
        let address = BlockAddress::new(attrs_header_pos,
                                        data_header_pos,
                                        data_header.full_region_size());

        self.addresses.push(address);
    }

    // Возвращает данные оглавления для конфигурационного файла
    pub fn for_cf(&self) -> Vec<u8> {

        trace!("Create table of content.");

        let valuable_toc_size = BlockAddress::size() * self.addresses.len();
        let mut data: Vec<u8> = Vec::new();
        data.reserve(valuable_toc_size);

        data.extend_from_slice(&CF::prefix()[..]);                  // префикс мультиблока
        data.extend_from_slice(&Header::for_cf(valuable_toc_size)); // Заголовок области оглавления

        let begin_file_size = data.len() + valuable_toc_size;

        for address in &self.addresses {
            data.extend_from_slice(&address.for_cf(begin_file_size)[..]);
        }

        trace!("-Create table of content.");

        return data;
    }
}

// Найти оглавление
fn find_toc(data: &Vec<u8>) -> Option<Header> {

    trace!("Find table of content.");

    let mut header: Option<Header> = None;

    if check_control_characters(&data, 0, 0, &GROUP_BLOKS_FLAG) {

        let header_pos = CF::prefix().len() as i32;

        if is_header(data, header_pos) {
            header = Some(Header::from_cf(&data, header_pos));
        }
    }

    trace!("-Find table of content: {}.", header.is_none());

    return header;
}

// Возвращает коллекцию адресов оглавления, прочитанных из конфигурационного файла
fn read_toc(data: &Vec<u8>, header_toc: &Header) -> Vec<BlockAddress> {

    trace!("Read table of content. Position={}",
           header_toc.region_position());

    let mut toc: Vec<BlockAddress> = Vec::new();
    let data_toc = get_block(data, header_toc);

    let value_size = BlockAddress::size();

    for i in 0..data_toc.len() / value_size {

        match BlockAddress::from_cf(&data_toc, i * value_size) {
            Some(val) => toc.push(val),
            None => break,
        }
    }

    trace!("-Read table of content. Count addresses={}", toc.len());

    return toc;
}

#[test]
fn test_find_toc() {

    let mut data_toc: Vec<u8> = Vec::new();
    data_toc.extend_from_slice(&[0x0A, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x00, 0xFF, 0xFF,
                                 0xFF, 0x7F]);  // 10-11
    data_toc.extend_from_slice(&[0x0C, 0x00, 0x00, 0x00, 0x0D, 0x00, 0x00, 0x00, 0xFF, 0xFF,
                                 0xFF, 0x7F]);  // 12-13

    let header_data = Header::for_cf(data_toc.len());

    let mut data = CF::prefix();
    data.extend(header_data);
    data.extend(data_toc);

    let toc = match find_toc(&data) {
        None => panic!("Failed test - find_toc"),
        Some(v) => v,
    };

    assert_eq!(24, toc.region_size());
}

#[test]
fn test_read_toc() {

    let block_data: Vec<u8> = vec![0x64, 0x61, 0x74, 0x61];
    let block_data_header = Header::for_cf(block_data.len());

    let mut conf_file = CF::prefix();
    let data_size_before_toc = conf_file.len() + Header::size() as usize + BlockAddress::size();

    let source_address = BlockAddress::new(1, 1, block_data.len() as i32);
    let source_address_cf = source_address.for_cf(data_size_before_toc);
    let header_toc = Header::for_cf(source_address_cf.len());

    conf_file.extend_from_slice(&header_toc[..]);
    conf_file.extend_from_slice(&source_address_cf[..]);
    conf_file.push(0x00);
    conf_file.extend_from_slice(&block_data_header[..]);
    conf_file.extend_from_slice(&block_data[..]);

    let test = match TOC::from_cf(&conf_file) {
        None => panic!("Failed create table of contents."),
        Some(v) => v,
    };

    assert_eq!(1, test.addresses().len());

    let test_address = test.addresses().get(0).unwrap();
    assert_eq!(source_address.attr_header_pos() + data_size_before_toc as i32,
               test_address.attr_header_pos());
    assert_eq!(source_address.data_header_pos() + data_size_before_toc as i32,
               test_address.data_header_pos());
}

#[test]
fn test_add() {

    let mut attrs: Vec<u8> = Vec::new();
    attrs.extend_from_slice(&[0xB0, 0x14, 0xC1, 0x30, 0x23, 0x42, 0x02, 0x00]);
    attrs.extend_from_slice(&[0xB0, 0x14, 0xC1, 0x30, 0x23, 0x42, 0x02, 0x00]);
    attrs.extend_from_slice(&[0xE4, 0x02, 0x00, 0x00]);
    attrs.extend_from_slice(&[0x72, 0x00, 0x6F, 0x00, 0x6F, 0x00, 0x74, 0x00]); // name
    attrs.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    let attr_header = Header::for_cf(attrs.len());

    let data: Vec<u8> = vec![0x64, 0x61, 0x74, 0x61];
    let data_header = Header::for_cf(data.len());

    let mut data_block: Vec<u8> = Vec::new();
    data_block.extend_from_slice(&attr_header);
    data_block.extend_from_slice(&attrs);
    data_block.extend_from_slice(&data_header);
    data_block.extend_from_slice(&data);

    let header_attr_in_block = Header::from_cf(&data_block, 0);
    let header_data_in_block = Header::from_cf(&data_block,
                                               (attr_header.len() + attrs.len()) as i32);

    let mut toc = TOC::new();
    toc.add(&header_attr_in_block, &header_data_in_block);
    toc.add(&header_attr_in_block, &header_data_in_block);

    for i in 0..toc.addresses().len() {
        let test = toc.addresses().get(i).unwrap();

        assert_eq!(i * data_block.len(), test.attr_header_pos() as usize);
        assert_eq!(i * data_block.len() + attr_header.len() + attrs.len(),
                   test.data_header_pos() as usize);
        assert_eq!(i * data_block.len() + data_block.len(),
                   test.next_block_position() as usize);
    }

    let cf = toc.for_cf();
    let test_toc = TOC::from_cf(&cf).unwrap();

    assert_eq!(toc.addresses().len(), test_toc.addresses().len());

    let begin_file_size = 71usize;
    for i in 0..test_toc.addresses.len() {
        assert_eq!(toc.addresses().get(i).unwrap().for_cf(begin_file_size),
                   test_toc.addresses().get(i).unwrap().for_cf(0));
    }
}
