use rustler::{NifTerm, NifEnv, NifEncoder};
use rustler::types::map::map_new;
use atoms;

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
        self.stack.push(map_new(self.env));
    }

    fn push_array(&mut self) {
        let vector: Vec<NifTerm<'a>> = vec![];
        self.stack.push(vector.encode(self.env));
    }

    fn push_string(&mut self, string: String) {
        self.stack.push(string.encode(self.env));
    }

    fn push_integer(&mut self, integer: i64) {
        self.stack.push(integer.encode(self.env));
    }

    fn push_float(&mut self, float: f64) {
        self.stack.push(float.encode(self.env));
    }

    fn push_bool(&mut self, boolean: bool) {
        self.stack.push(boolean.encode(self.env));
    }

    fn push_nil(&mut self) {
        self.stack.push(atoms::nil().to_term(self.env));
    }

    fn finalize_map(&mut self) {
        //
    }

    fn finalize_array(&mut self) {
        let array = self.pop();
        self.stack.push(array.list_reverse().ok().unwrap());
    }

    fn pop_insert_map(&mut self, key: String) {
        let value = self.pop();
        let map = self.pop();

        self.stack.push(map.map_put(key.encode(self.env), value).ok().unwrap());
    }

    fn pop_insert_array(&mut self) {
        let value = self.pop();
        let array = self.pop();

        self.stack.push(array.list_prepend(value));
    }
}
