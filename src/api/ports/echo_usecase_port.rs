pub trait EchoPort {
    fn echo(&self, message: &str);
}
