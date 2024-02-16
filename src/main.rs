//todo!("Bundle binary with pdfium-render libs for each platform");
use pdfium_render::prelude::*;

mod date;
pub use date::*;

fn main() -> Result<(), PdfiumError> {
    // todo!("Initialize pdfium-render");
    let pdfium = Pdfium::default();

    let mut document = pdfium.create_new_pdf()?;

    let mut page = document
        .pages_mut()
        .create_page_at_start(PdfPagePaperSize::a4())?;

    let mut dates = generate_date_strings(2024, 2);
    dates.into_iter().for_each(|x| println!("{:?}", x));

    // todo!("Generate display window");
    // todo!("Take user input");
    // todo!("Validate user input");
    // todo!("Generate NaiveDate for first of month");
    // todo!("Generate PDF primitives for OJT timesheet");
    // todo!("Fill primitives with data, form fields");
    // todo!("Export PDF");
    Ok(())
}

//todo!("Find prewritten software licensing form and package with executable");
//todo!("Sell to other companies and fuck over their SaaS for a quick buck");