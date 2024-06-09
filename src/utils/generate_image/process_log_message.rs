pub fn process_log_message(messages: Vec<String>) -> String {
    // println!("messages={:?}",messages);
    let message = match messages
        .into_iter()
        .filter(|entry| { entry.contains("Program log:")})
        .collect::<Vec<String>>()
        .last()
    {
        Some(message) => format!("{}\"", extract_error_message(message)),
        None => "null\"".to_string(),
    };

    message.to_string()
}

fn extract_error_message(log: &str) -> String {
    // Search for the start of the message marker
    let message_marker = "Message:";
    if let Some(start) = log.find(message_marker) {
        // Calculate the start index of the actual message
        let start_of_message = start + message_marker.len();

        // Extract the substring from this index to the end of the log string
        let message = log[start_of_message..].trim(); // Trim to remove any leading whitespace

        return message.to_string();
    }

    "null".to_string()
}
