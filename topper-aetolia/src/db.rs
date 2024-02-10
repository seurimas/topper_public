use std::convert::TryFrom;

use topper_core::timeline::db::DatabaseModule;

use crate::{
    agent::Hypnosis,
    classes::{Class, VenomPlan},
    curatives::first_aid::FirstAidPriorities,
};

pub const HINT_TREE: &str = "HINTS";

pub trait AetDatabaseModule {
    fn get_class(&self, who: &String) -> Option<Class>;

    fn set_class(&self, who: &String, class: Class);

    fn get_venom_plan(&self, stack_name: &String) -> Option<Vec<VenomPlan>>;

    fn get_hypno_plan(&self, stack_name: &String) -> Option<Vec<Hypnosis>>;

    fn get_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
    ) -> Option<FirstAidPriorities>;

    fn set_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
        priorities: FirstAidPriorities,
    );

    fn insert_hint(&self, key: &String, value: &String);

    fn get_hint(&self, key: &String) -> Option<String>;
}

impl<T: DatabaseModule> AetDatabaseModule for T {
    fn get_class(&self, who: &String) -> Option<Class> {
        let class_id = self.get("classes", who);
        if let Some(class_id) = class_id {
            if let [class_id] = class_id.as_ref() {
                Class::try_from(*class_id).ok()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn set_class(&self, who: &String, class: Class) {
        self.insert("classes", who, (&[class as u8]));
    }

    fn get_venom_plan(&self, stack_name: &String) -> Option<Vec<VenomPlan>> {
        self.get_json::<Vec<VenomPlan>>("stacks", stack_name)
    }

    fn get_hypno_plan(&self, stack_name: &String) -> Option<Vec<Hypnosis>> {
        self.get_json::<Vec<Hypnosis>>("hypnosis", stack_name)
    }

    fn set_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
        priorities: FirstAidPriorities,
    ) {
        self.insert_json::<FirstAidPriorities>(
            "first_aid",
            &format!("{}_{}", who, priorities_name),
            priorities,
        );
    }

    fn get_first_aid_priorities(
        &self,
        who: &String,
        priorities_name: &String,
    ) -> Option<FirstAidPriorities> {
        self.get_json::<FirstAidPriorities>("first_aid", &format!("{}_{}", who, priorities_name))
    }

    fn insert_hint(&self, key: &String, value: &String) {
        self.insert(HINT_TREE, key, value.as_bytes());
    }
    fn get_hint(&self, key: &String) -> Option<String> {
        self.get(HINT_TREE, key).and_then(|bytes| {
            std::str::from_utf8(&bytes)
                .map(|str_ref| str_ref.to_string())
                .ok()
        })
    }
}
