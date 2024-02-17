use pdfium_render::prelude::*;
use std::error::Error;

pub fn generate_pdf(pdfium: &Pdfium, year:i32, month:u32, coordinator: &String) -> Result<(), Box<dyn Error>> {
    let mut document = pdfium.create_new_pdf()?;

    // US Letter paper has PdfPoints width 612, height 792.
    let mut page = document
        .pages_mut()
        .create_page_at_start(PdfPagePaperSize::Portrait(PdfPagePaperStandardSize::USLetterAnsiA))?;

    let page_width = page.width();
    let page_height = page.height();

    // Begin Chart

    let chart_num_rows = 33;
    let chart_interior_left = 51.5;
    let chart_interior_bottom = 211.5;
    let chart_interior_right = 298.5;
    let chart_interior_top = 653.5;
    let chart_exterior_left = 50.0;
    let chart_exterior_bottom = 210.0;
    let chart_exterior_right = 300.0;
    let chart_exterior_top = 655.0;
    let line_thin = 0.5;
    let line_medium = 1.0;
    let line_bold = 1.5;

    // some reference lines

    page.objects_mut().create_path_object_line(
        page_width/2.0, 
        PdfPoints::new(0.0), 
        page_width/2.0, 
        page_height,
        PdfColor::BLACK,
        PdfPoints::new(line_thin),
    );

    page.objects_mut().create_path_object_rect(
        PdfRect::new(
            PdfPoints::new(chart_exterior_bottom), 
            PdfPoints::new(chart_exterior_left), 
            PdfPoints::new(chart_exterior_top), 
            PdfPoints::new(chart_exterior_right)
        ),
        Some(PdfColor::BLACK),
        Some(PdfPoints::new(line_thin)),
        None
    );

    page.objects_mut().create_path_object_rect(
        PdfRect::new(
            PdfPoints::new(chart_interior_bottom), 
            PdfPoints::new(chart_interior_left), 
            PdfPoints::new(chart_interior_top), 
            PdfPoints::new(chart_interior_right)
        ),
        Some(PdfColor::BLACK),
        Some(PdfPoints::new(0.5)),
        None
    );

    // Interior of chart
    for i in 1..=chart_num_rows {
        page.objects_mut().create_path_object_line(
            PdfPoints::new(chart_interior_left),
            PdfPoints::new(
                chart_interior_bottom + i as f32 
                * (chart_interior_top-chart_interior_bottom)/chart_num_rows as f32),
            PdfPoints::new(chart_interior_right),
            PdfPoints::new(chart_interior_bottom + i as f32 
                * (chart_interior_top-chart_interior_bottom)/chart_num_rows as f32),
            PdfColor::BLACK,
            PdfPoints::new(line_thin),
        );
    }

    // Vertical lines
    for i in 0..=6 {
        page.objects_mut().create_path_object_line(
            PdfPoints::new(98.0 + i as f32 * 150.0/6.0),
            PdfPoints::new(chart_interior_bottom),
            PdfPoints::new(98.0 + i as f32 * 150.0/6.0),
            PdfPoints::new(chart_interior_top),
            PdfColor::BLACK,
            PdfPoints::new(line_thin),
        );
    }

    // Bolded vertical lines
    let right_bold_line = 248.0 + 150.0/6.0;
    page.objects_mut().create_path_object_line(
        PdfPoints::new(right_bold_line),
        PdfPoints::new(chart_interior_bottom),
        PdfPoints::new(right_bold_line),
        PdfPoints::new(chart_interior_top),
        PdfColor::BLACK,
        PdfPoints::new(line_medium)
    );

    let left_bold_line = 98.0;
    page.objects_mut().create_path_object_line(
        PdfPoints::new(left_bold_line),
        PdfPoints::new(chart_interior_bottom),
        PdfPoints::new(left_bold_line),
        PdfPoints::new(chart_interior_top),
        PdfColor::BLACK,
        PdfPoints::new(line_medium)
    );

    // Horizontal bold lines
    page.objects_mut().create_path_object_line(
        PdfPoints::new(chart_interior_left),
        PdfPoints::new(chart_interior_bottom + (chart_interior_top-chart_interior_bottom)/chart_num_rows as f32),
        PdfPoints::new(chart_interior_right),
        PdfPoints::new(chart_interior_bottom + (chart_interior_top-chart_interior_bottom)/chart_num_rows as f32),
        PdfColor::BLACK,
        PdfPoints::new(line_bold),
    );

    page.objects_mut().create_path_object_line(
        PdfPoints::new(chart_interior_left),
        PdfPoints::new(chart_interior_top - (chart_interior_top-chart_interior_bottom)/chart_num_rows as f32),
        PdfPoints::new(chart_interior_right),
        PdfPoints::new(chart_interior_top - (chart_interior_top-chart_interior_bottom)/chart_num_rows as f32),
        PdfColor::BLACK,
        PdfPoints::new(line_bold),
    );

    // Gray warning box
    let warning_box_left = chart_exterior_right + 13.0;
    let warning_box_right = page_width.value - 50.0;
    let warning_box_bottom = chart_exterior_bottom;
    let warning_box_top = chart_exterior_bottom + 50.0;
    page.objects_mut().create_path_object_rect(
        PdfRect::new(
            PdfPoints::new(warning_box_bottom), 
            PdfPoints::new(warning_box_left), 
            PdfPoints::new(warning_box_top), 
            PdfPoints::new(warning_box_right)
        ),
        Some(PdfColor::BLACK),
        Some(PdfPoints::new(line_bold)),
        Some(PdfColor::LIGHT_GREY)
    );
    
    let chart_row_spacing = (chart_interior_top - chart_interior_bottom)/33.0;
    let mut dates = super::generate_date_strings(year, month);
    dates.clone()
        .into_iter()
        .enumerate()
        .for_each(|(num, val)| {    
            page.objects_mut().create_text_object(
                PdfPoints::new(53.0),
                PdfPoints::new(chart_interior_top - 22.0 - num as f32 * chart_row_spacing ),
                format!("{}", val),
                document.fonts_mut().helvetica(),
                PdfPoints::new(7.5));
        });
    page.objects_mut().create_text_object(
        PdfPoints::new(64.0),
        PdfPoints::new(chart_interior_top - 9.5),
        "Days",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(7.5)
        );

    let mut header = vec!["A","B","C","D","E","F","G"];
    let chart_col_spacing = (right_bold_line - left_bold_line)/7.0;
    header.into_iter()
        .enumerate()
        .for_each(|(num, val)| {
            page.objects_mut().create_text_object(
                PdfPoints::new(107.0 + num as f32 * chart_col_spacing),
                PdfPoints::new(chart_interior_top - 9.5),
                format!("{}", val),
                document.fonts_mut().helvetica_bold(),
                PdfPoints::new(10.0));
        });
    page.objects_mut().create_text_object(
        PdfPoints::new(right_bold_line + 2.0),
        PdfPoints::new(chart_interior_top - 8.0),
        "TOTAL",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(6.0)
    );
    page.objects_mut().create_text_object(
        PdfPoints::new(left_bold_line - 40.0),
        PdfPoints::new(chart_interior_bottom + 3.0),
        "TOTAL",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    // Text objects
    // Builtin PDF Font Tokens are:
    // "Times-Roman", "Times-Bold", "Times-Italic", "Times-BoldItalic",
    // "Helvetica", "Helvetica-Bold", "Helvetica-Oblique", "Helvetica-BoldOblique",
    // "Courier", "Courier-Bold", "Courier-Oblique", "Courier-BoldOblique",
    // "Symbol", and "ZapfDingbats" for some reason.

    // Title
    page.objects_mut().create_text_object(
        PdfPoints::new(233.0),
        page_height-PdfPoints::new(50.0),
        "Monthly OJT Form",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(15.0)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(240.0 - coordinator.len() as f32 * 3.0),
        page_height-PdfPoints::new(65.0),
        format!("Return to: {}, COORDINATOR", coordinator.to_uppercase()),
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(50.0),
        PdfPoints::new(695.0),
        "Apprentice Name:",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_path_object_line(
    	PdfPoints::new(130.0),
    	PdfPoints::new(692.0),
    	PdfPoints::new(280.0),
    	PdfPoints::new(692.0),
    	PdfColor::BLACK,
    	PdfPoints::new(line_medium)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(350.0),
        PdfPoints::new(695.0),
        "Employer:",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_path_object_line(
    	PdfPoints::new(397.0),
    	PdfPoints::new(692.0),
    	PdfPoints::new(550.0),
    	PdfPoints::new(692.0),
    	PdfColor::BLACK,
    	PdfPoints::new(line_medium)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(50.0),
        PdfPoints::new(680.0),
        "Hourly Wage:",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_text_object(
    	PdfPoints::new(115.0),
    	PdfPoints::new(680.0),
    	"[   ] Below Average   [   ] Average   [   ] Above Average",
    	document.fonts_mut().helvetica(),
    	PdfPoints::new(9.0)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(350.0),
        PdfPoints::new(680.0),
        "School Year:",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_text_object(
    	PdfPoints::new(410.0),
    	PdfPoints::new(680.0),
    	"[   ] 1   [   ] 2   [   ] 3   [   ] 4",
    	document.fonts_mut().helvetica(),
    	PdfPoints::new(9.0)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(50.0),
        PdfPoints::new(665.0),
        "   OJT Month:",
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    ); 

    page.objects_mut().create_text_object(
        PdfPoints::new(110.0),
        PdfPoints::new(665.0),
        format!(" {}", super::generate_monthyear(year, month)), 
        document.fonts_mut().helvetica(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_path_object_line(
    	PdfPoints::new(110.0),
    	PdfPoints::new(662.0),
    	PdfPoints::new(210.0),
    	PdfPoints::new(662.0),
    	PdfColor::BLACK,
    	PdfPoints::new(line_medium)
    );

    let mut paragraph = vec![
        "The Manasota Plumbing Apprenticeship Program requires 8,000",
        "OT hours over a four-year period (2,000 hours per year) in",
        "relation to letters A thru G as listed below. Please make every",
        "effort to give your employee(s) the right amount of training hours", 
        "to satisfy the Program requirements."
    ];
    paragraph.into_iter()
        .enumerate()
        .for_each(|(num, val)| {
            page.objects_mut().create_text_object(
                PdfPoints::new(chart_exterior_right + 13.0 ),
                PdfPoints::new(chart_interior_top - 8.0 - num as f32 * 12.0),
                val,
                document.fonts_mut().helvetica(),
                PdfPoints::new(9.0)
            );
        });
 
    let mut paragraph = vec![
        "You are required to turn in one sheet for each of the twelve",
        "months before the 10th of the following month."
    ];
    paragraph.into_iter()
        .enumerate()
        .for_each(|(num, val)| {
            page.objects_mut().create_text_object(
                PdfPoints::new(chart_exterior_right + 13.0 ),
                PdfPoints::new(chart_interior_top - 90.0 - num as f32 * 12.0),
                val,
                document.fonts_mut().helvetica(),
                PdfPoints::new(9.0)
            );
        });

    let mut lcolelements = vec!["A","B","C","D","E","F","G"];
    let mut mcolelements = vec![
        "Safety",
        "Preparation, loading, use and selection of various tools,",
        "supplies, machinery and equipment for the trade.",
        "Hot and cold water systems, water treatment, hot water",
        "and steam systems, boilers, and backflow prevention.",
        "Fixture installation",
        "Determining kinds of pipe and proper installation for",
        "drainage, waste and venting. Fire stopping and",
        "installations.",
        "Gas and industrial piping",
        "Trouble shooting and repairs",
    ];
    let mut rcolelements = vec![
        "   300", "1,400", "2,100", "1,200", 
        "1,600", "   800", "   600"
    ];
    let mut lines = [1, 2, 2, 1, 3, 1, 1];
    let mut lindex = 0;
    let mut height = 0.0;
    for (num, nlines) in lines.iter().enumerate() {
        page.objects_mut().create_text_object(
            PdfPoints::new(chart_exterior_right + 13.0),
            PdfPoints::new(chart_interior_top - 126.0 
                - num as f32 * 18.0 
                - lindex as f32 * 12.0),
            lcolelements[num], 
            document.fonts_mut().helvetica_bold(),
            PdfPoints::new(9.0)
        );
        for x in 0..*nlines {
            height = chart_interior_top - 126.0 
                - num as f32 * 18.0 
                - lindex as f32 * 12.0
                - x as f32 * 12.0;
            page.objects_mut().create_text_object(
            PdfPoints::new(chart_exterior_right + 28.0),
            PdfPoints::new(height),
            mcolelements[lindex + x], 
            document.fonts_mut().helvetica(),
            PdfPoints::new(9.0)
            );
            if x == *nlines - 1 {
                page.objects_mut().create_text_object(
                page_width - PdfPoints::new(62.0),
                PdfPoints::new(height),
                rcolelements[num], 
                document.fonts_mut().helvetica(),
                PdfPoints::new(9.0));
            }
        }
        lindex += nlines;
    }

    page.objects_mut().create_path_object_line(
    	PdfPoints::new(520.0),
    	PdfPoints::new(height - 13.0),
    	PdfPoints::new(573.0),
    	PdfPoints::new(height - 13.0),
    	PdfColor::BLACK,
    	PdfPoints::new(line_medium)
    );

    page.objects_mut().create_text_object(
        page_width - PdfPoints::new(182.5),
        PdfPoints::new(height - 30.0),
        "TOTAL HOURS REQUIRED: 8,000", 
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(warning_box_left + 5.0),
        PdfPoints::new(warning_box_top - 10.0),
        "PRINT CLEARLY                          USE FULL HOURS ONLY", 
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(warning_box_left + 50.0),
        PdfPoints::new(warning_box_top - 28.0),
        "Make sure A through G totals are in the", 
        document.fonts_mut().helvetica(),
        PdfPoints::new(9.0)
    );

    page.objects_mut().create_text_object(
        PdfPoints::new(warning_box_left + 50.0),
        PdfPoints::new(warning_box_top - 40.0),
        "TOTAL on the bottom and on the right", 
        document.fonts_mut().helvetica(),
        PdfPoints::new(9.0)
    );

    // Lower section
    let lower_header_x = chart_exterior_right - 105.0;
    let lower_header_y = chart_exterior_bottom - 20.0;
    page.objects_mut().create_text_object(
        PdfPoints::new(lower_header_x),
        PdfPoints::new(lower_header_y),
        "EMPLOYER / SUPERVISOR ON-THE-JOB EVALUATION", 
        document.fonts_mut().helvetica_bold(),
        PdfPoints::new(9.0)
    );

    let mut radios = vec![
        "Work Habits:", "Dependability:", "Trade Knowledge:", 
        "Attitude:", "Cooperation:"
    ];
    radios.iter()
        .enumerate()
        .for_each(|(num, x)| {
            page.objects_mut().create_text_object(
                PdfPoints::new(lower_header_x - 30.0 ),
                PdfPoints::new(lower_header_y - (num+1) as f32 * 18.0),
                x, 
                document.fonts_mut().helvetica_bold(),
                PdfPoints::new(9.0)
            );
        });

    for x in 1..=5 {
    page.objects_mut().create_text_object(
    	PdfPoints::new(lower_header_x + 60.0),
    	PdfPoints::new(lower_header_y - x as f32 * 18.0),
    	"[   ] Below Average   [   ] Average   [   ] Above Average",
    	document.fonts_mut().helvetica(),
    	PdfPoints::new(9.0)
    );}

	page.objects_mut().create_text_object(
    	PdfPoints::new(80.0),
    	PdfPoints::new(50.0),
    	"Employer / Supervisor Signature",
    	document.fonts_mut().helvetica_bold(),
    PdfPoints::new(9.0));

    page.objects_mut().create_text_object(
    	PdfPoints::new(340.0),
    	PdfPoints::new(50.0),
    	"Title",
    	document.fonts_mut().helvetica_bold(),
    PdfPoints::new(9.0));

    page.objects_mut().create_text_object(
    	PdfPoints::new(490.0),
    	PdfPoints::new(50.0),
    	"Date",
    	document.fonts_mut().helvetica_bold(),
    PdfPoints::new(9.0));

    page.objects_mut().create_path_object_line(
    	PdfPoints::new(50.0),
    	PdfPoints::new(60.0),
    	PdfPoints::new(250.0),
    	PdfPoints::new(60.0),
    	PdfColor::BLACK,
    	PdfPoints::new(line_medium)
    );

    page.objects_mut().create_path_object_line(
    	PdfPoints::new(300.0),
    	PdfPoints::new(60.0),
    	PdfPoints::new(400.0),
    	PdfPoints::new(60.0),
    	PdfColor::BLACK,
    	PdfPoints::new(line_medium)
    );

    page.objects_mut().create_path_object_line(
    	PdfPoints::new(450.0),
    	PdfPoints::new(60.0),
    	PdfPoints::new(550.0),
    	PdfPoints::new(60.0),
    	PdfColor::BLACK,
    	PdfPoints::new(line_medium)
    );


    // todo!("Fill primitives with data, form fields");
    // todo!("Export PDF");
    let export_path = format!("pdfs/{}-{}.pdf", year, month);
    document.save_to_file(export_path.as_str())?;
    Ok(())
}