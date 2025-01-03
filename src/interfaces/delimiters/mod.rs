use crate::{
    error::{NenyrError, NenyrErrorKind},
    tokens::NenyrTokens,
    NenyrParser, NenyrResult,
};

/// # `NenyrParser` Delimiter Parsing Methods
///
/// This section of the `NenyrParser` focuses on utility methods designed to parse
/// various delimiters used in the Nenyr language, such as curly brackets (`{}`),
/// square brackets (`[]`), and colons (`:`). These methods are crucial for
/// ensuring that the input tokens conform to the expected syntax rules for the
/// language's structure.
///
/// Each parsing method aims to validate the presence and correctness of the specified
/// delimiters while providing flexible error handling and informative feedback.
/// In cases where the delimiters are missing or incorrectly parsed, detailed error
/// messages are generated, offering suggestions for corrections and aiding users
/// in troubleshooting syntax issues.
///
/// This group is currently focused on foundational delimiter parsing methods, with
/// plans to expand its functionality to include additional delimiter types in future
/// implementations.
impl NenyrParser {
    /// Implements parsing of a block enclosed in curly brackets (`{}`) and provides flexible error handling.
    ///
    /// This method attempts to parse an opening curly bracket (`{`), executes a provided parsing function
    /// for the contents inside the block, and then expects a closing curly bracket (`}`). If any of these
    /// conditions are not met, it returns a detailed syntax error.
    ///
    /// # Parameters
    ///
    /// * `suggestion_on_open`: An optional `String` that contains a suggestion in case of an error when
    ///   parsing the opening curly bracket (`{`). This is helpful for users to understand potential
    ///   solutions when the opening delimiter is missing or incorrect.
    ///
    /// * `error_message_on_open`: A `&str` that defines the error message to display if the opening curly bracket
    ///   is missing or incorrectly parsed. This message will be used to provide context to the error.
    ///
    /// * `suggestion_on_close`: An optional `String` containing a suggestion if the closing curly bracket (`}`)
    ///   is not found or incorrectly parsed. Similar to `suggestion_on_open`, this allows for better
    ///   feedback on potential syntax fixes when the closing delimiter is missing.
    ///
    /// * `error_message_on_close`: A `&str` that specifies the error message in case the closing curly bracket (`}`)
    ///   is not correctly found or parsed. This helps in pinpointing the issue during the parsing process.
    ///
    /// * `parse_fn`: A closure or function (`F`) that takes a mutable reference to the `NenyrParser` and returns
    ///   a `NenyrResult<T>`. This function is responsible for parsing the contents inside the curly brackets.
    ///   The `parse_fn` is called after a successful parsing of the opening curly bracket, and its result is
    ///   returned after the closing bracket is successfully parsed.
    ///
    /// # Returns
    ///
    /// Returns a `NenyrResult<T>`, where `T` is the type produced by `parse_fn`. If the opening or closing
    /// curly bracket is not correctly parsed, a `NenyrError` with appropriate context, suggestions, and error
    /// messages is returned instead.
    ///
    /// * On success, it returns `Ok(parsed_value)` where `parsed_value` is the result of the `parse_fn`.
    /// * On failure, it returns an `Err(NenyrError)` with error information regarding either the missing or
    ///   incorrect opening or closing delimiter.
    ///
    /// # Errors
    ///
    /// The function can return a `NenyrError` of kind `SyntaxError` in the following situations:
    ///
    /// * When the opening curly bracket (`{`) is not found, it returns an error with the message provided
    ///   in `error_message_on_open`, along with an optional suggestion from `suggestion_on_open`.
    ///
    /// * When the closing curly bracket (`}`) is not found, it returns an error with the message provided
    ///   in `error_message_on_close`, along with an optional suggestion from `suggestion_on_close`.
    pub(crate) fn parse_curly_bracketed_delimiter<F, T>(
        &mut self,
        suggestion_on_open: Option<String>,
        error_message_on_open: &str,
        suggestion_on_close: Option<String>,
        error_message_on_close: &str,
        mut parse_fn: F,
    ) -> NenyrResult<T>
    where
        F: FnMut(&mut Self) -> NenyrResult<T>,
    {
        // Checks if the current token is an opening curly bracket
        if let NenyrTokens::CurlyBracketOpen = self.current_token {
            // Processes the next token (inside the curly brackets)
            self.process_next_token()?;

            // Executes the provided parsing function
            let parsed_value = parse_fn(self)?;

            // Expects a closing curly bracket
            if let NenyrTokens::CurlyBracketClose = self.current_token {
                return Ok(parsed_value);
            }

            // Returns an error if the closing curly bracket is missing
            return Err(NenyrError::new(
                suggestion_on_close,
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(error_message_on_close),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        // Returns an error if the opening curly bracket is missing
        Err(NenyrError::new(
            suggestion_on_open,
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message_on_open),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Parses an expression that is enclosed within parentheses, ensuring both the
    /// opening and closing parentheses are present, and then executes the custom parsing
    /// logic provided.
    ///
    /// # Parameters
    /// - `suggestion_on_open`: An optional suggestion string to be shown in the error
    ///   message if the opening parenthesis is missing.
    /// - `error_message_on_open`: A string slice representing the error message to be
    ///   displayed when the opening parenthesis is not found.
    /// - `suggestion_on_close`: An optional suggestion string to be shown in the error
    ///   message if the closing parenthesis is missing.
    /// - `error_message_on_close`: A string slice representing the error message to be
    ///   displayed when the closing parenthesis is not found.
    /// - `parse_fn`: A function or closure that provides custom logic to parse the tokens
    ///   between the parentheses. This function is called once the opening parenthesis
    ///   is validated and before checking for the closing parenthesis.
    ///
    /// # Returns
    /// - `NenyrResult<T>`: The result of the parsing operation. If the parentheses are valid
    ///   and the `parse_fn` executes without error, the result of the `parse_fn` is returned.
    ///   Otherwise, an error is returned if the parentheses are not correctly opened or closed.
    ///
    /// # Errors
    /// This function returns a `NenyrError` in the following cases:
    /// - **Missing opening parenthesis**: If the current token is not an opening parenthesis,
    ///   an error is generated using `suggestion_on_open` and `error_message_on_open`.
    /// - **Missing closing parenthesis**: If the function encounters an opening parenthesis but
    ///   fails to find a closing parenthesis, an error is generated using `suggestion_on_close`
    ///   and `error_message_on_close`.
    pub(crate) fn parse_parenthesized_delimiter<F, T>(
        &mut self,
        suggestion_on_open: Option<String>,
        error_message_on_open: &str,
        suggestion_on_close: Option<String>,
        error_message_on_close: &str,
        mut parse_fn: F,
    ) -> NenyrResult<T>
    where
        F: FnMut(&mut Self) -> NenyrResult<T>,
    {
        // Checks if the current token is an opening parenthesis
        if let NenyrTokens::ParenthesisOpen = self.current_token {
            // Processes the next token (inside the parenthesis)
            self.process_next_token()?;

            // Executes the provided parsing function
            let parsed_value = parse_fn(self)?;

            // Expects a closing parenthesis
            if let NenyrTokens::ParenthesisClose = self.current_token {
                return Ok(parsed_value);
            }

            // Returns an error if the closing parenthesis is missing
            return Err(NenyrError::new(
                suggestion_on_close,
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(error_message_on_close),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        // Returns an error if the opening parenthesis is missing
        Err(NenyrError::new(
            suggestion_on_open,
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message_on_open),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Parses a colon (`:`) delimiter from the current token and optionally
    /// moves to the next token if `with_next_move` is true.
    ///
    /// # Parameters
    /// - `suggestion`: An optional string containing suggestions for fixing syntax errors if
    ///   the parsing fails. This will be included in the error if the colon is missing.
    /// - `error_message`: A string describing the error that will be included in the error
    ///   if the expected colon delimiter is not found.
    /// - `with_next_move`: A boolean flag that, if set to `true`, advances the parser to the next
    ///   token after successfully parsing the colon.
    ///
    /// # Returns
    /// - `Ok(())` if the colon is successfully parsed.
    /// - `Err(NenyrError)` if the colon is missing or invalid.
    ///
    /// # Errors
    /// An error of type `SyntaxError` is returned if the current token is not a colon (`:`).
    /// This error includes the provided `error_message` and any optional `suggestion`.
    pub(crate) fn parse_colon_delimiter(
        &mut self,
        suggestion: Option<String>,
        error_message: &str,
        with_next_move: bool,
    ) -> NenyrResult<()> {
        // Checks if the current token is a colon.
        if let NenyrTokens::Colon = self.current_token {
            if with_next_move {
                self.process_next_token()?;
            }

            return Ok(());
        }

        // Returns an error if the colon is missing
        Err(NenyrError::new(
            suggestion,
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }

    /// Parses content enclosed within square brackets (`[ ... ]`) using a custom parsing function.
    ///
    /// This method is used to validate and process syntax enclosed within square brackets by:
    /// 1. Checking for an opening square bracket (`[`).
    /// 2. Applying a provided parsing function (`parse_fn`) to process the enclosed content.
    /// 3. Verifying the presence of a closing square bracket (`]`).
    ///
    /// If the syntax is correct and square brackets are properly closed, the method returns the parsed result.
    /// Otherwise, it generates a `NenyrError` with custom messages depending on whether the opening or closing bracket is missing.
    ///
    /// # Type Parameters
    /// - `F`: A mutable function or closure that implements the parsing logic on the enclosed content.
    /// - `T`: The type expected as the return from the parsing function (`parse_fn`), indicating the parsed result.
    ///
    /// # Parameters
    /// - `suggestion_on_open`: An optional `String` that provides a suggestion message if the opening square bracket (`[`) is missing.
    /// - `error_message_on_open`: A `&str` containing the error message to display if the opening square bracket (`[`) is missing.
    /// - `suggestion_on_close`: An optional `String` offering a suggestion if the closing square bracket (`]`) is not found.
    /// - `error_message_on_close`: A `&str` containing the error message to display if the closing square bracket (`]`) is missing.
    /// - `parse_fn`: A function or closure (`F`) that operates on the parser, defining how to process the enclosed content within square brackets.
    ///
    /// # Returns
    /// - `NenyrResult<T>`: Returns `Ok(T)` if parsing completes successfully within the brackets, where `T` is the parsed value's type.
    /// - `Err(NenyrError)`: Returns a detailed error if either the opening or closing square bracket is missing, including:
    ///     - Context information (like `context_name` and `context_path`).
    ///     - Custom error messages (`error_message_on_open` or `error_message_on_close`).
    ///     - An optional suggestion to guide corrective action (`suggestion_on_open` or `suggestion_on_close`).
    pub(crate) fn parse_square_bracketed_delimiter<F, T>(
        &mut self,
        suggestion_on_open: Option<String>,
        error_message_on_open: &str,
        suggestion_on_close: Option<String>,
        error_message_on_close: &str,
        mut parse_fn: F,
    ) -> NenyrResult<T>
    where
        F: FnMut(&mut Self) -> NenyrResult<T>,
    {
        // Checks if the current token is an opening square bracket
        if let NenyrTokens::SquareBracketOpen = self.current_token {
            // Processes the next token (inside the square brackets)
            self.process_next_token()?;

            // Executes the provided parsing function
            let parsed_value = parse_fn(self)?;

            // Expects a closing square bracket
            if let NenyrTokens::SquareBracketClose = self.current_token {
                return Ok(parsed_value);
            }

            // Returns an error if the closing square bracket is missing
            return Err(NenyrError::new(
                suggestion_on_close,
                self.context_name.clone(),
                self.context_path.to_string(),
                self.add_nenyr_token_to_error(error_message_on_close),
                NenyrErrorKind::SyntaxError,
                self.get_tracing(),
            ));
        }

        // Returns an error if the opening square bracket is missing
        Err(NenyrError::new(
            suggestion_on_open,
            self.context_name.clone(),
            self.context_path.to_string(),
            self.add_nenyr_token_to_error(error_message_on_open),
            NenyrErrorKind::SyntaxError,
            self.get_tracing(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::NenyrParser;

    #[test]
    fn bracketed_section_is_valid() {
        let raw_nenyr = "{ }";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            parser.parse_curly_bracketed_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }

    #[test]
    fn bracketed_section_missing_opening_curly_bracket() {
        let raw_nenyr = "}";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_curly_bracketed_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }

    #[test]
    fn bracketed_section_missing_closing_curly_bracket() {
        let raw_nenyr = "{";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_curly_bracketed_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }

    #[test]
    fn parenthesized_section_is_valid() {
        let raw_nenyr = "( )";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            parser.parse_parenthesized_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }

    #[test]
    fn parenthesized_section_missing_opening_parenthesis() {
        let raw_nenyr = ")";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_parenthesized_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }

    #[test]
    fn parenthesized_section_missing_closing_parenthesis() {
        let raw_nenyr = "(";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_parenthesized_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }

    #[test]
    fn colon_is_valid() {
        let raw_nenyr = ":";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(parser.parse_colon_delimiter(None, "", false), Ok(()));
    }

    #[test]
    fn colon_is_not_valid() {
        let raw_nenyr = ";";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(parser.parse_colon_delimiter(None, "", false), Ok(()));
    }

    #[test]
    fn squared_section_is_valid() {
        let raw_nenyr = "[] ]";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_eq!(
            parser.parse_square_bracketed_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }

    #[test]
    fn squared_section_missing_opening_square_bracket() {
        let raw_nenyr = "]";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_square_bracketed_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }

    #[test]
    fn squared_section_missing_closing_square_bracket() {
        let raw_nenyr = "[";
        let mut parser = NenyrParser::new();

        parser.setup_dependencies(raw_nenyr.to_string(), "".to_string());

        let _ = parser.process_next_token();
        assert_ne!(
            parser.parse_square_bracketed_delimiter(None, "", None, "", |_| Ok(())),
            Ok(())
        );
    }
}
