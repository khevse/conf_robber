
use meta_data::types::{FORMS_ID_DOC, FORMS_ID_CATALOG, PROPS_ID, LAYOUTS_ID, COMMANDS_ID,
                       TABULAR_SELECTIONS_ID};
use meta_data::reader::{RegexTypes, find_ids, find_ids_and_names, find_type_coordinates};
use meta_data::substr::find_text;

use std::collections::HashMap;

/// Описание объекта метаданных конфигурации(может содержать описание объекта метаданных, формы, модуля и т.д.)
pub struct Description {
    #[allow(dead_code)]
    block_id: String, // идентификатор блока
    internal_id: String, /* внутренний идентификатор объекта ( для всех типов соответствует идентификатору блока,
                          * но для описания конфигурации будет не соотвествовать ) */
    name: String,
    reference_ids: Vec<String>, /* Идентификаторы, один из которых используется в других объектах,
                                 * в виде указателя, чтобы показать что какой-то рекизит имееет тип этого объекта.
                                 * Расположение идентификатора находится в произвольной позиции второй
                                 * строки в зависимости от типа объекта.
                                 * Например для справочника и документа этот идентификатор находится
                                 * в третьей позиции строки, для перечисления в первой позиции */
    internal_ids: HashMap<&'static str, Vec<String>>, /* Идентификаторы вложенных объектов, таких как:
                                                       * - табличные части;
                                                       * - команды;
                                                       * - щаблоны;
                                                       * - формы;
                                                       * - свойства. */
}

impl Description {
    pub fn new(block_id: &String, data: &Vec<u8>) -> Description {

        // Поиск наименования объекта
        let mut object_name: Option<String> = None;

        // Первые три байта это маркер utf-8
        let is_configuration_data = data.len() > 2 && "{2,".as_bytes().to_vec().eq(&&data[3..6]);

        let mut internal_id = block_id.clone();

        for (id, name) in find_ids_and_names(data) {
            if is_configuration_data {
                internal_id = id;
                object_name = Some(name);
                break;
            }
            if block_id.eq(&*id) {
                object_name = Some(name);
                break;
            }
        }

        if object_name.is_none() {
            error!("Not found name of object for the block: {}", block_id);
            panic!("Not found name of object for the block: {}", block_id);
        }

        // Поиск идентификаторов, которые использются другими объектами для указания ссылки на этот объект
        let end_position = find_text(data, block_id).unwrap();
        let text_with_reference_ids = part_bytes!(data, 0, end_position);
        let reference_ids = find_ids(&text_with_reference_ids, RegexTypes::All)
            .iter()
            .map(|x| x.clone())
            .filter(|x| block_id.ne(&*x))
            .collect();

        // Поиск идентификаторов подчиненных объектов
        let mut internal_ids: HashMap<&'static str, Vec<String>> = HashMap::new();
        for type_id in &[FORMS_ID_DOC,
                         FORMS_ID_CATALOG,
                         PROPS_ID,
                         LAYOUTS_ID,
                         COMMANDS_ID,
                         TABULAR_SELECTIONS_ID] {

            let substr = match find_type_coordinates(type_id, data) {
                None => continue,
                Some(v) => v,
            };

            let buf = part_bytes!(data, substr);

            let mut ids: Vec<_> = find_ids_and_names(&buf)
                .iter()
                .map(|x| {
                    let (id, _) = x.clone();
                    id
                })
                .filter(|x| (*x).ne(type_id))
                .collect();
            if ids.is_empty() {
                ids = find_ids(&buf, RegexTypes::All)
                    .iter()
                    .map(|x| x.clone())
                    .filter(|x| (*x).ne(type_id))
                    .collect();
            }
            internal_ids.insert(type_id, ids);
        }

        return Description {
            block_id: block_id.clone(),
            internal_id: internal_id,
            name: object_name.unwrap(),
            reference_ids: reference_ids,
            internal_ids: internal_ids,
        };
    }

    #[allow(dead_code)]
    pub fn block_id<'a>(&'a self) -> &'a str {
        &*self.block_id
    }

    pub fn internal_id<'a>(&'a self) -> &'a str {
        &*self.internal_id
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &*self.name
    }

    pub fn internal_types_ids<'a>(&'a self, type_id: &str) -> Option<&'a Vec<String>> {
        self.internal_ids.get(type_id)
    }

    pub fn reference_ids<'a>(&'a self) -> &'a Vec<String> {
        &self.reference_ids
    }
}

#[cfg(test)]
mod tests {
    use super::Description;
    use meta_data::types::{FORMS_ID_DOC, PROPS_ID, LAYOUTS_ID, COMMANDS_ID, TABULAR_SELECTIONS_ID};

    #[test]
    fn test_new() {
        let text = "\
    {1,{37,fdb1bf03-6e62-4b6b-8fd1-6ce57d9175fb,\
                                 e13bf249-363d-4761-b6d7-420b068d915b,,{0,{0,{0,0,\
                                 9d1a9f27-cd9f-488b-b5e1-c8b410fb7856},\"Документ\",{1,\"ru\",\
                                 \"Документ\"},\"\"}},00000000-0000-0000-0000-000000000000,1,9,0,\
                                 1,1,974ece69-f9af-4130-9c4e-c7ac08cf9669,\
                                 00000000-0000-0000-0000-000000000000,\
                                 00000000-0000-0000-0000-000000000000,0,2,0,,5,\
                                 {21c53e09-8950-4b5e-a6a0-1054f1bbc274,1,{{1,{11,\
                                 a2f16e30-fd67-49d2-bed0-c35ea44de4bf,\
                                 2fc7b0e3-e949-4c2b-926c-26426802c080,\
                                 211ed9bc-ca03-42fd-826d-afa6f5de001f,\
                                 33285166-a97f-41ce-9d88-1fb2759f1f20,{0,{0,{0,0,\
                                 cf482c88-5f23-4b06-aca5-05fcf2be41b6},\"ТабличнаяЧасть1\",{0},\
                                 \"\"}}}},1,{888744e1-b616-11d4-9436-004095e12fc7,0}}},\
                                 {3daea016-69b7-4ed4-9453-127911372fe6,1,\
                                 9c00eead-82cb-4fdb-a798-7e7d584327bc, \
                                 1708fdaa-cbce-4289-b373-07a5a74bee91},\
                                 {45e46cbc-3e24-4165-8b7b-cc98a6f80211,1,\
                                 53d90c1c-07d1-432b-985c-f808bc26d83b,\
                                 }{4fe87c89-9ad4-43f6-9fdb-9dc83b3879c6,1,{{0,{0,0,0,{0,{2,\
                                 bf6f6886-73c1-4515-92ae-697f46bed2ac,\
                                 078a6af8-d22c-4248-9c33-7e90075a3d2c},{7,{3,0,{0},\"\",-1,-1,1,\
                                 0},3,{0},1,{0,0,0},0,{1,1af6d528-0b86-4fba-ab95-bd7475db03ba},\
                                 {\"Pattern\"},{0,{0,0,bf6f6886-73c1-4515-92ae-697f46bed2ac},\
                                 \"ДокументКоманда1\",{1,\"ru\",\"Документ \
                                 команда1\"},\"\"},0,0}}}},0}},\
                                 {d5b0e5ed-256d-401c-9c36-f630cafd8a62,1,\
                                 31529189-9dfa-4dd5-ad66-1c93d70d83e1}}".as_bytes().to_vec();


        let id = String::from("9d1a9f27-cd9f-488b-b5e1-c8b410fb7856");
        let desc = Description::new(&id, &text);
        assert_eq!(id, desc.block_id());
        assert_eq!("Документ", desc.name());

        assert_eq!(["31529189-9dfa-4dd5-ad66-1c93d70d83e1"].to_vec(),
                   get_internal_ids(desc.internal_types_ids(&FORMS_ID_DOC)));
        assert_eq!(["53d90c1c-07d1-432b-985c-f808bc26d83b"].to_vec(),
                   get_internal_ids(desc.internal_types_ids(&PROPS_ID)));
        assert_eq!(["9c00eead-82cb-4fdb-a798-7e7d584327bc",
                    "1708fdaa-cbce-4289-b373-07a5a74bee91"]
                       .to_vec(),
                   get_internal_ids(desc.internal_types_ids(&LAYOUTS_ID)));
        assert_eq!(["bf6f6886-73c1-4515-92ae-697f46bed2ac"].to_vec(),
                   get_internal_ids(desc.internal_types_ids(&COMMANDS_ID)));
        assert_eq!(["cf482c88-5f23-4b06-aca5-05fcf2be41b6"].to_vec(),
                   get_internal_ids(desc.internal_types_ids(&TABULAR_SELECTIONS_ID)));

        assert_eq!(["fdb1bf03-6e62-4b6b-8fd1-6ce57d9175fb",
                    "e13bf249-363d-4761-b6d7-420b068d915b"]
                       .to_vec(),
                   *desc.reference_ids());
    }

    fn get_internal_ids(val: Option<&Vec<String>>) -> Vec<String> {
        if val.is_some() {
            return val.unwrap()
                .iter()
                .map(|x| (*x).clone())
                .collect::<Vec<String>>();
        }

        return Vec::new();
    }
}
