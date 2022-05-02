pub trait HashGenerator {
    fn generate(&mut self, message: &[u8]) -> String;

    fn generate_hex(&mut self, message: &[u8]) -> String;

    fn name(&self) -> &'static str;
}

impl HashGenerator for Box<dyn HashGenerator> {
    fn generate(&mut self, message: &[u8]) -> String {
        self.as_mut().generate(message)
    }

    fn generate_hex(&mut self, message: &[u8]) -> String {
        self.as_mut().generate_hex(message)
    }

    fn name(&self) -> &'static str {
        self.as_ref().name()
    }
}
