use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle, PrimitiveStyle},
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
};
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, StrokeAlignment};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let yoffset = 10;


    // Draw a 3px wide outline around the display.
    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut display)?;

    Circle::new(Point::new(72, 8), 48)
        .into_styled(line_style)
        .draw(&mut display)?;

    Line::new(Point::new(48, 16), Point::new(8, 16))
        .into_styled(line_style)
        .draw(&mut display)?;

    Line::new(Point::new(48, 16), Point::new(64, 32))
        .into_styled(line_style)
        .draw(&mut display)?;

    Rectangle::new(Point::new(0, 0), Size::new(34, 34))
        .into_styled(line_style)
        .draw(&mut display)?;

    Text::new("Aqua-Control", Point::new(4, 8), text_style).draw(&mut display)?;
    Text::new("Temp: 21°C", Point::new(40, 8), text_style).draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}