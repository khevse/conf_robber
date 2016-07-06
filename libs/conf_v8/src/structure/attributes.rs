
use conv;
use std::clone::Clone;


// Типы блоков (возможно есть другие, более плотно не анализировал)
pub const GROUP_TYPE_MODULE: i32 = 740;   // Модуль ( Последовательный групповой блок: атрибуты, данные, атрибуты, данные, ...)
pub const GROUP_TYPE_FORM: i32 = 689;   // Форма ( Зеркальный групповой блок: атрибуты1, атрибуты2, данные2, данные1 )
pub const GROUP_TYPE_SIMPLY: i32 = 0;     // Простой
// TODO (сейчас не требуются)
//pub const GROUP_TYPE_CONFIG: i32 = 686;   // Заголовок конфигурационного файла
//pub const GROUP_TYPE_NO_MODULE: i32 = 84846; // Без модуля (Последовательный групповой блок: атрибуты, данные, атрибуты, данные, ...)

// Атрибуты блока
pub struct Attributes {
    creation_date: u64, // дата создания
    modification_date: u64, // дата модификации
    group_type: i32, // тип блока
    id: String, // идентификатор блока
}


impl Attributes {
    // Получить атрибуты для нового блока, используется при создании нового конфигурационного файла
    pub fn new(_creation_date: u64, _group_type: i32, _id: &String) -> Attributes {

        Attributes {
            creation_date: _creation_date,
            modification_date: _creation_date,
            group_type: _group_type,
            id: _id.clone(),
        }
    }

    // Получить атрибуты блока на основании данных конфигурационного файла
    pub fn from_cf(data: &[u8]) -> Attributes {
        let end_id = data.len() - 4;
        let id = match String::from_utf8(conv::utf16_to_utf8(&data[20..end_id])) {
            Ok(v) => v,
            Err(e) => {
                error!("Failed to get the id of the block: {}", e);
                panic!("Failed to get the id of the block: {}", e);
            }
        };

        Attributes {
            creation_date: conv::bytes_to_int64(&data[0..8]),
            modification_date: conv::bytes_to_int64(&data[8..16]),
            group_type: conv::bytes_to_int32(&data[16..20]),
            id: id,
        }
    }

    // Возвращает данные атрибутов для конфигурационного файла
    pub fn for_cf(&self) -> Vec<u8> {

        let unknow_block: [u8; 4] = match self.id.len() {
            0 => [0x7f, 0xff, 0xff, 0xff], // Пустое значение = 2147483647 - i32::max_value()
            _ => [0x00, 0x00, 0x00, 0x00],
        };

        let mut data: Vec<u8> = vec![];
        data.extend_from_slice(&conv::int64_to_bytes(self.creation_date));
        data.extend_from_slice(&conv::int64_to_bytes(self.modification_date));
        data.extend_from_slice(&conv::int32_to_bytes(self.group_type));
        data.extend_from_slice(&conv::utf8_to_utf16(&self.id.as_bytes()[..]));
        data.extend_from_slice(&unknow_block);

        data
    }

    // Возвращает наименование блока
    pub fn id<'a>(&'a self) -> &'a String {
        &self.id
    }
}

impl Clone for Attributes {
    fn clone(&self) -> Attributes {
        Attributes {
            creation_date: self.creation_date,
            modification_date: self.modification_date,
            group_type: self.group_type,
            id: self.id.clone(),
        }
    }
}

#[test]
fn test_attributes() {
    let creation_date = [0xB0, 0x14, 0xC1, 0x30, 0x23, 0x42, 0x02, 0x00];
    let modification_date = [0xB0, 0x14, 0xC1, 0x30, 0x23, 0x42, 0x02, 0x00];
    let group_type = [0xE4, 0x02, 0x00, 0x00];
    let id = [0x72, 0x00, 0x6F, 0x00, 0x6F, 0x00, 0x74, 0x00];
    let unknown = [0x00, 0x00, 0x00, 0x00];

    let mut data: Vec<u8> = vec![];
    data.extend_from_slice(&creation_date);
    data.extend_from_slice(&modification_date);
    data.extend_from_slice(&group_type);
    data.extend_from_slice(&id);
    data.extend_from_slice(&unknown);

    let attrs = Attributes::from_cf(&data[..]);
    let test = attrs.for_cf();

    assert_eq!(data, test);
}
