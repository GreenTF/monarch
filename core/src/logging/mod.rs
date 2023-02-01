mod client;

use std::error::Error;

use self::client::enable_text_msg_hook;

pub fn init_logging_hooks() -> Result<(), Box<dyn Error>> {
    dll_callback!("client.dll", "TextMsg", enable_text_msg_hook);
    Ok(())
}
