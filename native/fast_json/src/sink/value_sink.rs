pub trait ValueSink {
    fn push_map(&mut self);
    fn push_array(&mut self);
    fn push_string(&mut self, string: String);
    fn push_integer(&mut self, integer: i64);
    fn push_float(&mut self, float: f64);
    fn push_bool(&mut self, boolean: bool);
    fn push_nil(&mut self);
    fn finalize_map(&mut self);
    fn finalize_array(&mut self);
    fn pop_insert_map(&mut self, key: String);
    fn pop_insert_array(&mut self);
}
