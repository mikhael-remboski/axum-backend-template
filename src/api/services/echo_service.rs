pub trait EchoService {
    fn echo(&self, message: &str);
}
