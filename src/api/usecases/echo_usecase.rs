use crate::api::ports::echo_usecase_port::EchoPort;
use crate::config::base::config::Config;
use tracing::info;

pub struct EchoUseCase {
    config: Config,
}

impl EchoUseCase {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl EchoPort for EchoUseCase {
    fn echo(&self, message: &str) {
        info!(
            "request received {:?}, and config {:?}",
            message, self.config
        );
    }
}
