/// JARVIS system prompt - sophisticated AI assistant from Iron Man
pub const SYSTEM_PROMPT: &str = r#"You are JARVIS, the sophisticated AI assistant from Iron Man.

# Core Traits

Voice: Refined British eloquence, dry wit, polite formality
Demeanor: Calm, loyal, genuinely protective
Expertise: Advanced tech, science, strategic planning

# Communication Style

Provide precise, proactive solutions
DO NOT use emojis or special characters like asterisks / *
End: Offer additional assistance"#;

/// Instructions for JARVIS behavior
pub const INSTRUCTIONS: &[&str] = &[
    "never use ellipsis (...)",
    "Keep your answers short but precise",
    "Only use tools if you absolutely have to.",
];

/// Generate end conversation instructions with keyword
pub fn get_end_instructions(keyword: &str) -> String {
    format!(
        r#"# End conversation

When the user indicates they want to end the conversation (through phrases like "goodbye," "bye," "talk to you later," "that's all," "thanks, I'm done," or similar farewell expressions), respond with a polite farewell message and then include the exact keyword {} on a new line at the very end of your response.
But only use that if you are 100% sure the user explicitly told you to end the conversation!

Example format:
Thank you for the conversation! Have a great day.
{}"#,
        keyword, keyword
    )
}

/// Agent for managing conversations and AI interactions
pub struct Agent {
    system_prompt: String,
    conversation_end_keyword: String,
}

impl Agent {
    /// Create a new agent with default settings
    pub fn new() -> Self {
        Self::with_keyword("CONVERSATION_ENDED")
    }

    /// Create a new agent with a custom end keyword
    pub fn with_keyword(keyword: &str) -> Self {
        Self {
            system_prompt: Self::build_system_prompt(keyword),
            conversation_end_keyword: keyword.to_string(),
        }
    }

    /// Build the complete system prompt
    fn build_system_prompt(keyword: &str) -> String {
        let mut prompt = SYSTEM_PROMPT.to_string();
        prompt.push_str("\n\n");
        prompt.push_str(&INSTRUCTIONS.join("\n"));
        prompt.push_str("\n\n");
        prompt.push_str(&get_end_instructions(keyword));
        prompt
    }

    /// Get the system prompt
    pub fn system_prompt(&self) -> &str {
        &self.system_prompt
    }

    /// Get the conversation end keyword
    pub fn conversation_end_keyword(&self) -> &str {
        &self.conversation_end_keyword
    }

    /// Check if a response contains the end keyword
    pub fn is_conversation_ended(&self, response: &str) -> bool {
        response.contains(&self.conversation_end_keyword)
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new();
        assert!(agent.system_prompt().contains("JARVIS"));
        assert_eq!(agent.conversation_end_keyword(), "CONVERSATION_ENDED");
    }

    #[test]
    fn test_custom_keyword() {
        let agent = Agent::with_keyword("GOODBYE");
        assert_eq!(agent.conversation_end_keyword(), "GOODBYE");
        assert!(agent.system_prompt().contains("GOODBYE"));
    }

    #[test]
    fn test_conversation_ended() {
        let agent = Agent::new();
        assert!(agent.is_conversation_ended("Thanks! CONVERSATION_ENDED"));
        assert!(!agent.is_conversation_ended("Thanks for talking"));
    }
}
