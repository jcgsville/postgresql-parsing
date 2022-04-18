#[derive(Debug, PartialEq)]
pub enum ParseCommandSectionResult<TParsedData> {
    Valid(usize, TParsedData),
    Invalid,
    EndOfInput,
}

macro_rules! parse_section {
    ($parse:ident, $tokens:ident, $start_idx:ident) => {
        match $parse($tokens, $start_idx) {
            crate::parser::commands::parse_section::ParseCommandSectionResult::Valid(
                idx_after_section,
                parsed_data,
            ) => (idx_after_section, parsed_data),
            crate::parser::commands::parse_section::ParseCommandSectionResult::Invalid => {
                return crate::parser::parse_command_result::ParseCommandResult::Invalid(
                    crate::parser::utils::skip_invalid_command($tokens, $start_idx),
                );
            }
            crate::parser::commands::parse_section::ParseCommandSectionResult::EndOfInput => {
                return crate::parser::parse_command_result::ParseCommandResult::EndOfInput
            }
        }
    };
}

macro_rules! parse_section_from_section {
    ($parse_result:expr) => {
        match $parse_result {
            crate::parser::commands::parse_section::ParseCommandSectionResult::Valid(
                idx_after_section,
                parsed_data,
            ) => (idx_after_section, parsed_data),
            crate::parser::commands::parse_section::ParseCommandSectionResult::Invalid => {
                return crate::parser::commands::parse_section::ParseCommandSectionResult::Invalid;
            }
            crate::parser::commands::parse_section::ParseCommandSectionResult::EndOfInput => {
                return crate::parser::commands::parse_section::ParseCommandSectionResult::EndOfInput;
            }
        }
    };
}

pub(crate) use parse_section;
pub(crate) use parse_section_from_section;
