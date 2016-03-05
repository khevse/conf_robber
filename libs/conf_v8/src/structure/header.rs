
use conv;
use std::clone::Clone;

// Маркеры начала и окончания заголовка
const BEGIN_HEADER_MARKER: [u8; 2] = [b'\r', b'\n'];
const END_HEADER_MARKER: [u8; 2] = [b'\r', b'\n'];

const SPACE: u8 = 0x20;
const EMPTY_HEADER_VALUE: [u8; 8] = [b'7', b'f', b'f', b'f', b'f', b'f', b'f', b'f']; // Пустое значение = 2147483647 - i32::max_value()

#[test]
fn test_check_control_characters() {
    assert_eq!(true,
               check_control_characters(&vec![0u8, 1, 2, 3, 4, 5], 0, 0, &[0, 1, 2]));
    assert_eq!(true,
               check_control_characters(&vec![0u8, 1, 2, 3, 4, 5], 1, 0, &[1, 2, 3]));
    assert_eq!(true,
               check_control_characters(&vec![0u8, 1, 2, 3, 4, 5], 0, 1, &[1, 2, 3]));
    assert_eq!(false,
               check_control_characters(&vec![0u8, 1, 2, 3, 4, 5], 1, 0, &[2, 3, 4]));
}

// Проверить, что данные в указанной позиции соответствуют коллекции контрольных байт
//
// @param коллекция исходных данных
// @param позиция в исходных данных, начиная с которой будем выполнять проверку
// @param позиция от начала исходных данных, где ожидается наличие контрольных байт
// @param коллекция контрольных байт
//
// @result - true - проверка контрольных байт выполнена успешно
pub fn check_control_characters(data: &Vec<u8>,
                                start_pos_in_data: i32,
                                control_char_pos: i32,
                                control_characters: &[u8])
                                -> bool {

    let start_data_pos = start_pos_in_data + control_char_pos;
    let end_data_pos = start_data_pos + control_characters.len() as i32;

    if start_data_pos > data.len() as i32 || end_data_pos > data.len() as i32 {
        return false;
    } else {
        return &data[start_data_pos as usize..end_data_pos as usize] == control_characters;
    }
}

#[test]
fn test_is_header() {
    let mut data = vec![0u8];
    data.extend_from_slice(&BEGIN_HEADER_MARKER);
    data.extend_from_slice(&[b'0', b'0', b'0', b'0', b'0', b'0', b'a', b'3', SPACE]);
    data.extend_from_slice(&[b'0', b'0', b'0', b'0', b'0', b'2', b'0', b'0', SPACE]);
    data.extend_from_slice(&[b'7', b'f', b'f', b'f', b'f', b'f', b'f', b'f', SPACE]);
    data.extend_from_slice(&END_HEADER_MARKER);

    assert_eq!(false, is_header(&data, 0));
    assert_eq!(true, is_header(&data, 1));
}

// В текущей позиции начинаются данные заголовка
// Пример: "\r\n000000a3 00000200 7fffffff \r\n" (всегда 31 символ)
//
// @param - данные
// @param - текущая позиция, в которой ожидаем увидеть данные заголовка
pub fn is_header(data: &Vec<u8>, start_pos: i32) -> bool {

    return check_control_characters(&data, start_pos, 0, &BEGIN_HEADER_MARKER) &&
           check_control_characters(&data, start_pos, 10, &[SPACE]) &&
           check_control_characters(&data, start_pos, 19, &[SPACE]) &&
           check_control_characters(&data, start_pos, 28, &[SPACE]) &&
           check_control_characters(&data, start_pos, 29, &END_HEADER_MARKER);
}

// Заголовок области, содержащий координаты и размеры в конфигурационном файле 1С
//   - пример заголовка с маркерами:    "\r\n000000ac 00000200 7fffffff \r\n"
//   - пример заголовка без маркеров: "000000ac 00000200 00000200 "
//  , где:
//   - первое значение: "000000ac" - размер полезных данных области
//   - второе значение: "00000200" - полный размер данных области
//   - третье значение: "00000200" - позиция продолжения данных области. Если содержит значение "7fffffff" - продолжение отсутствует
pub struct Header {
    valuable_region_size: i32, // размер полезных данных области
    total_region_size: i32, // полный размер области
    next_header_position: i32, // позиция заголовка продолжения данных
    region_position: i32, // позиция с которой начинаются данные
}

impl Header {
    // Инициализивать объект на основании данных конфигурационного файла
    pub fn from_cf(data: &Vec<u8>, header_position: i32) -> Header {

        let begin_pos = header_position as usize + BEGIN_HEADER_MARKER.len();
        let end_pos = begin_pos + Header::value_size() as usize;
        if begin_pos > data.len() || end_pos > data.len() {
            error!("Failed create header object: data size={}; header position={}; begin \
                    position={}; end position={}.",
                   data.len(),
                   header_position,
                   begin_pos,
                   end_pos);
            panic!("Failed create header object: data size={}; header position={}; begin \
                    position={}; end position={}.",
                   data.len(),
                   header_position,
                   begin_pos,
                   end_pos);
        }

        // "000000ac 00000200 00000200 " => [172, 512, 512]
        let mut num_group = 0i8;
        let mut valuable_region_size = 0i32;
        let mut total_region_size = 0i32;
        let mut next_header_position = 0i32;

        for group in data[begin_pos..end_pos].to_vec().split(|byte| *byte == SPACE) {
            if group.is_empty() {
                continue;
            }

            num_group += 1;
            let mut value = conv::hex_to_int(group);
            if value == i32::max_value() {
                value = 0;
            }

            match num_group {
                1 => valuable_region_size = value,
                2 => total_region_size = value,
                3 => next_header_position = value,
                _ => {
                    error!("Bad format of the header.");
                    panic!("Bad format of the header.");
                }
            }
        }

        return Header {
            valuable_region_size: valuable_region_size,
            total_region_size: total_region_size,
            next_header_position: next_header_position,
            region_position: header_position + Header::size(),
        };
    }

    // Получить данные заголовка в виде пригодном для записи в конфигурационный файл
    pub fn for_cf(valuable_region_size: usize) -> Vec<u8> {

        let mut data: Vec<u8> = Vec::new();
        data.reserve(Header::size() as usize);

        data.extend_from_slice(&BEGIN_HEADER_MARKER);
        data.extend_from_slice(&conv::int32_to_hex_bytes(valuable_region_size as i32));
        data.push(SPACE);
        data.extend_from_slice(&conv::int32_to_hex_bytes(valuable_region_size as i32));
        data.push(SPACE);
        data.extend_from_slice(&EMPTY_HEADER_VALUE);
        data.push(SPACE);
        data.extend_from_slice(&END_HEADER_MARKER);

        return data;
    }

    // Возвращает позицию области в файле
    pub fn region_position(&self) -> i32 {
        return self.region_position;
    }

    // Возвращает размер полезных данных области
    pub fn region_size(&self) -> i32 {

        match self.valuable_region_size > self.total_region_size || self.valuable_region_size == 0 {
            true => self.total_region_size,
            false => self.valuable_region_size,
        }
    }

    // Возвращает полный размер области к которой относится заголовок
    pub fn full_region_size(&self) -> i32 {
        return self.total_region_size;
    }

    // Возвращает позицию продолжения данных
    pub fn next_header_position(&self) -> Option<i32> {
        return match self.next_header_position {
            0 => None,
            _ => Some(self.next_header_position as i32),
        };
    }

    // Заголовок принадлежит области с данными атрибутов блока
    pub fn is_region_of_attrs(&self, data: &Vec<u8>) -> bool {

        const MIN_BLOCK_SIZE: i32 = 21; // второй символ в имени блока
        if self.region_size() < MIN_BLOCK_SIZE {
            return false;
        }

        let start_pos = self.region_position();
        if data.len() - start_pos as usize <= MIN_BLOCK_SIZE as usize {
            return false;
        }

        return    check_control_characters(data, start_pos, 7,  &[0x00])  // последний символ в значении даты модификации
               && check_control_characters(data, start_pos, 15, &[0x00])  // последний символ в значении даты создания
               && check_control_characters(data, start_pos, 19, &[0x00])  // последний символ в значении типа блока
               && check_control_characters(data, start_pos, 21, &[0x00]); // второй символ в имени блока
    }

    // Полный размер заголовка вместе с маркерами
    pub fn size() -> i32 {
        return 31;
    }

    // Размер полезных данных заголовка
    pub fn value_size() -> i32 {
        return Header::size() - BEGIN_HEADER_MARKER.len() as i32 - END_HEADER_MARKER.len() as i32;
    }
}

impl Clone for Header {
    fn clone(&self) -> Header {
        return Header {
            valuable_region_size: self.valuable_region_size,
            total_region_size: self.total_region_size,
            next_header_position: self.next_header_position,
            region_position: self.region_position,
        };
    }
}

#[test]
fn test_add_header() {
    let mut data: Vec<u8> = vec![];
    data.extend_from_slice(&BEGIN_HEADER_MARKER);
    data.extend_from_slice(&[b'0', b'0', b'0', b'0', b'0', b'0', b'a', b'c', SPACE]);
    data.extend_from_slice(&[b'0', b'0', b'0', b'0', b'0', b'0', b'a', b'c', SPACE]);
    data.extend_from_slice(&[b'7', b'f', b'f', b'f', b'f', b'f', b'f', b'f', SPACE]);
    data.extend_from_slice(&END_HEADER_MARKER);

    assert_eq!(data, Header::for_cf(172));
}


#[test]
fn test_header_new() {
    let mut data: Vec<u8> = vec![0u8];
    data.extend(Header::for_cf(172));

    let test = Header::from_cf(&data, 1);
    assert_eq!(test.valuable_region_size, 172);
    assert_eq!(test.total_region_size, 172);
    assert_eq!(test.next_header_position, 0);
}

#[test]
fn test_region_position() {

    for i in [0usize, 1].into_iter() {
        let mut data: Vec<u8> = vec![0u8; *i];
        data.extend(Header::for_cf(172));

        let pos = *i as i32;
        let test = Header::from_cf(&data, pos);
        assert_eq!(test.region_position(), pos + Header::size());
    }
}

#[test]
fn test_get_region_size() {

    for source_data in [[172, 172], [0, 512], [1024, 512]].into_iter() {
        let val_size: usize = source_data[0] as usize;

        let data = Header::for_cf(val_size);
        let test = Header::from_cf(&data, 0);

        assert_eq!(test.region_size(), val_size as i32);
        assert_eq!(test.full_region_size(), val_size as i32);
    }
}

#[test]
fn test_get_next_header_position() {

    let next_header = 10i32;
    let mut data: Vec<u8> = Vec::new();

    data.extend_from_slice(&BEGIN_HEADER_MARKER);
    data.extend_from_slice(&conv::int32_to_hex_bytes(1));
    data.push(SPACE);
    data.extend_from_slice(&conv::int32_to_hex_bytes(2));
    data.push(SPACE);
    data.extend_from_slice(&conv::int32_to_hex_bytes(next_header));
    data.push(SPACE);
    data.extend_from_slice(&END_HEADER_MARKER);

    let test = Header::from_cf(&data, 0);

    assert_eq!(test.next_header_position(), Some(next_header));
}

#[test]
fn test_is_region_of_attrs() {

    let creation_date = [0xB0, 0x14, 0xC1, 0x30, 0x23, 0x42, 0x02, 0x00];
    let modification_date = [0xB0, 0x14, 0xC1, 0x30, 0x23, 0x42, 0x02, 0x00];
    let group_type = [0xE4, 0x02, 0x00, 0x00];
    let name = [0x72, 0x00, 0x6F, 0x00, 0x6F, 0x00, 0x74, 0x00];
    let unknown = [0x00, 0x00, 0x00, 0x00];

    let mut attr_data: Vec<u8> = Vec::new();

    attr_data.extend_from_slice(&creation_date);
    attr_data.extend_from_slice(&modification_date);
    attr_data.extend_from_slice(&group_type);
    attr_data.extend_from_slice(&name);
    attr_data.extend_from_slice(&unknown);

    let mut header_data: Vec<u8> = Vec::new();

    header_data.extend_from_slice(&BEGIN_HEADER_MARKER);
    header_data.extend_from_slice(&conv::int32_to_hex_bytes(attr_data.len() as i32));
    header_data.push(SPACE);
    header_data.extend_from_slice(&conv::int32_to_hex_bytes(512));
    header_data.push(SPACE);
    header_data.extend_from_slice(&EMPTY_HEADER_VALUE);
    header_data.push(SPACE);
    header_data.extend_from_slice(&END_HEADER_MARKER);

    let mut data: Vec<u8> = Vec::new();
    data.extend(header_data);
    data.extend(attr_data);

    let test = Header::from_cf(&data, 0);
    assert_eq!(true, test.is_region_of_attrs(&data));
}
