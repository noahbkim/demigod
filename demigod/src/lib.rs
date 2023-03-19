use std::collections::HashMap;
use std::rc::{Weak, Rc};

pub mod message;

struct Database<T>
{
    items: Vec<T>,
    by_name: HashMap<String, usize>,
}

struct Vector2 {
    x: f32,
    y: f32,
}

struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

struct BitStream {}

enum DataTableValue {
    Bool(bool),
    U32(u32),
    I32(i32),
    Float(f32),
    String(String),
    U64(u64),
    I64(i64),
    Array(Vec<DataTableValue>),
}

impl DataTableValue {
    fn update(&mut self, stream: &mut BitStream) {

    }
}

struct DataTableProperty {

}

struct DataTableExclude {

}

struct DataTable {
    name: String,
    properties: Database<DataTableProperty>,
    excludes: Vec<DataTableExclude>,
    server_class: Weak<ServerClass>,
    is_array: bool,
}

type ServerClassIndex = u16;

struct ServerClass {
    index: ServerClassIndex,
    name: String,
    base_class: Rc<ServerClass>,
    data_table: Rc<DataTable>,
}

struct Entity {
    class: Rc<ServerClass>,
    values: Vec<DataTableValue>,
}
