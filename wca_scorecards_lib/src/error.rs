use scorecard_to_pdf::PdfError;

#[derive(Debug)]
pub enum Error {
    PdfError(PdfError),
    CsvError,
}

impl From<PdfError> for Error {
    fn from(value: PdfError) -> Self {
        Error::PdfError(value)
    }
}
