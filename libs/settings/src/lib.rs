#[macro_use]
extern crate log;
extern crate xml;
extern crate regex;

#[cfg(test)]
extern crate encoding;
#[cfg(test)]
extern crate file_system;

use std::collections::HashMap;

pub mod metadata;

/// Настройки сборки
pub struct Settings {
    source_ib_connection_settings: HashMap<String, String>, /* Настройки подключения к исходной информационной базе */
    metadata_selections: Vec<metadata::Metadata>, // Настройки отбора
}

impl Settings {
    // Пример xml файла с настройками: conf_robber/test_data/pom1c.xml
    pub fn new(xml_text: &String) -> Settings {

        let xml = xml::XmlDOM::parse(xml_text);
        let xml_root = xml.root();

        let mut source_ib_connection_settings: HashMap<String, String> = HashMap::new();
        let mut metadata_selections: Vec<metadata::Metadata> = Vec::new();

        match xml_root.first("sourceIB") {
            Some(v) => {
                source_ib_connection_settings = v.attributes.clone();

                metadata_selections = match v.first("objects") {
                    None => Vec::new(),
                    Some(v) => {
                        let mut collection: Vec<metadata::Metadata> = Vec::new();
                        for item in &v.childrens {
                            let object = match metadata::Metadata::new(&item.name,
                                                                       &item.text,
                                                                       &item.attributes) {
                                Err(e) => {
                                    error!("{}", e);
                                    panic!("{}", e);
                                }
                                Ok(v) => v,
                            };

                            collection.push(object);
                        }
                        collection
                    }
                };
            }
            _ => (),
        }

        return Settings {
            source_ib_connection_settings: source_ib_connection_settings,
            metadata_selections: metadata_selections,
        };
    }

    /// Возвращает параметры подключения к исходной базе данных
    pub fn source_ib_connection_settings(&self) -> Option<HashMap<String, String>> {

        return match self.source_ib_connection_settings.is_empty() {
            true => None,
            false => {
                let mut retval: HashMap<String, String> = HashMap::new();
                for (k, v) in &self.source_ib_connection_settings {
                    retval.insert(k.clone(), v.clone());
                }
                Some(retval)
            }
        };
    }

    /// Возвращает параметры отбора метаданных запрошенного типа
    pub fn metadata_selections(&self, type_name: &String) -> Option<Vec<metadata::Metadata>> {

        let mut retval: Vec<metadata::Metadata> = Vec::new();

        for item in &self.metadata_selections {

            if type_name.eq(item.type_name()) {
                retval.push(item.clone());
            }
        }

        return match retval.is_empty() {
            true => None,
            false => Some(retval),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use encoding::{Encoding, EncoderTrap};
    use encoding::all::UTF_8;

    #[test]
    fn test_metadata_selections() {

        let settings = create();

        assert!(settings.metadata_selections(&str_to_string(r""))
                        .is_none());
        assert!(settings.metadata_selections(&str_to_string(r"Документы"))
                        .is_none());
        assert!(settings.metadata_selections(&str_to_string(r"Справочники"))
                        .is_some());
        assert!(settings.metadata_selections(&str_to_string(r"Языки"))
                        .is_some());
        assert!(settings.metadata_selections(&str_to_string(r"Обработки"))
                        .is_some());

        let test_data = settings.metadata_selections(&str_to_string(r"Обработки"))
                                .unwrap();
        assert_eq!(1, test_data.len());

        let test_data = test_data.get(0).unwrap();
        assert_eq!(&*str_to_string(r"Обработки"),
                   test_data.type_name());
        assert_eq!(r"test", test_data.name());
        assert_eq!(true, test_data.main());
    }

    #[test]
    fn test_source_ib_connection_settings() {
        let settings = create();
        assert!(settings.source_ib_connection_settings().is_some());

        match settings.source_ib_connection_settings() {
            None => assert!(false),
            Some(v) => {
                assert!(!v.get("platform")
                          .unwrap_or(&String::new())
                          .is_empty());
            }
        }

    }

    // For string with unicode symbols
    fn str_to_string(s: &str) -> String {
        let res = UTF_8.encode(s, EncoderTrap::Strict).unwrap();
        return String::from_utf8(res).unwrap();
    }

    fn create() -> Settings {
        extern crate file_system;
        use std::path::Path;

        let path_to_current_dir = file_system::get_current_dir()
                                      .ok()
                                      .expect("Failed read current directory.");
        let path_to_pom1c_xml = Path::new(&path_to_current_dir)
                                    .parent().unwrap() // libs
                                    .parent().unwrap() // conf_robber
                                    .join("test_data")
                                    .join("pom1c.xml");
        let path_to_pom1c_xml = file_system::path_to_str(path_to_pom1c_xml.as_path());

        let pom1c_xml = match file_system::read_file(&path_to_pom1c_xml) {
            Ok(v) => String::from_utf8(v).unwrap(),
            Err(e) => panic!("{}", e),
        };

        return Settings::new(&pom1c_xml);
    }
}
