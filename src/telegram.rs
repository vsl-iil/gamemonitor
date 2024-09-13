use frankenstein::{Api, Message, MethodResponse, SendMessageParams, TelegramApi};

pub struct TelegramSender {
    api: Api,
    chat_id: i64
}

impl TelegramSender {
    pub fn new(api_key: &str, chat_id: i64) -> Self {
        TelegramSender { api: Api::new(api_key), chat_id }
    }

    pub fn send_message(&self, message: String) 
    -> Result<MethodResponse<Message>, frankenstein::Error> 
    {
        let send_params_builder = SendMessageParams::builder()
                          .chat_id(self.chat_id)
                          .text(message);

        let send_params: SendMessageParams;
        // if let Some(markup) = reply_markup {
        send_params = send_params_builder.build();

        self.api.send_message(&send_params)
    }
}
