use crate::api::services::echo_service::EchoService;
use crate::config::base::config::Config;
use tracing::info;

pub struct EchoServiceImpl {
    config: Config,
}

impl EchoServiceImpl {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl EchoService for EchoServiceImpl {
    fn echo(&self, message: &str) {
        info!(
            "request received {:?}, and config {:?}",
            message, self.config
        );
    }
}
