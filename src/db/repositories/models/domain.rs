use poem_openapi::Object;
use serde::Serialize;

#[derive(Debug, Object, Serialize, Clone, Eq, PartialEq)]
pub struct Domain {}
