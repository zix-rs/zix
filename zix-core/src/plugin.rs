use std::any::Any;
use crate::entry::options::Opti;

pub trait Plugin: Any + Send + Sync {
    fn info(&self) -> Ext;
    fn run(&self);
}

pub struct Ext {
    pub typo:  Option<Vec<Opti>>,
    pub name:  &'static str,
    pub flag:  Option<&'static str>,
    pub sflag: Option<&'static str>,
    pub desc:  Option<&'static str>
}
