use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, World};

pub struct InMemoryWorld {
    source: Source,
    fonts: Vec<Font>,
    book: LazyHash<FontBook>,
    library: LazyHash<Library>,
}

impl InMemoryWorld {
    pub fn new(text: impl Into<String>, fonts: Vec<Font>) -> Self {
        let source = Source::detached(text);
        let book = FontBook::from_fonts(&fonts);

        Self {
            source,
            fonts,
            book: LazyHash::new(book),
            library: LazyHash::new(Library::default()),
        }
    }
}

impl World for InMemoryWorld {
    fn main(&self) -> FileId {
        self.source.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::AccessDenied)
        }
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::AccessDenied)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let offset = offset.unwrap_or(0);
        let offset = time::UtcOffset::from_hms(offset as i8, 0, 0).ok()?;
        let now = time::OffsetDateTime::now_utc().checked_to_offset(offset)?;
        Some(Datetime::Date(now.date()))
    }

    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }
}
