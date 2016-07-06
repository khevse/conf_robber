
use std::collections::HashMap;

/// Параметры подключения к информационной базе
#[derive(Clone)]
pub struct ConnectionSettings {
    platform: String,
    path: String,
    user_name: String,
    user_pwd: String,
    storage_path: String,
    storage_user_name: String,
    storage_user_pwd: String,
}

impl ConnectionSettings {
    pub fn from_map(attributes: &HashMap<String, String>) -> ConnectionSettings {

        return ConnectionSettings::new(attributes.get("platform")
                                           .unwrap_or(&String::new()),
                                       attributes.get("path")
                                           .unwrap_or(&String::new()),
                                       attributes.get("user_name")
                                           .unwrap_or(&String::new()),
                                       attributes.get("user_pwd")
                                           .unwrap_or(&String::new()),
                                       attributes.get("storage_path")
                                           .unwrap_or(&String::new()),
                                       attributes.get("storage_user_name")
                                           .unwrap_or(&String::new()),
                                       attributes.get("storage_user_pwd")
                                           .unwrap_or(&String::new()));

    }

    pub fn new(platform: &String,
               path: &String,
               user_name: &String,
               user_pwd: &String,
               storage_path: &String,
               storage_user_name: &String,
               storage_user_pwd: &String)
               -> ConnectionSettings {

        return ConnectionSettings {
            platform: platform.clone(),
            path: path.clone(),
            user_name: user_name.clone(),
            user_pwd: user_pwd.clone(),
            storage_path: storage_path.clone(),
            storage_user_name: storage_user_name.clone(),
            storage_user_pwd: storage_user_pwd.clone(),
        };
    }

    pub fn platform<'a>(&'a self) -> &'a String {
        return &self.platform;
    }

    pub fn path<'a>(&'a self) -> &'a String {
        return &self.path;
    }

    pub fn user_name<'a>(&'a self) -> &'a String {
        return &self.user_name;
    }

    pub fn user_pwd<'a>(&'a self) -> &'a String {
        return &self.user_pwd;
    }

    pub fn storage_path<'a>(&'a self) -> &'a String {
        return &self.storage_path;
    }

    pub fn storage_user_name<'a>(&'a self) -> &'a String {
        return &self.storage_user_name;
    }

    pub fn storage_user_pwd<'a>(&'a self) -> &'a String {
        return &self.storage_user_pwd;
    }
}

#[test]
fn test_create_connection_settings() {

    let platform = String::from("platform");
    let path = String::from("path");
    let user_name = String::from("user_name");
    let user_pwd = String::from("user_pwd");
    let storage_path = String::from("storage_path");
    let storage_user_name = String::from("storage_user_name");
    let storage_user_pwd = String::from("storage_user_pwd");

    let mut settings: HashMap<String, String> = HashMap::new();
    settings.insert(platform.clone(), platform.clone());
    settings.insert(path.clone(), path.clone());
    settings.insert(user_name.clone(), user_name.clone());
    settings.insert(user_pwd.clone(), user_pwd.clone());
    settings.insert(storage_path.clone(), storage_path.clone());
    settings.insert(storage_user_name.clone(), storage_user_name.clone());
    settings.insert(storage_user_pwd.clone(), storage_user_pwd.clone());

    let test_data = ConnectionSettings::from_map(&settings);
    assert_eq!(&*platform, test_data.platform());
    assert_eq!(&*path, test_data.path());
    assert_eq!(&*user_name, test_data.user_name());
    assert_eq!(&*user_pwd, test_data.user_pwd());
    assert_eq!(&*storage_path, test_data.storage_path());
    assert_eq!(&*storage_user_name, test_data.storage_user_name());
    assert_eq!(&*storage_user_pwd, test_data.storage_user_pwd());
}
