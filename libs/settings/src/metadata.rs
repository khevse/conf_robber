
use regex;
use std::collections::HashMap;

/// Настройки объекта
#[derive(PartialEq,Debug,Clone)]
pub struct Metadata {
    type_name: String, /* Наименование типа которому принадлежит объект */
    name: String, // Наименование
    main: bool, /* Является основным объектом сборки (не будут удаляться команды, шаблоны, формы) */
}

impl Metadata {
    pub fn new(type_name: &String,
               name: &String,
               attributes: &HashMap<String, String>)
               -> Result<Metadata, String> {

        let re = match regex::Regex::new(r"(?P<bad_symbol>[^a-zA-Zа-яА-Я0-9_*])") {
            Ok(v) => v,
            Err(e) => {
                panic!(r"Failed regex string: {}", e);
            }
        };

        let name_temp = re.replace_all(&*name.clone(), "<$bad_symbol>");

        if name_temp.find("<").is_some() {
            return Err(format!("Filed name: {}", name_temp));
        } else if name_temp.is_empty() {
            return Err(format!("Empty name of metadata object: {:?}", type_name));
        }

        return Ok(Metadata {
            type_name: type_name.clone(),
            name: name_temp,
            main: attributes.get("main").unwrap_or(&String::from("false")).eq("true"),
        });
    }

    pub fn type_name<'a>(&'a self) -> &'a String {
        return &self.type_name;
    }

    pub fn name<'a>(&'a self) -> &'a String {
        return &self.name;
    }

    pub fn main(&self) -> bool {
        return self.main;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_full_attributes() {

        let test_data = create("name", "true").unwrap();

        assert_eq!("type", test_data.type_name());
        assert_eq!("name", test_data.name());
        assert_eq!(true, test_data.main());
    }

    #[test]
    fn test_main_attribute() {

        let test_data = create("name", "false").unwrap();
        assert_eq!(false, test_data.main());

        let test_data = create("name", "").unwrap();
        assert_eq!(false, test_data.main());
    }

    #[test]
    fn test_name_attribute() {

        let test_data = create(r"azAZаяАЯ09_*", "").unwrap();
        assert_eq!(r"azAZаяАЯ09_*", test_data.name());

        let test_data = create(r"nameаяАЯ+?-", "");
        assert!(test_data.is_err());
        assert_eq!(Err(String::from(r"Filed name: nameаяАЯ<+><?><->")),
                   test_data);
    }

    fn create(name: &str, main: &str) -> Result<Metadata, String> {

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(String::from("main"), String::from(main));

        return Metadata::new(&String::from("type"), &String::from(name), &attributes);
    }
}
