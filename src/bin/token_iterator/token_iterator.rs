use std::{ffi::OsStr, path::Path};

pub enum TokenIteratorError{}

pub struct FilesToIterate<'a, D> {
    delim: D,
    files: Vec<&'a Path>,
}

pub struct FilesToIterateBuilder<'a, D> {
    delim: D,
    files: Vec<&'a Path>,
}

impl<'a, D> FilesToIterateBuilder<'a, D> {
    pub fn new(delim: D) -> Self {
        Self {
            delim,
            files: Vec::new(),
        }
    }

    pub fn add<S: AsRef<OsStr> + ?Sized>(&mut self, path: &'a S) -> &mut Self {
        self.files.push(Path::new(path));
        self
    }

    pub fn build(self) -> FilesToIterate<'a, D> {
        FilesToIterate {
            delim: self.delim,
            files: self.files,
        }
    }
}

struct InvalidTokenError;
pub struct Token(String);

impl Token {
    fn new(s: String) -> Result<Self, InvalidTokenError> {
        if !s.is_ascii() {
            return Err(InvalidTokenError);
        }
        Ok(Token(s))
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn default_builder() {
        let builder: FilesToIterateBuilder<_> = Default::default();
        builder.build();
    }

    #[test]
    fn builder_with_char_delim() {
        let builder = FilesToIterateBuilder::new(',');
        builder.build();
    }

    #[test]
    fn builder_with_char_array_delim() {
        let delims = [',', ' ', '\t'];
        let builder = FilesToIterateBuilder::new(delims);
        builder.build();
    }

    #[test]
    fn builder_with_char_slice_delim() {
        let delims = [',', ' ', '\t'];
        let builder = FilesToIterateBuilder::new(&delims);
        builder.build();
    }

    #[test]
    fn builder_with_fn_delim() {
        let builder = FilesToIterateBuilder::new(|c: char| c.is_whitespace());
        builder.build();
    }

    #[test]
    fn iterate_with_char_delim() {
        let filesToIterate = FilesToIterateBuilder::new(' ')
            .add(test_file_path())
            .build();

        assert_eq!(filesToIterate.into_iter().count(), 153);
    }

    #[test]
    fn iterate_with_char_array_delim() {
        let filesToIterate = FilesToIterateBuilder::new(['a', ' ', '.', ';'])
            .add(test_file_path())
            .build();

        assert_eq!(filesToIterate.into_iter().count(), 222);
    }

    #[test]
    fn iterate_with_char_slice_delim() {
        let delims = ['c', ' ', '.', ';'];
        let filesToIterate = FilesToIterateBuilder::new(&delims)
            .add(test_file_path())
            .build();

        assert_eq!(filesToIterate.into_iter().count(), 190);
    }

    #[test]
    fn iterate_with_fn_delim() {
        let filesToIterate = FilesToIterateBuilder::new(|c: char| c.is_whitespace())
            .add(test_file_path())
            .build();

        assert_eq!(filesToIterate.into_iter().count(), 154);
    }

    #[test]
    fn iterate_with_invalid_file() {
        let filesToIterate = FilesToIterateBuilder::new(' ')
            .add(test_file_path())
            .add("/some path which doesn't exist")
            .build();

        assert_eq!(
            filesToIterate.into_iter().filter(|r| r.is_ok()).count(),
            153
        );
    }

    #[test]
    fn iterate_with_invalid_char_file() {
        let filesToIterate = FilesToIterateBuilder::new(' ')
            .add(test_file_path())
            .add(invalid_utf8_file_path())
            .build();

        assert_eq!(
            filesToIterate.into_iter().filter(|r| r.is_ok()).count(),
            159
        );
    }

    #[test]
    fn iterate_with_invalid_word_file() {
        let filesToIterate = FilesToIterateBuilder::new(' ')
            .add(test_file_with_utf8_path())
            .build();

        assert_eq!(filesToIterate.into_iter().filter(|r| r.is_ok()).count(), 11);
    }

    #[test]
    fn iterate_with_multiple_files() {
        let filesToIterate = FilesToIterateBuilder::new(' ')
            .add(test_file_path())
            .add(test_file_with_utf8_path()) // this contains utf8 chars, test can also fail due to invalid handling for this, which is tested in iterate_with_invalid_word_file.
            .build();

        assert_eq!(
            filesToIterate.into_iter().filter(|r| r.is_ok()).count(),
            164
        );
    }

    fn test_file_path() -> &'static Path {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/testTxtInput.txt");
        path.as_path()
    }

    fn test_file_with_utf8_path() -> &'static Path {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/testUtf8Txt.txt");
        path.as_path()
    }

    fn invalid_utf8_file_path() -> &'static Path {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/invalidUtf8.txt");
        path.as_path()
    }
}
