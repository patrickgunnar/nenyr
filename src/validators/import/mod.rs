use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();
}

/// A trait responsible for validating the import of external CSS styles.
///
/// The `NenyrImportValidator` trait provides methods to ensure that the
/// CSS imports in a project are valid. It checks various conditions to
/// determine the validity of an import, including:
///
/// - Whether the import string is empty.
/// - Whether the import string is a valid URL (either HTTP or FTP).
/// - Whether the import points to a file that exists in the specified project
///   path.
///
/// # Methods
///
/// - `is_valid_import(&self, import: &str, context_path: &str) -> bool`:
///   This method performs the validation checks for the given import string.
///   
/// # Parameters
///
/// - `import`: A string slice that represents the import path or URL. This
///   can be a relative path to a CSS file in the project or a full URL to
///   an external CSS resource.
/// - `context_path`: A string slice that represents the path to the context
///   file from which the import is being validated. This is used to resolve
///   relative paths against the file system.
///
/// # Returns
///
/// Returns `true` if the import is valid according to the specified rules;
/// otherwise, it returns `false`.
pub trait NenyrImportValidator {
    fn is_valid_import(&self, import: &str) -> bool {
        if import.is_empty() {
            return false;
        }

        if URL_REGEX.is_match(import) {
            return true;
        }

        let import_path = Path::new(import);

        if import_path.is_absolute() || import_path.parent().is_some() {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::NenyrImportValidator;

    struct Import {}

    impl Import {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl NenyrImportValidator for Import {}

    #[test]
    fn all_imports_are_valid() {
        let import = Import::new();
        let external_paths = vec![
            "../../../mocks/imports/another_external.css",
            "../../../mocks/imports/external_styles.css",
            "../../../mocks/imports/styles.css",
            "https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap",
            "https://fonts.googleapis.com/css2?family=Afacad+Flux:wght@100..1000&display=swap",
            "https://fonts.googleapis.com/css2?family=Sixtyfour+Convergence&display=swap"
        ];

        for external_path in external_paths {
            assert!(import.is_valid_import(external_path));
        }
    }

    #[test]
    fn all_imports_are_not_valid() {
        let import = Import::new();
        let external_paths = vec![
            "../../mocks/imports/another_external.css",
            "../../mocks/imports/external_styles.css",
            "../../mocks/imports/styles.css",
            "htt://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap",
            "htt://fonts.googleapis.com/css2?family=Afacad+Flux:wght@100..1000&display=swap",
            "htt://fonts.googleapis.com/css2?family=Sixtyfour+Convergence&display=swap"
        ];

        for external_path in external_paths {
            assert!(!import.is_valid_import(external_path));
        }
    }

    #[test]
    fn test_invalid_relative_paths() {
        let import = Import::new();

        let invalid_paths = vec![
            "../../../mocks/imports/nonexistent_file.css",
            "nonexistent_dir/another_external.css",
            "../../../../../mocks/imports/styles.css",
            "invalid_path_with_space .css",
        ];

        for external_path in invalid_paths {
            assert!(!import.is_valid_import(&external_path));
        }
    }

    #[test]
    fn test_empty_import_path() {
        let import = Import::new();

        assert!(!import.is_valid_import(""));
    }

    #[test]
    fn test_http_import() {
        let import = Import::new();

        // Testa um caminho http válido
        assert!(import.is_valid_import("http://example.com/styles.css"));
    }
}
