use crate::{
    agent::Hypnosis,
    classes::{Class, VenomPlan},
    curatives::first_aid::FirstAidPriorities,
};

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
}
