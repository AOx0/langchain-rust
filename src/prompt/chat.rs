use std::error::Error;

use crate::schemas::{messages::Message, prompt::PromptValue};

use super::{FormatPrompter, MessageFormatter, PromptArgs, PromptFromatter, PromptTemplate};

/// A template for creating human-readable messages.

#[derive(Clone)]
pub struct HumanMessagePromptTemplate {
    prompt: PromptTemplate,
}

impl Into<Box<dyn MessageFormatter>> for HumanMessagePromptTemplate {
    fn into(self) -> Box<dyn MessageFormatter> {
        Box::new(self)
    }
}

impl HumanMessagePromptTemplate {
    pub fn new(prompt: PromptTemplate) -> Self {
        Self { prompt }
    }
}
impl MessageFormatter for HumanMessagePromptTemplate {
    fn format_messages(&self, input_variables: PromptArgs) -> Result<Vec<Message>, Box<dyn Error>> {
        let message = Message::new_human_message(&self.prompt.format(input_variables)?);
        log::debug!("message: {:?}", message);
        Ok(vec![message])
    }
    fn input_variables(&self) -> Vec<String> {
        self.prompt.variables().clone()
    }
}

impl FormatPrompter for HumanMessagePromptTemplate {
    fn format_prompt(&self, input_variables: PromptArgs) -> Result<PromptValue, Box<dyn Error>> {
        let messages = self.format_messages(input_variables)?;
        Ok(PromptValue::from_messages(messages))
    }
    fn get_input_variables(&self) -> Vec<String> {
        self.input_variables()
    }
}

/// A template for creating system messages.
#[derive(Clone)]
pub struct SystemMessagePromptTemplate {
    prompt: PromptTemplate,
}

impl Into<Box<dyn MessageFormatter>> for SystemMessagePromptTemplate {
    fn into(self) -> Box<dyn MessageFormatter> {
        Box::new(self)
    }
}

impl SystemMessagePromptTemplate {
    pub fn new(prompt: PromptTemplate) -> Self {
        Self { prompt }
    }
}

impl FormatPrompter for SystemMessagePromptTemplate {
    fn format_prompt(&self, input_variables: PromptArgs) -> Result<PromptValue, Box<dyn Error>> {
        let messages = self.format_messages(input_variables)?;
        Ok(PromptValue::from_messages(messages))
    }
    fn get_input_variables(&self) -> Vec<String> {
        self.input_variables()
    }
}

impl MessageFormatter for SystemMessagePromptTemplate {
    fn format_messages(&self, input_variables: PromptArgs) -> Result<Vec<Message>, Box<dyn Error>> {
        let message = Message::new_system_message(&self.prompt.format(input_variables)?);
        log::debug!("message: {:?}", message);
        Ok(vec![message])
    }
    fn input_variables(&self) -> Vec<String> {
        self.prompt.variables().clone()
    }
}

/// A template for creating AI (assistant) messages.
#[derive(Clone)]
pub struct AIMessagePromptTemplate {
    prompt: PromptTemplate,
}

impl Into<Box<dyn MessageFormatter>> for AIMessagePromptTemplate {
    fn into(self) -> Box<dyn MessageFormatter> {
        Box::new(self)
    }
}

impl FormatPrompter for AIMessagePromptTemplate {
    fn format_prompt(&self, input_variables: PromptArgs) -> Result<PromptValue, Box<dyn Error>> {
        let messages = self.format_messages(input_variables)?;
        Ok(PromptValue::from_messages(messages))
    }
    fn get_input_variables(&self) -> Vec<String> {
        self.input_variables()
    }
}

impl MessageFormatter for AIMessagePromptTemplate {
    fn format_messages(&self, input_variables: PromptArgs) -> Result<Vec<Message>, Box<dyn Error>> {
        let message = Message::new_ai_message(&self.prompt.format(input_variables)?);
        log::debug!("message: {:?}", message);
        Ok(vec![message])
    }
    fn input_variables(&self) -> Vec<String> {
        self.prompt.variables().clone()
    }
}

impl AIMessagePromptTemplate {
    pub fn new(prompt: PromptTemplate) -> Self {
        Self { prompt }
    }
}

pub enum MessageOrTemplate {
    Message(Message),
    Template(Box<dyn MessageFormatter>),
    MessagesPlaceholder(String),
}

// Macro for formatting a `Message` variant for the formatter
#[macro_export]
macro_rules! fmt_message {
    ($msg:expr) => {
        $crate::prompt::MessageOrTemplate::Message($msg)
    };
}

// Macro for formatting a `Template` variant for the formatter
#[macro_export]
macro_rules! fmt_template {
    ($template:expr) => {
        $crate::prompt::MessageOrTemplate::Template(Box::new($template))
    };
}

// Macro for formatting a `MessagesPlaceholder` variant for the formatter
#[macro_export]
macro_rules! fmt_placeholder {
    ($placeholder:expr) => {
        $crate::prompt::MessageOrTemplate::MessagesPlaceholder($placeholder.into())
    };
}

pub struct MessageFormatterStruct {
    items: Vec<MessageOrTemplate>,
}

impl MessageFormatterStruct {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_message(&mut self, message: Message) {
        self.items.push(MessageOrTemplate::Message(message));
    }

    pub fn add_template(&mut self, template: Box<dyn MessageFormatter>) {
        self.items.push(MessageOrTemplate::Template(template));
    }

    pub fn add_messages_placeholder(&mut self, placeholder: &str) {
        self.items.push(MessageOrTemplate::MessagesPlaceholder(
            placeholder.to_string(),
        ));
    }

    fn format(&self, input_variables: PromptArgs) -> Result<Vec<Message>, Box<dyn Error>> {
        let mut result: Vec<Message> = Vec::new();
        for item in &self.items {
            match item {
                MessageOrTemplate::Message(msg) => result.push(msg.clone()),
                MessageOrTemplate::Template(tmpl) => {
                    result.extend(tmpl.format_messages(input_variables.clone())?)
                }
                MessageOrTemplate::MessagesPlaceholder(placeholder) => {
                    let messages = input_variables[placeholder].clone();
                    result.extend(Message::messages_from_value(&messages)?);
                }
            }
        }
        Ok(result)
    }
}

impl MessageFormatter for MessageFormatterStruct {
    fn format_messages(&self, input_variables: PromptArgs) -> Result<Vec<Message>, Box<dyn Error>> {
        self.format(input_variables)
    }
    fn input_variables(&self) -> Vec<String> {
        let mut variables = Vec::new();
        for item in &self.items {
            match item {
                MessageOrTemplate::Message(_) => {}
                MessageOrTemplate::Template(tmpl) => {
                    variables.extend(tmpl.input_variables());
                }
                MessageOrTemplate::MessagesPlaceholder(placeholder) => {
                    variables.extend(vec![placeholder.clone()]);
                }
            }
        }
        variables
    }
}

impl FormatPrompter for MessageFormatterStruct {
    fn format_prompt(&self, input_variables: PromptArgs) -> Result<PromptValue, Box<dyn Error>> {
        let messages = self.format(input_variables)?;
        Ok(PromptValue::from_messages(messages))
    }
    fn get_input_variables(&self) -> Vec<String> {
        self.input_variables()
    }
}

#[macro_export]
macro_rules! message_formatter {
($($item:expr),* $(,)?) => {{
    let mut formatter = $crate::prompt::MessageFormatterStruct::new();
    $(
        match $item {
            $crate::prompt::MessageOrTemplate::Message(msg) => formatter.add_message(msg),
            $crate::prompt::MessageOrTemplate::Template(tmpl) => formatter.add_template(tmpl),
            $crate::prompt::MessageOrTemplate::MessagesPlaceholder(placeholder) => formatter.add_messages_placeholder(&placeholder.clone()),
        }
    )*
    formatter
}};
}

#[cfg(test)]
mod tests {
    use crate::{
        message_formatter,
        prompt::{chat::AIMessagePromptTemplate, FormatPrompter},
        prompt_args,
        schemas::messages::Message,
        template_fstring,
    };

    #[test]
    fn test_message_formatter_macro() {
        // Create a human message and system message
        let human_msg = Message::new_human_message("Hello from user");

        // Create an AI message prompt template
        let ai_message_prompt = AIMessagePromptTemplate::new(template_fstring!(
            "AI response: {content} {test}",
            "content",
            "test"
        ));

        // Use the `message_formatter` macro to construct the formatter
        let formatter = message_formatter![
            fmt_message!(human_msg),
            fmt_template!(ai_message_prompt),
            fmt_placeholder!("history")
        ];

        // Define input variables for the AI message template
        let input_variables = prompt_args! {
            "content" => "This is a test",
            "test" => "test2",
            "history" => vec![
                Message::new_human_message("Placeholder message 1"),
                Message::new_ai_message("Placeholder message 2"),
            ],


        };

        // Format messages
        let formatted_messages = formatter
            .format_prompt(input_variables)
            .unwrap()
            .to_chat_messages();

        // Verify the number of messages
        assert_eq!(formatted_messages.len(), 4);

        // Verify the content of each message
        assert_eq!(formatted_messages[0].content, "Hello from user");
        assert_eq!(
            formatted_messages[1].content,
            "AI response: This is a test test2"
        );
        assert_eq!(formatted_messages[2].content, "Placeholder message 1");
        assert_eq!(formatted_messages[3].content, "Placeholder message 2");
    }
}
