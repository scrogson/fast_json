use rustler::{NifTerm, NifEnv, NifEncoder};
use rustler::types::map::map_new;
use rustler::types::atom::get_atom;

pub mod value_sink;
pub use self::value_sink::ValueSink;

pub struct TermSink<'a> {
    env: NifEnv<'a>,
    stack: Vec<NifTerm<'a>>,
}

impl<'a> TermSink<'a> {
    pub fn new(env: NifEnv<'a>, stack: Vec<NifTerm<'a>>) -> TermSink<'a> {
        TermSink {
            env: env,
            stack: stack,
        }
    }

    pub fn to_stack(self) -> Vec<NifTerm<'a>> {
        self.stack
    }

    pub fn push(&mut self, value: NifTerm<'a>) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> NifTerm<'a> {
        self.stack.pop().unwrap()
    }
}

impl<'a> ValueSink for TermSink<'a> {
    fn push_map(&mut self) {
        let map = map_new(self.env);
        self.push(map);
    }

    fn push_array(&mut self) {
        let vector: Vec<NifTerm<'a>> = vec![];
        let value = vector.encode(self.env);
        self.push(value);
    }

    fn push_string(&mut self, string: String) {
        let value = string.encode(self.env);
        self.push(value);
    }

    fn push_integer(&mut self, integer: i64) {
        let value = integer.encode(self.env);
        self.push(value);
    }

    fn push_float(&mut self, float: f64) {
        let value = float.encode(self.env);
        self.push(value);
    }

    fn push_bool(&mut self, boolean: bool) {
        let value = boolean.encode(self.env);
        self.push(value);
    }

    fn push_nil(&mut self) {
        let nil = get_atom("nil").unwrap().to_term(self.env);
        self.push(nil);
    }

    fn finalize_map(&mut self) {
        //
    }

    fn finalize_array(&mut self) {
        let array = self.pop();
        self.push(array.list_reverse().ok().unwrap());
    }

    fn pop_insert_map(&mut self, key: String) {
        let value = self.pop();
        let map = self.pop();
        let new_map = map.map_put(key.encode(self.env), value).ok().unwrap();

        self.push(new_map);
    }

    fn pop_insert_array(&mut self) {
        let value = self.pop();
        let array = self.pop();

        self.push(array.list_prepend(value));
    }
}
