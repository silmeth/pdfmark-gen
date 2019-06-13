use encoding::codec::utf_16::UTF_16BE_ENCODING;
use encoding::types::Encoding;
use encoding::EncoderTrap;
use serde::Deserialize;
use std::env;
use std::fmt::Write;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
struct PdfMarkData {
    title: String,
    authors: Vec<String>,
    toc: Vec<TocEntry>,
}

impl PdfMarkData {
    fn generate_pdfmark_string(&self) -> String {
        let mut output = String::new();
        self.write_pdfmark(&mut output);
        output
    }

    fn write_pdfmark<W: Write>(&self, w: &mut W) {
        let title = to_utf16_bom_string(&self.title);
        let authors = to_utf16_bom_string(&self.authors.join(", "));

        w.write_str(&format!(
            "[ /Title {} /Author {} /DOCINFO pdfmark\n",
            title, authors
        ))
        .unwrap();
        for toc_entry in &self.toc {
            toc_entry.write_pdfmark(w);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize)]
struct TocEntry {
    title: String,
    page: u32,
    #[serde(default = "Vec::new")]
    children: Vec<TocEntry>,
}

impl TocEntry {
    fn write_pdfmark<W: Write>(&self, w: &mut W) {
        let title = to_utf16_bom_string(&self.title);
        let children_cnt = self.children.len();

        if children_cnt > 0 {
            w.write_str(&format!(
                "[ /Title {} /Page {} /Count {} /OUT pdfmark\n",
                title, self.page, children_cnt
            ))
            .unwrap();

            for child in &self.children {
                child.write_pdfmark(w);
            }
        } else {
            w.write_str(&format!(
                "[ /Title {} /Page {} /OUT pdfmark\n",
                title, self.page
            ))
            .unwrap();
        }
    }
}

fn to_utf16_bom_string(str: &str) -> String {
    let u16bytes = UTF_16BE_ENCODING.encode(str, EncoderTrap::Strict).unwrap();
    let u16str = dbg!(u16bytes
        .into_iter()
        .map(|byte| format!("{:02X?}", byte))
        .collect::<String>());
    format!("<FEFF{}>", u16str)
}

fn main() {
    let mut args = env::args();
    let data_path = args.nth(1).expect("Expected path to input file");
    let output_path = args.next().expect("Expected output path");

    let data = fs::read_to_string(data_path).expect("Could not read input file");
    let data: PdfMarkData = serde_json::from_str(&data).expect("Could not parse input file");

    dbg!(&data);

    let output = &dbg!(data.generate_pdfmark_string());

    fs::write(output_path, output).expect("Could not write to output file");
}
