
/// Конвертация байт в строку
#[macro_export]
macro_rules! bytes_to_string {
    ( $value:expr ) => {
        {
            bytes_to_string!($value, "Failed converting bytes to string")
        }
    };

    ( $value:expr, $error_message:expr ) => {
        {
            match String::from_utf8($value.clone()) {
                Ok(v) => v,
                Err(e) => {
                    let tmp_msg: String = format!("{}: {}.", $error_message, e);
                    error!("{}", tmp_msg);
                    panic!("{}", tmp_msg);
                }
            }
        }
    };

    ( $value:expr, $error_message:expr, $($message_args:tt )+ ) => {
        {
            bytes_to_string!($value, format!($error_message, $( $message_args )+))
        }
    };
}

/// Возвращает часть данных по указанным координатам
#[macro_export]
macro_rules! part_bytes {

    ( $value:expr, $start_pos:expr, $end_pos:expr ) => {
        {
            if $value.len() < $end_pos {
                error!("Failed substring: ({}, {}) => {:?}",
                       $start_pos,
                       $end_pos,
                       $value);
                panic!("Failed substring: ({}, {}) => {:?}",
                       $start_pos,
                       $end_pos,
                       $value);
            }

            $value[$start_pos..$end_pos].to_vec()
        }
    };

    ( $value:expr, $coordinates:expr ) => {
        {
            part_bytes!($value, $coordinates.start(), $coordinates.end())
        }
    };
}

/// Заменяет данные по указанным координатам
#[macro_export]
macro_rules! replace_bytes {

    ( $value:ident, $start_pos:expr, $end_pos:expr, $new_data:expr ) => {
        {
            let mut begin = $value.clone();
            $value.clear();

            let mut end = begin.split_off($start_pos);
            let end = end.split_off($end_pos - $start_pos);

            $value.extend_from_slice(&begin[..]);
            $value.extend_from_slice($new_data.as_bytes());
            $value.extend_from_slice(&end[..]);
        }
    };

    ( $value:ident, $coordinates:expr, $new_data:expr ) => {
        {
            replace_bytes!($value, $coordinates.start(), $coordinates.end(), $new_data);
        }
    };
}

/// Находит текст в коллекции байт
pub fn find_text(data: &Vec<u8>, text: &str) -> Option<usize> {

    if !data.is_empty() && data.len() >= text.len() {

        let tmp = text.as_bytes().to_vec();
        let max_pos = data.len() - tmp.len() + 1;

        for i in 0..max_pos {
            if data[i] != tmp[0] {
                continue;
            }

            let buf = &data[i..i + tmp.len()];
            if tmp == buf {
                let pos: usize = i;
                return Some(pos);
            }
        }
    }

    return None;
}

/// Координаты начала и окончания текста
#[derive(Debug, PartialEq)]
pub struct Substr {
    start: usize,
    end: usize,
}

impl Substr {
    pub fn new(start: usize, end: usize) -> Substr {
        Substr {
            start: start,
            end: end,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

#[test]
fn test_part() {

    use std::collections::HashMap;

    let mut test_data: HashMap<(usize, usize), &'static str> = HashMap::new();
    test_data.insert((0, 0), "");
    test_data.insert((0, 1), "a");
    test_data.insert((0, 2), "ab");
    test_data.insert((0, 3), "abc");
    test_data.insert((1, 2), "b");
    test_data.insert((1, 3), "bc");
    test_data.insert((2, 3), "c");
    test_data.insert((3, 3), "");

    for (k, v) in &test_data {
        let (start, end) = *k;
        println!("positions: {}-{}", start, end);

        // v.1
        let source_text = "abc".as_bytes().to_vec();
        assert_eq!(*v, bytes_to_string!(part_bytes!(source_text, start, end)));
        assert_eq!(bytes_to_string!(source_text), "abc");

        // v.2
        let source_text = "abc";
        assert_eq!((*v).as_bytes().to_vec(),
                   part_bytes!(source_text.as_bytes().to_vec(), start, end));
        assert_eq!(source_text, "abc");
    }
}


#[test]
fn test_replace() {

    use std::collections::HashMap;

    let mut test_data: HashMap<(usize, usize), &'static str> = HashMap::new();
    test_data.insert((0, 0), "<new>abc");
    test_data.insert((0, 1), "<new>bc");
    test_data.insert((1, 2), "a<new>c");
    test_data.insert((1, 3), "a<new>");
    test_data.insert((2, 3), "ab<new>");
    test_data.insert((3, 3), "abc<new>");

    for (k, v) in &test_data {
        let (start, end) = *k;
        println!("positions: {}-{}", start, end);

        // v.1
        let mut source_text = "abc".as_bytes().to_vec();
        replace_bytes!(source_text, Substr::new(start, end), String::from("<new>"));
        assert_eq!(*v, bytes_to_string!(source_text));

        // v.2
        let mut source_text = "abc".as_bytes().to_vec();
        replace_bytes!(source_text, start, end, String::from("<new>"));
        assert_eq!(*v, bytes_to_string!(source_text));
    }
}

#[test]
fn test_find_text() {

    let data = "aaa_id__bbb".as_bytes().to_vec();
    let result = find_text(&data, "id");
    assert_eq!(Some(4), result);

    let result = find_text(&data, "cc");
    assert_eq!(None, result);

    let result = find_text(&data, "aaa");
    assert_eq!(Some(0), result);

    let result = find_text(&data, "bbb");
    assert_eq!(Some(8), result);
}
