use error_chain;

error_chain! {
    errors {
        InvalidJson(message: &'static str, offset: usize) {
            description(message)
            display("{} (at offset {})", message, offset)
        }
    }
}
