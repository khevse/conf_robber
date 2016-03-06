
#[macro_use]
extern crate log;
extern crate xml;

use std::ptr;
use std::collections::HashMap;

/// Element of the xml
pub struct XmlElement {
    pub name: String,
    pub parent: *mut XmlElement,
    pub attributes: HashMap<String, String>,
    pub childrens: Vec<XmlElement>,
    pub text: String,
}

impl XmlElement {
    /// Find all items by tag name
    pub fn find(&self, tag_name: &str) -> Vec<&XmlElement> {

        let mut retval: Vec<&XmlElement> = Vec::new();

        for item in &self.childrens {
            if item.name.eq(tag_name) {
                retval.push(item);
            }
        }

        return retval;
    }

    /// Find first item by tag name
    pub fn first(&self, tag_name: &str) -> Option<&XmlElement> {

        for item in &self.childrens {
            if item.name.eq(tag_name) {
                return Some(item);
            }
        }

        return None;
    }
}

/// DOM of the xml
pub struct XmlDOM {
    childrens: Vec<XmlElement>,
}

impl XmlDOM {
    pub fn parse(xml_text: &String) -> XmlDOM {

        let parser = xml::reader::EventReader::from_str(&*(*xml_text));
        let mut childrens: Vec<XmlElement> = Vec::new();
        let mut stack: Vec<*mut XmlElement> = Vec::new();

        for e in parser {
            match e {
                Ok(xml::reader::XmlEvent::StartElement { name, attributes, .. }) => {

                    let mut attrs: HashMap<String, String> = HashMap::new();
                    for a in attributes {
                        attrs.insert(a.name.local_name, a.value);
                    }

                    let parent: *mut XmlElement = match stack.last() {
                        None => ptr::null_mut(),
                        Some(v) => *v,
                    };

                    let element = XmlElement {
                        name: name.local_name,
                        attributes: attrs,
                        parent: parent,
                        childrens: Vec::new(),
                        text: String::new(),
                    };

                    if stack.is_empty() {
                        childrens.push(element);
                        stack.push(&mut *childrens.last_mut().unwrap());
                    } else {
                        unsafe {
                            (*parent).childrens.push(element);
                            stack.push(&mut *(*parent).childrens.last_mut().unwrap());
                        }
                    }

                }
                Ok(xml::reader::XmlEvent::Characters(data)) => {

                    let parent: *mut XmlElement = match stack.last() {
                        None => ptr::null_mut(),
                        Some(v) => *v,
                    };

                    unsafe {
                        (*parent).text = data;
                    }

                }
                Ok(xml::reader::XmlEvent::EndElement {..}) => {
                    stack.pop();
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        if childrens.len() != 1 {
            error!("A few root elements.");
            panic!("A few root elements.");
        }

        return XmlDOM { childrens: childrens };
    }

    pub fn root(&self) -> &XmlElement {
        return self.childrens.first().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use XmlDOM;

    #[test]
    fn test_xml_parse() {

        let pom1c_xml = get_test_data();
        let xml = XmlDOM::parse(&pom1c_xml);
        let root = xml.root();
        assert_eq!("project", root.name);
        assert_eq!("", root.text);
        assert_eq!(2, root.childrens.len());

        assert_eq!(1, root.find("platform").len());
        assert_eq!(1, root.find("sourceIB").len());

        let platform = root.first("platform").unwrap();
        let source_ib = root.first("sourceIB").unwrap();

        assert_eq!(r"C:\Program Files (x86)\1cv8\8.3.5.1517\bin\1cv8.exe",
                   platform.text);
        assert_eq!("", source_ib.text);

        assert_eq!(4, source_ib.attributes.len());
        assert_eq!(true, source_ib.attributes.contains_key("createNewCf"));
        assert_eq!(true, source_ib.attributes.contains_key("path"));
        assert_eq!(true, source_ib.attributes.contains_key("userName"));
        assert_eq!(true, source_ib.attributes.contains_key("userPwd"));

        assert_eq!("true", source_ib.attributes.get("createNewCf").unwrap());
        assert_eq!(r"E:\MyWork\C++\ConfRobber\tests\Data\ib",
                   source_ib.attributes.get("path").unwrap());
        assert_eq!("", source_ib.attributes.get("userName").unwrap());
        assert_eq!("", source_ib.attributes.get("userPwd").unwrap());

        assert_eq!(1, source_ib.find("objects").len());
    }

    fn get_test_data() -> String {
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

        return match file_system::read_file(&path_to_pom1c_xml) {
            Ok(v) => String::from_utf8(v).unwrap(),
            Err(e) => panic!("{}", e),
        };
    }
}
