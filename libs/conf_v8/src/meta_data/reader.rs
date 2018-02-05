use regex;
use structure::block::Block;
use meta_data::substr::{Substr, find_text};

macro_rules! try_regex {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!(r"Failed regex string: {}", e)
        }
    }
}

lazy_static! {
// {fdf816d2-1ead-11d5-b975-0050bae0a95d,1,2605a1e0-a034-4fd1-885b-7a2fdf618144}
// Результат: vec![ "fdf816d2-1ead-11d5-b975-0050bae0a95d" , "2605a1e0-a034-4fd1-885b-7a2fdf618144" ]
    static ref RE_ALL_IDS: regex::Regex = try_regex!(regex::Regex::new(r"(?P<id>[a-zA-Z0-9]{8}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{12})"));

// {fdf816d2-1ead-11d5-b975-0050bae0a95d,2,2605a1e0-a034-4fd1-885b-7a2fdf618144,3305a1e0-a034-4fd1-885b-7a2fdf618144}
// Результат: vec![ "2605a1e0-a034-4fd1-885b-7a2fdf618144" , "3305a1e0-a034-4fd1-885b-7a2fdf618144" ]
    static ref RE_ELEMENTS_OF_TYPE: regex::Regex = try_regex!(regex::Regex::new(r",(?P<id>[a-zA-Z0-9]{8}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{12})"));

// {fdf816d2-1ead-11d5-b975-0050bae0a95d,1,{.... {0,0,b8c42329-e3ee-47c0-9d9e-0fae37b7eefa},"Команда1",}}
// Результат: vec![ "b8c42329-e3ee-47c0-9d9e-0fae37b7eefa" ]
    static ref RE_ID_AND_NAME: regex::Regex =try_regex!(regex::Regex::new(r"\{\d+,\d+,(?P<id>[a-zA-Z0-9]{8}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{4}-[a-zA-Z0-9]{12})\},\W(?P<name>\w+)\W"));
}

pub enum RegexTypes {
    All,
    ElementsOfType,
    IdAndName,
}

/// Выполняет поиск блока по имени (GUID)
pub fn block_by_name<'a>(blocks: &'a Vec<Block>, id: &String) -> Option<&'a Block> {

    for block in blocks {

        if block.id().eq(id) {
            return Some(block);
        }
    }

    return None;
}

/// Выполняет поиск идентификатора блока, в котором описана структура конфигурации
pub fn main_conf_block_id(blocks: &Vec<Block>) -> String {

    let block = block_by_name(blocks, &String::from("root"));

    if block.is_some() {

        for nested_block in block.unwrap().get_data() {
            let matches = find_ids(&nested_block.data, RegexTypes::All);
            if matches.len() == 1 {
                return matches.get(0).unwrap().to_string();
            }
        }
    }

    error!("Failed finding id of configuration.");
    panic!("Failed finding id of configuration.");
}

/// Возвращает текст основного блока
pub fn main_block_data(id: &String, blocks: &Vec<Block>) -> Vec<u8> {

    let block = block_by_name(blocks, id);

    if block.is_none() {
        error!("Failed reading text of main block.");
        panic!("Failed reading text of main block.");
    }

    simply_block_data(&block.unwrap())
}

/// Возращает данные простого блока (блок у которого нет вложенных блоков)
pub fn simply_block_data(block: &Block) -> Vec<u8> {
    let block_data = block.get_data();

    if block_data.len() != 1 {
        error!("Incorrect content of the block. Block is not simple.");
        panic!("Incorrect content of the block. Block is not simple.");
    }

    block_data.get(0).unwrap().data.clone()
}

/// Выполняет поиск координат с описанием типа
///
/// Примеры возможных результатов:
/// 1. 37f2fa9a-b276-11d4-9435-004095e12fc7,1,d1e22a47-ab34-43b8-9b0e-a27113e944ad
/// 2. 37f2fa9a-b276-11d4-9435-004095e12fc7,1,{d1e22a47-ab34-43b8-9b0e-a27113e944ad, 0}
pub fn find_type_coordinates(type_id: &'static str, data: &Vec<u8>) -> Option<(Substr)> {

    let start_position: usize = match find_text(data, &*format!(r"{}{},", "{", type_id)) {
        None => return None,
        Some(v) => v,
    };

    let mut end_position: usize = start_position;

    let mut open_bracket = 0;
    for b in data[start_position..].iter() {
        open_bracket += match *b {
            b'{' => 1,
            b'}' => -1,
            _ => 0,
        };

        end_position += 1;

        if open_bracket == 0 {
            break;
        }
    }

    if open_bracket != 0 {
        return None;
    }

    return Some(Substr::new(start_position + 1, end_position - 1)); // +1 и -1 чтобы пропустить фигурные скобки
}

/// Выполняет поиск в строке по указанному типу регулярного выражения
fn find_by_regex(text: &String, expression: RegexTypes) -> regex::FindCaptures {

    match expression {
        RegexTypes::All => RE_ALL_IDS.captures_iter(&*text),
        RegexTypes::ElementsOfType => RE_ELEMENTS_OF_TYPE.captures_iter(&*text),
        RegexTypes::IdAndName => RE_ID_AND_NAME.captures_iter(&*text),
    }
}

/// Выполняет поиск всех идентификаторов в указанной строке
pub fn find_ids(data: &Vec<u8>, id_type: RegexTypes) -> Vec<String> {

    let text = bytes_to_string!(data, "Failed converting data to text for reading IDs.");
    let mut retval = <Vec<String>>::new();

    for cap in find_by_regex(&text, id_type) {
        match cap.name("id") {
            Some(v) => retval.push(v.to_string()),
            None => (),
        }
    }

    retval
}

/// Выполняет поиск идентификаторов и наименований в указанной строке
pub fn find_ids_and_names(data: &Vec<u8>) -> Vec<(String, String)> {

    let text = bytes_to_string!(data,
                                "Failed converting data to text for reading IDs and names.");
    let mut retval = <Vec<(String, String)>>::new();

    let mut id = String::new();

    for cap in find_by_regex(&text, RegexTypes::IdAndName) {
        match cap.name("id") {
            Some(v) => id = v.to_string(),
            None => (),
        }
        match cap.name("name") {
            Some(name) => retval.push((id.clone(), name.to_string())),
            None => (),
        }
    }

    retval
}



#[cfg(test)]
mod tests {
    use super::{block_by_name, main_conf_block_id, find_type_coordinates, find_ids,
                find_ids_and_names, RegexTypes};
    use structure::block::Block;
    use meta_data::substr::Substr;

    #[test]
    fn test_find_name() {

        let text =
            "{4fe87c89-9ad4-43f6-9fdb-9dc83b3879c6,1,{0,{2,b8c42329-e3ee-47c0-9d9e-0fae37b7eefa,\
             77ea1b8f-dd79-4717-9dba-5628e7f348cf},{0,{0,0,b8c42329-e3ee-47c0-9d9e-0fae37b7eefa},\
             \"СправочникКоманда1\",},0}}}}}}"
                .as_bytes()
                .to_vec();

        let vals = find_ids_and_names(&text);
        assert_eq!(1, vals.len());
        assert_eq!((String::from("b8c42329-e3ee-47c0-9d9e-0fae37b7eefa"),
                    String::from(r"СправочникКоманда1")),
                   *vals.get(0).unwrap());

        let text = "{0,0,84d6683f-8f5c-450e-918b-fc154c58dcde},\
                                 \"СсылкаНаСправочникСГруппами\",
                                 \
                                 {0,0,7820d564-9910-4b25-9ace-4238715e7247},\"Код\""
            .as_bytes()
            .to_vec();

        let vals = find_ids_and_names(&text);
        assert_eq!(2, vals.len());
        assert_eq!((String::from("84d6683f-8f5c-450e-918b-fc154c58dcde"),
                    String::from(r"СсылкаНаСправочникСГруппами")),
                   *vals.get(0).unwrap());
        assert_eq!((String::from("7820d564-9910-4b25-9ace-4238715e7247"), String::from(r"Код")),
                   *vals.get(1).unwrap())
            ;
    }

    #[test]
    fn test_find_ids() {

        let text = "{30d554db-541e-4f62-8970-a1c6dcfeb2bc,0},\
                    \n{37f2fa9a-b276-11d4-9435-004095e12fc7,1,\
                    d1e22a47-ab34-43b8-9b0e-a27113e944ad,39bddf6a-0c3c-452b-921c-d99cfa1c2f1a},\
                    \n{39bddf6a-0c3c-452b-921c-d99cfa1c2f1b,0},"
            .as_bytes()
            .to_vec();
        let substr = find_type_coordinates("37f2fa9a-b276-11d4-9435-004095e12fc7", &text).unwrap();

        let type_description = part_bytes!(text, substr);
        let ids = find_ids(&type_description, RegexTypes::ElementsOfType);
        assert_eq!(vec!["d1e22a47-ab34-43b8-9b0e-a27113e944ad",
                        "39bddf6a-0c3c-452b-921c-d99cfa1c2f1a"],
                   ids);

        let text = "{4fe87c89-9ad4-43f6-9fdb-9dc83b3879c6,1,{0,{2,\
                                 b8c42329-e3ee-47c0-9d9e-0fae37b7eefa,\
                                 078a6af8-d22c-4248-9c33-7e90075a3d2c},{3,0,{1,\
                                 77ea1b8f-dd79-4717-9dba-5628e7f348cf},{0,\
                                 {0,0,b8c42329-e3ee-47c0-9d9e-0fae37b7eefa},\
                                 \"СправочникКоманда1\",},0}}}}}}"
            .as_bytes()
            .to_vec();
        let substr = find_type_coordinates("4fe87c89-9ad4-43f6-9fdb-9dc83b3879c6", &text).unwrap();

        let type_description = part_bytes!(text, substr);
        let ids = find_ids(&type_description, RegexTypes::IdAndName);
        assert_eq!(vec!["b8c42329-e3ee-47c0-9d9e-0fae37b7eefa"], ids);
    }


    #[test]
    fn test_block_by_name() {

        let mut blocks = vec![Block::new("b1", &vec![]), Block::new("b2", &&vec![])];
        let block = block_by_name(&mut blocks, &String::from("b2"));

        assert!(block.is_some());
    }

    #[test]
    fn test_main_conf_block_id() {

        let root_block_text = String::from("{2,be22b29f-2db7-4fcb-8772-eeb5500d2170,\
                                              rXTtt26LznRtS6f248iyFJYHVq5J/wLbtrzWkS1AXcDOr8hHHU\
                                              DsZnKRJhFSdHfMYClc5jaEL32sG19910Rs2w==}");
        let block = Block::new("root", &root_block_text.into_bytes());
        assert_eq!("root", block.id());

        let id = main_conf_block_id(&mut vec![block]);
        assert_eq!("be22b29f-2db7-4fcb-8772-eeb5500d2170", id);
    }

    #[test]
    fn test_find_type_coordinates() {

        let result = "type,f{}";

        assert_eq!(None,
                   find_type_coordinates("type", &"{type".as_bytes().to_vec()));
        assert_eq!(None,
                   find_type_coordinates("type", &"{}".as_bytes().to_vec())) ;

        // v.1
        let text = format!("s{}{}{}f", "{", result, "}").as_bytes().to_vec();
        let substr = find_type_coordinates("type", &text).unwrap();
        assert_eq!(Substr::new(2, 10), substr);
        assert_eq!(result.as_bytes().to_vec(), part_bytes!(text, substr));

        // v.2
        let text = format!("{}{}{}", "{", result, "}").as_bytes().to_vec();
        let substr = find_type_coordinates("type", &text).unwrap();
        assert_eq!(Substr::new(1, 9), substr);
        assert_eq!(result.as_bytes().to_vec(), part_bytes!(text, substr));
    }
}
