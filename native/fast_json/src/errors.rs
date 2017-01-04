error_chain! {
    errors {
        InvalidJson(message: String, offset: usize) {
            description(message)
            display("{} at position {}", message, offset)
        }
    }
}
