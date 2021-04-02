use crate::common::{PropertyId, ServicePropertyChannel};
use crate::error::Result;
use crate::ffi::SPXSPEECHCONFIGHANDLE;
use crate::speech::SpeechConfig;

pub trait DialogServiceConfig {
    fn get_speech_config(&mut self) -> &mut SpeechConfig;
    fn get_handle(&self) -> SPXSPEECHCONFIGHANDLE;

    fn set_property(&mut self, id: PropertyId, value: String) -> Result<()> {
        self.get_speech_config().set_property(id, value)
    }

    fn get_property(&mut self, id: PropertyId) -> Result<String> {
        self.get_speech_config().get_property(id)
    }

    fn set_property_by_string(&mut self, name: String, value: String) -> Result<()> {
        self.get_speech_config().set_property_by_string(name, value)
    }

    fn get_property_by_string(&mut self, name: String) -> Result<String> {
        self.get_speech_config().get_property_by_string(name)
    }

    fn set_service_property(
        &mut self,
        name: String,
        value: String,
        channel: ServicePropertyChannel,
    ) -> Result<()> {
        self.get_speech_config()
            .set_service_property(name, value, channel)
    }

    fn set_proxy(&mut self, hostname: String, port: u64) -> Result<()> {
        self.get_speech_config().set_proxy(hostname, port)
    }

    fn set_proxy_with_usrname_and_pwd(
        &mut self,
        hostname: String,
        port: u64,
        username: String,
        password: String,
    ) -> Result<()> {
        self.get_speech_config()
            .set_proxy_with_usrname_and_pwd(hostname, port, username, password)
    }

    fn set_language(&mut self, lang: String) -> Result<()> {
        self.get_speech_config()
            .set_property(PropertyId::SpeechServiceConnectionRecoLanguage, lang)
    }

    fn get_language(&mut self) -> Result<String> {
        self.get_speech_config()
            .get_property(PropertyId::SpeechServiceConnectionRecoLanguage)
    }
}
