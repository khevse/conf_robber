
use conv;
use structure::header::Header;
use std::i32::MAX;

// Адреса областей атрибутов и данных одного блока для оглавления конфигурационного файла
#[derive(Clone)]
pub struct BlockAddress {
    attrs_header_pos: i32, // Позиция заголовка атрибутов блока
    data_header_pos: i32, // Позиция заголовка данных блока
    data_size: i32, // Размер полезных данных блока
}

impl BlockAddress {
    // Получить адреса атрибутоы и данных блока, используется при создании нового конфигурационного файла
    pub fn new(_attrs_header_pos: i32, _data_header_pos: i32, _data_size: i32) -> BlockAddress {
        return BlockAddress {
            attrs_header_pos: _attrs_header_pos,
            data_header_pos: _data_header_pos,
            data_size: _data_size,
        };
    }

    // Инициализировать данные адреса на основании данных конфигурационного файла
    pub fn from_cf(data: &Vec<u8>, start_pos: usize) -> Option<BlockAddress> {

        let mut values: Vec<i32> = Vec::new();
        let mut end_region_pos: usize = 0;

        for i in 0..3 {
            let start_region_pos = start_pos + i * BlockAddress::element_size();
            end_region_pos = start_region_pos + BlockAddress::element_size();

            let val = conv::bytes_to_int32(&data[start_region_pos..end_region_pos]) as i32;

            if val == MAX {
                break;
            } else if val == 0 {

                if i == 0 {
                    return None;
                } else {
                    error!("Error creation a address object: start_pos={}; end_pos={}; data={:?}",
                           start_pos,
                           end_region_pos,
                           &data[start_pos..end_region_pos]);
                    panic!("Error creation a address object: start_pos={}; end_pos={}; data={:?}",
                           start_pos,
                           end_region_pos,
                           &data[start_pos..end_region_pos]);
                }

            }

            values.push(val);
        }

        return match values.len() {
            2 => {
                let data_header_pos = *values.get(1).unwrap();

                Some(BlockAddress {
                    attrs_header_pos: *values.get(0).unwrap(),
                    data_header_pos: data_header_pos,
                    data_size: -1,
                })
            }
            _ => {
                error!("Error creation a address object: start_pos={}; end_pos={}; data={:?}",
                       start_pos,
                       end_region_pos,
                       &data[start_pos..end_region_pos]);
                panic!("Error creation a address object: start_pos={}; end_pos={}; data={:?}",
                       start_pos,
                       end_region_pos,
                       &data[start_pos..end_region_pos]);
            }
        };
    }

    // Возвращает данные атрибутов для конфигурационного файла
    pub fn for_cf(&self, toc_len: usize) -> Vec<u8> {

        let mut data: Vec<u8> = Vec::new();
        data.extend_from_slice(&conv::int32_to_bytes(self.attrs_header_pos + toc_len as i32));
        data.extend_from_slice(&conv::int32_to_bytes(self.data_header_pos + toc_len as i32));
        data.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0x7F]); // &conv::int32_to_bytes(i32::max_value())

        return data;
    }

    // Возвращает позицию заголовка атрибутов блока
    pub fn attr_header_pos(&self) -> i32 {
        return self.attrs_header_pos;
    }

    // Возвращает позицию данных блока
    pub fn data_header_pos(&self) -> i32 {
        return self.data_header_pos;
    }

    // Возвращает количество байт одного из значений адреса таблицы оглавления:
    // - 4 байта (Позиция заголовка атрибутов)
    // - 4 байта (Позиция заголовка данных)
    // - 4 байта (Разделитель адресов - emptyValue)
    pub fn element_size() -> usize {
        return 4; // = conv.int32_to_bytes(BlockAddress.attrs_header_pos).len()
    }

    // Возвращает размер одного адреса в таблице оглавления в байтах
    pub fn size() -> usize {
        return 12; // Позиция заголовка атрибутов + Позиция заголовка данных + Разделитель адресов = element_size() * 3
    }

    // Возвращает позицию следующего блока
    pub fn next_block_position(&self) -> i32 {
        if self.data_size < 0 {
            error!("The size of the data is not initilized.");
            panic!("The size of the data is not initilized.");
        }

        return self.data_header_pos + Header::size() + self.data_size;
    }
}


#[test]
fn test_block_address_from_cf() {

    let toc: Vec<u8> = vec![0x0D, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x7F]; // 13-14
    let attrs: Vec<u8> = vec![0x00];
    let block_data: Vec<u8> = vec![0x68, 0x65, 0x6C, 0x6C, 0x6F]; // hello
    let block_data_header = Header::for_cf(block_data.len());


    let mut source_data: Vec<u8> = Vec::new();
    source_data.extend_from_slice(&toc);
    source_data.extend_from_slice(&attrs);
    source_data.extend_from_slice(&block_data_header);
    source_data.extend_from_slice(&block_data);

    let test = BlockAddress::from_cf(&Vec::from(&source_data[..]), 0).unwrap();

    assert_eq!(toc, test.for_cf(0));
    assert_eq!(13, test.attr_header_pos());
}

#[test]
fn test_block_address_for_cf() {

    let block_data_size = 1i32;
    let test = BlockAddress::new(13, 14, block_data_size);
    let toc: Vec<u8> = vec![0x0D, 0x00, 0x00, 0x00, 0x0E, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x7F]; // 13-14

    assert_eq!(toc, test.for_cf(0));
    assert_eq!(14 + Header::size() + block_data_size,
               test.next_block_position());
}
