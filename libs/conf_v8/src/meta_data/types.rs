use std::collections::HashMap;

/// Идентификатор типа "Любая ссылка"
pub const ANY_REF: &'static str = "280f5f0e-9c8a-49cc-bf6d-4d296cc17a63";

/// Идентификатор типа "Неопределенная ссылка"
pub const EMPTY_REF: &'static str = "00000000-0000-0000-0000-000000000000";

/// Идентификаторы внутренних типов
pub const FORMS_ID_DOC: &'static str = "d5b0e5ed-256d-401c-9c36-f630cafd8a62";
pub const FORMS_ID_CATALOG: &'static str = "fdf816d2-1ead-11d5-b975-0050bae0a95d";

pub const PROPS_ID: &'static str = "45e46cbc-3e24-4165-8b7b-cc98a6f80211";
pub const LAYOUTS_ID: &'static str = "3daea016-69b7-4ed4-9453-127911372fe6";
pub const COMMANDS_ID: &'static str = "4fe87c89-9ad4-43f6-9fdb-9dc83b3879c6";
pub const TABULAR_SELECTIONS_ID: &'static str = "21c53e09-8950-4b5e-a6a0-1054f1bbc274";

pub const DOCUMENT: &'static str = r"061d872a-5787-460e-95ac-ed74ea3a3e84";
pub const CATALOG: &'static str = r"cf4abea6-37b2-11d4-940f-008048da11f9";

/// Возвращает коллекцию идентификаторов типов и их наименования
pub fn get_types() -> HashMap<&'static str, &'static str> {

    let mut retval: HashMap<&'static str, &'static str> = HashMap::new();

    retval.insert(r"09736b02-9cac-4e3f-b4f7-d3e9576ab948", r"Роли");
    retval.insert(r"0c89c792-16c3-11d5-b96b-0050bae0a95d",
                  r"ОбщиеМакеты");
    retval.insert(r"0fe48980-252d-11d6-a3c7-0050bae0a776",
                  r"ОбщиеМодули");
    retval.insert(r"11bdaf85-d5ad-4d91-bb24-aa0eee139052",
                  r"РегламентныеЗадания");
    retval.insert(r"15794563-ccec-41f6-a83c-ec5f7b9a5bc1",
                  r"ОбщиеРеквизиты");
    retval.insert(r"24c43748-c938-45d0-8d14-01424a72b11e",
                  r"ПараметрыСеанса");
    retval.insert(r"30d554db-541e-4f62-8970-a1c6dcfeb2bc",
                  r"ПараметрыФункциональныхОпций");
    retval.insert(r"37f2fa9a-b276-11d4-9435-004095e12fc7",
                  r"Подсистемы");
    retval.insert(r"39bddf6a-0c3c-452b-921c-d99cfa1c2f1b",
                  r"Интерфейсы");
    retval.insert(r"3e5404af-6ef8-4c73-ad11-91bd2dfac4c8", r"Стили");
    retval.insert(r"c045099e-13b9-4fb6-9d50-fca00202971e",
                  r"ОпределяемыеТипы");
    retval.insert(r"3e7bfcc0-067d-11d6-a3c7-0050bae0a776",
                  r"КритерииОтбора");
    retval.insert(r"46b4cd97-fd13-4eaa-aba2-3bddd7699218",
                  r"ХранилищаНастроек");
    retval.insert(r"4e828da6-0f44-4b5b-b1c0-a2b3cfe7bdcc",
                  r"ПодпискиНаСобытия");
    retval.insert(r"58848766-36ea-4076-8800-e91eb49590d7",
                  r"ЭлементыСтиля");
    retval.insert(r"7dcd43d9-aca5-4926-b549-1842e6a4e8cf",
                  r"ОбщиеКартинки");
    retval.insert(r"857c4a91-e5f4-4fac-86ec-787626f1c108",
                  r"ПланыОбмена");
    retval.insert(r"8657032e-7740-4e1d-a3ba-5dd6e8afb78f",
                  r"WebСервисы");
    retval.insert(r"9cd510ce-abfc-11d4-9434-004095e12fc7", r"Языки");
    retval.insert(r"af547940-3268-434f-a3e7-e47d6d2638c3",
                  r"ФункциональныеОпции");
    retval.insert(r"cc9df798-7c94-4616-97d2-7aa0b7bc515e", r"XDTO");
    retval.insert(r"d26096fb-7a5d-4df9-af63-47d04771fa9b", r"WSСсылки");
    retval.insert(r"0195e80c-b157-11d4-9435-004095e12fc7",
                  r"Константы");
    retval.insert(DOCUMENT, r"Документы");
    retval.insert(r"07ee8426-87f1-11d5-b99c-0050bae0a95d",
                  r"ОбщиеФормы");
    retval.insert(r"13134201-f60b-11d5-a3c7-0050bae0a776",
                  r"РегистрыСведений");
    retval.insert(r"1c57eabe-7349-44b3-b1de-ebfeab67b47d",
                  r"ГруппыКоманды");
    retval.insert(r"2f1a5187-fb0e-4b05-9489-dc5dd6412348",
                  r"ОбщиеКоманды");
    retval.insert(r"36a8e346-9aaa-4af9-bdbd-83be3c177977",
                  r"НумераторыДокументов");
    retval.insert(r"4612bd75-71b7-4a5c-8cc5-2b0b65f9fa0d", r"Жуналы");
    retval.insert(r"631b75a0-29e2-11d6-a3c7-0050bae0a776", r"Отчеты");
    retval.insert(r"82a1b659-b220-4d94-a9bd-14d757b95a48",
                  r"ПланыВидовХарактеристик");
    retval.insert(r"b64d9a40-1642-11d6-a3c7-0050bae0a776",
                  r"РегистрыНакопления");
    retval.insert(r"bc587f20-35d9-11d6-a3c7-0050bae0a776",
                  r"ПоследовательностиДокументов");
    retval.insert(r"bf845118-327b-4682-b5c6-285d2a0eb296",
                  r"Обработки");
    retval.insert(CATALOG, r"Справочники");
    retval.insert(r"f6a80749-5ad7-400b-8519-39dc5dff2542",
                  r"Перечисления");
    retval.insert(r"30b100d6-b29f-47ac-aec7-cb8ca8a54767",
                  r"ПланыВидовРасчетов");
    retval.insert(r"2deed9b8-0056-4ffe-a473-c20a6c32a0bc",
                  r"РегистрыБухгалтерскогоУчета");
    retval.insert(r"238e7e88-3c5f-48b2-8a3b-81ebbecb20ed",
                  r"РегистрыРачета");
    retval.insert(r"fcd3404e-1523-48ce-9bc0-ecdb822684a1",
                  r"БизнесПроцессы");
    retval.insert(r"3e63355c-1378-4953-be9b-1deb5fb6bec5", r"Задачи");
    retval.insert(r"5274d9fc-9c3a-4a71-8f5e-a0db8ab23de5",
                  r"ВнешниеИсточникиДанных");

    return retval;
}
