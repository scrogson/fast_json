use rustler::{NifTerm, NifEnv, NifEncoder};
use rustler::map::map_new;
use rustler::atom::get_atom;

pub mod value_sink;
pub use self::value_sink::ValueSink;

pub struct TermSink<'a> {
    env: &'a NifEnv,
    stack: Vec<NifTerm<'a>>,
}

impl<'a> TermSink<'a> {
    pub fn new(env: &'a NifEnv, stack: Vec<NifTerm<'a>>) -> TermSink<'a> {
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
        self.push(map_new(self.env));
    }

    fn push_array(&mut self) {
        let vector: Vec<NifTerm<'a>> = vec![];
        self.push(vector.encode(self.env));
    }

    fn push_string(&mut self, string: String) {
        self.push(string.encode(self.env));
    }

    fn push_integer(&mut self, integer: i64) {
        self.push(integer.encode(self.env));
    }

    fn push_float(&mut self, float: f64) {
        self.push(float.encode(self.env));
    }

    fn push_bool(&mut self, boolean: bool) {
        self.push(boolean.encode(self.env));
    }

    fn push_nil(&mut self) {
        self.push(get_atom("nil").unwrap().to_term(self.env));
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

        self.push(map.map_put(key.encode(self.env), value).ok().unwrap());
    }

    fn pop_insert_array(&mut self) {
        let value = self.pop();
        let array = self.pop();

        self.push(array.list_prepend(value));
    }
}
