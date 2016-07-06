
use meta_data;

use settings;
use structure::block::Block;
use meta_data::description::Description;
use aho_corasick::{Automaton, AcAutomaton};

/// Выполняет фильтрацию блоков, которые необходимы, остальные блоки удаляются
pub fn filter(blocks: &mut Vec<Block>, settings: &settings::Settings) {

    info!("Filtrating of blocks");

    let conf_id = meta_data::reader::main_conf_block_id(blocks);
    let mut conf_data = meta_data::reader::main_block_data(&conf_id, blocks);
    let conf_desc = Description::new(&conf_id, &conf_data);

    // Исключаем идентификаторы файла поставки, т.к. конфигурация уже получается не полной,
    // то эта информация уже является не актуальной и должна быть удалена
    let mut except_blocks_ids = distributive_ids(&blocks, &conf_desc);

    let mut force_blocks_ids =
        vec!["root", "version", "versions", &*conf_id, conf_desc.internal_id()]
            .iter()
            .map(|x| String::from(*x))
            .collect::<Vec<String>>();

    const internal_types_ids: [&'static str; 5] = [meta_data::types::FORMS_ID_DOC,
                                                   meta_data::types::FORMS_ID_CATALOG,
                                                   meta_data::types::PROPS_ID,
                                                   meta_data::types::COMMANDS_ID,
                                                   meta_data::types::LAYOUTS_ID];

    let mut deleted_ref_ids = <Vec<String>>::new();

    let CATALOG_ID: String = String::from(meta_data::types::CATALOG);
    let DOCUMENT_ID: String = String::from(meta_data::types::DOCUMENT.to_string());

    for (type_id, type_name) in meta_data::types::get_types() {
        let coordinates = find_type_coordinates(type_id, type_name, &conf_data);
        let mut obj_ids = metadata_blocks_ids(&conf_data, &coordinates);
        let type_filter = find_type_filter(type_name, &settings);

        // [3] Определяем идентификаторы блоков, которые соответствуют типу и заданным фильтрам
        for item in blocks.iter() {
            if obj_ids.iter().find(|x| (*x).eq(&*item.id())).is_none() {
                continue;
            }

            let mut data = meta_data::reader::simply_block_data(item);
            let desc = Description::new(item.id(), &data);
            let filtr = check_object_name(&type_filter, desc.name().to_string());

            if filtr.is_none() {
                // Имя объекта метаданных не найдено в списке нужных, следовательно удаляем его идентификатор
                obj_ids.retain(|x| x.ne(&*item.id()));

                // Записываем идентификаторы внешних ссылок, чтобы удалить из оставшихся объектов
                deleted_ref_ids.extend_from_slice(&desc.reference_ids()[..]);
                continue;
            }

            let filtr = filtr.unwrap();

            // Если объект не основной, то удаляем модули объекта и менеджера
            if !filtr.main() {
                if CATALOG_ID.eq(type_id) {
                    except_blocks_ids.push(format!("{}.0", item.id())); // модуль объекта
                    except_blocks_ids.push(format!("{}.3", item.id())); // модуль менеджера справочника
                } else if DOCUMENT_ID.eq(type_id) {
                    except_blocks_ids.push(format!("{}.0", item.id())); // модуль объекта
                    except_blocks_ids.push(format!("{}.2", item.id())); // модуль менеджера документа
                }
            }

            // Отбираем блоки внутренних типов
            for internal_type_id in internal_types_ids.iter() {
                let internal_ids = internal_objects_ids(internal_type_id, &desc);

                let (force_internal_ids, except_internal_ids) =
                    filtr_internal_ids(&filtr, internal_type_id, &internal_ids, &blocks);

                force_blocks_ids.extend_from_slice(&force_internal_ids[..]);
                except_blocks_ids.extend_from_slice(&except_internal_ids[..]);

                if !except_internal_ids.is_empty() {
                    update_internal_ids(&mut data,
                                        internal_type_id,
                                        force_internal_ids,
                                        except_internal_ids);

                    item.set_data(&item.id(), &data);
                }
            }

            force_blocks_ids.push(item.id().clone());
        }

        replace_bytes!(conf_data, coordinates, type_desc(type_id, &obj_ids));
    }

    // Удаляем блоки которые не прошли филтр
    blocks.retain(|x| {
        force_blocks_ids.iter().find(|y| (*x).id().starts_with(&*(*y))).is_some() &&
        except_blocks_ids.iter().find(|y| (*x).id().eq(&*(*y))).is_none()
    });

    // Обновляем описание конфигурации
    let conf_block = meta_data::reader::block_by_name(blocks, &conf_id).unwrap();
    conf_block.set_data(&conf_id, &conf_data);

    remove_references_deleted_blocks(&blocks, deleted_ref_ids);

    info!("-Filtrating of blocks");
}

/// Возращает идентификаторы файла поставки конфигурации
fn distributive_ids(blocks: &Vec<Block>, conf_desc: &Description) -> Vec<String> {

    info!("Reading IDs of blocks with data of distributive.");

    let mut retval = <Vec<String>>::new();

    let dist_id = format!("{}.4", conf_desc.internal_id());
    retval.push(dist_id.clone());

    for item in blocks.iter() {
        if item.id().ne(&*dist_id) {
            continue;
        }

        let data = meta_data::reader::simply_block_data(item);
        let dist_ids = meta_data::reader::find_ids(&data, meta_data::reader::RegexTypes::All);

        for id in &dist_ids {
            retval.push(format!("{}.{}", conf_desc.block_id(), id))
        }

        break;
    }

    retval.sort();
    retval.dedup();

    // TODO: правильным решением является два элемента этой коллекции
    // - первый блок с индентификатором: <идентификатор конфигурации>.4
    // - второй блок: <первый идентификатор из данных первого блока>.<идентификатор блока с описанием конфигурации>

    info!("-Reading IDs of blocks with data of distributive: {}",
          retval.len());

    retval
}

/// Определяет координаты типа в описании конфигурации
fn find_type_coordinates(type_id: &'static str,
                         type_name: &'static str,
                         conf_data: &Vec<u8>)
                         -> meta_data::substr::Substr {

    let coordinates = meta_data::reader::find_type_coordinates(type_id, &conf_data);

    if coordinates.is_none() {
        // Возможно что-то изменилось в конфигурационном файле и для типа определен новый идентификатор
        error!("Failed finding type '{}' coordinates", type_name);
        panic!("Failed finding type '{}' coordinates", type_name);
    }

    coordinates.unwrap()
}

/// Определяет параметры фильтрации типа
fn find_type_filter(type_name: &'static str,
                    settings: &settings::Settings)
                    -> Vec<settings::metadata::Metadata> {

    let mut type_filter = settings.metadata_selections(&type_name.to_string());
    if type_filter.is_none() {
        // Нет отбора по типу метаданных, но блоки необходимо обработать т.к. на них могут быть указаны ссылки в других объектах
        type_filter = Some(Vec::new());
    }

    type_filter.unwrap()
}

/// Поиск идентификаторов блоков с описанием объектов метаданных, которые относятся к типу
fn metadata_blocks_ids(conf_data: &Vec<u8>,
                       coordinates: &meta_data::substr::Substr)
                       -> Vec<String> {

    let type_description = part_bytes!(&conf_data, coordinates);
    meta_data::reader::find_ids(&type_description,
                                meta_data::reader::RegexTypes::ElementsOfType)
}

/// Сравнить имя с шаблоном
#[inline(always)]
fn compare_name(full_name: &String, name_template: String) -> bool {

    let mut name_template = name_template;
    let i = name_template.find("*");

    match i.is_some() {
        true => {
            let i = i.unwrap();
            name_template.remove(i);

            name_template.is_empty() || (i == 0 && full_name.ends_with(&*name_template)) ||
            (i > 0 && full_name.starts_with(&*name_template))
        }
        false => full_name.eq(&*name_template),
    }
}

/// Проверить соотвествие имени объекта установленному фильтру метаданных
#[inline(always)]
fn check_object_name<'a>(type_filter: &'a Vec<settings::metadata::Metadata>,
                         object_name: String)
                         -> Option<&'a settings::metadata::Metadata> {

    type_filter.iter().find(|&x| compare_name(&object_name, x.name().clone()))
}

/// Проверить имя внутреннего объекта (формы, шаблона и т.д.)
fn check_internal_object_name(blocks: &Vec<Block>, block_id: &String, names: &Vec<String>) -> bool {

    let mut except = false;

    for item in blocks.iter() {
        if item.id().ne(&*block_id) {
            continue;
        }

        let data = meta_data::reader::simply_block_data(item);
        let data = Description::new(&block_id, &data);
        let name = String::from(data.name().clone());

        except = names.iter()
            .find(|x| compare_name(&name, (*x).clone()))
            .is_some();
        break;
    }

    except
}

/// Получить идентфикаторы объектов внутренних типов (форм, шаблонов и т.д.)
fn internal_objects_ids(type_id: &'static str, desc: &Description) -> Vec<String> {

    let internal_ids = desc.internal_types_ids(type_id);
    if internal_ids.is_some() {
        return internal_ids.unwrap().clone();
    }

    <Vec<String>>::new()
}

/// Отфильтровать индентификаторы блоков внутренних типов (формы, шаблоны и т.д.)
fn filtr_internal_ids(object_filtr: &settings::metadata::Metadata,
                      internal_type_id: &'static str,
                      internal_ids: &Vec<String>,
                      blocks: &Vec<Block>)
                      -> (Vec<String>, Vec<String>) {

    let mut force_ids = <Vec<String>>::new();
    let mut except_ids = <Vec<String>>::new();

    if object_filtr.main() {
        let except_forms = object_filtr.except_forms();
        let except_templates = object_filtr.except_templates();

        let type_id = internal_type_id.clone().to_string();

        for id in internal_ids.iter() {
            let mut except = false;

            if !except_forms.is_empty() &&
               (type_id == meta_data::types::FORMS_ID_DOC ||
                type_id == meta_data::types::FORMS_ID_CATALOG) {

                except = check_internal_object_name(&blocks, &id, &except_forms);

            } else if !except_templates.is_empty() && type_id == meta_data::types::LAYOUTS_ID {

                except = check_internal_object_name(&blocks, &id, &except_templates);
            }

            if except {
                except_ids.push(id.clone());
            } else {
                force_ids.push(id.clone());
            }
        }
    } else {
        // Т.к. объект не основной, то удаляем все формы, шаблоны и т.д.
        except_ids.extend_from_slice(&internal_ids[..]);
    }

    (force_ids, except_ids)
}

/// Обновить описание идентификаторов внутренних типов в описании блока
fn update_internal_ids(data: &mut Vec<u8>,
                       internal_type_id: &'static str,
                       force_ids: Vec<String>,
                       except_ids: Vec<String>) {

    let internal_type_coordinates = meta_data::reader::find_type_coordinates(internal_type_id,
                                                                             &data);
    let internal_type_coordinates = internal_type_coordinates.unwrap();

    replace_bytes!(data,
                   internal_type_coordinates,
                   type_desc(internal_type_id, &force_ids));

    let aut = AcAutomaton::new(except_ids);
    let tmp_buf = data.clone();
    let mut it = aut.find(&tmp_buf[..]);

    loop {
        let m = match it.next() {
            Some(m) => m,
            None => break,
        };

        replace_bytes!(data, m.start, m.end, meta_data::types::EMPTY_REF);
    }
}

/// Заменить ссылки на удаленные объекты метаданных на тип "Любая ссылка"
fn remove_references_deleted_blocks(blocks: &Vec<Block>, deleted_ref_ids: Vec<String>) {

    info!("Removing references of deleted blocks.");

    let mut deleted_ref_ids = deleted_ref_ids;
    deleted_ref_ids.sort();
    deleted_ref_ids.dedup();

    let aut = AcAutomaton::new(deleted_ref_ids);

    for item in blocks.iter() {

        for nested_block in item.get_data() {
            // т.к. длина заменяемго идентификатора всегда равна длине нового,
            // то не боимся за то что координаты изменятся
            let mut data = nested_block.data;
            let temp_buf = data.clone();
            let mut it = aut.find(&temp_buf[..]);

            loop {
                let m = match it.next() {
                    Some(m) => m,
                    None => break,
                };

                replace_bytes!(data, m.start, m.end, meta_data::types::ANY_REF);

            }

            item.set_data(nested_block.attrs.id(), &data);
        }
    }

    info!("-Removing references of deleted blocks.");
}


/// Возвращает описание типа в формате: <идентификатор типа>,<количество подчиненных объектов>[, идентификаторы объектов]
/// Пример: fdf816d2-1ead-11d5-b975-0050bae0a95d,2,2605a1e0-a034-4fd1-885b-7a2fdf618144,3305a1e0-a034-4fd1-885b-7a2fdf618144
fn type_desc(type_id: &str, obj_ids: &Vec<String>) -> String {

    let mut obj_ids_text = String::new();
    let obj_ids_text = obj_ids.iter().fold(&mut obj_ids_text, |acc, x| {
        acc.push_str(",");
        acc.push_str(&*(*x));
        acc
    });

    format!("{},{}{}", type_id, obj_ids.len(), obj_ids_text)
}


#[cfg(test)]
mod tests {

    extern crate logger;
    extern crate settings;
    extern crate file_system;
    use std::path::Path;

    use settings::Settings;

    use structure;
    use super::filter;

    #[test]
    fn test_filter() {
        init_log();

        let mut blocks = read_conf();
        let settings = read_settings();

        let blocks_before = blocks.clone();
        filter(&mut blocks, &settings);

        let deleted = blocks_before.iter()
            .filter(|&x| blocks.iter().find(|y| x.id().eq(&*(y.id()))).is_none())
            .map(|x| x.id())
            .collect::<Vec<_>>();
        println!("deleted blocks: {:?}", deleted);

        assert!(!deleted.is_empty());
    }

    fn init_log() {
        let path_to_current_dir = file_system::get_current_dir()
            .ok()
            .expect("Failed read current directory.");
        let target_dir = Path::new(&path_to_current_dir)
            .join("target")
            .join("debug");
        let target_dir = file_system::path_to_str(target_dir.as_path());

        assert_eq!("", logger::get_log_directory());

        logger::init_log(&target_dir, Some(&String::from("info")));
    }

    fn read_conf() -> Vec<structure::block::Block> {

        let path_to_current_dir = file_system::get_current_dir()
            .ok()
            .expect("Failed read current directory.");
        let path_to_cf = Path::new(&path_to_current_dir)
                                    .parent().unwrap() // libs
                                    .parent().unwrap() // conf_robber
                                    .join("test_data")
                                    //.join("BP_3_0 (modern).cf");
        .join("original.cf");
        let path_to_cf = file_system::path_to_str(path_to_cf.as_path());

        let data = match file_system::read_file(&path_to_cf) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        let blocks = structure::reader::from_cf(&data);
        assert!(!blocks.is_empty());

        return blocks;
    }

    fn read_settings() -> Settings {

        let path_to_current_dir = file_system::get_current_dir()
            .ok()
            .expect("Failed read current directory.");
        let path_to_pom1c_xml = Path::new(&path_to_current_dir)
                                    .parent().unwrap() // libs
                                    .parent().unwrap() // conf_robber
                                    .join("test_data")
                                    .join("settings.xml");
        let path_to_pom1c_xml = file_system::path_to_str(path_to_pom1c_xml.as_path());

        let pom1c_xml = match file_system::read_file(&path_to_pom1c_xml) {
            Ok(v) => String::from_utf8(v).unwrap(),
            Err(e) => panic!("{}", e),
        };

        return Settings::new(&pom1c_xml);
    }
}
