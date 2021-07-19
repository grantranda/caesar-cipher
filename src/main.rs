use std::sync::Arc;

use druid::{
    AppLauncher, Color, Data, Env, Event, EventCtx, FontDescriptor, FontWeight, Insets, Lens,
    LensExt, LocalizedString, theme, Widget, WidgetExt, WindowDesc,
};
use druid::widget::{
    Controller, CrossAxisAlignment, Flex, Label, LensWrap, Parse, RadioGroup, Stepper,
    TextBox, ViewSwitcher,
};

fn main() {
    let window = WindowDesc::new(build_root())
        .title(LocalizedString::new("Caesar Cipher"))
        .window_size((750.0, 660.0));

    let data = AppData {
        current_view: 0,
        conversion: ConversionType::Encryption,
        shift: 6.0,
        plaintext: "".to_string().into(),
        ciphertext: "".to_string().into(),
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(data)
        .expect("Application failed to launch");
}

#[derive(Clone, Data, PartialEq)]
enum ConversionType {
    Encryption,
    Decryption,
}

#[derive(Clone, Data, Lens)]
struct AppData {
    current_view: u32,
    conversion: ConversionType,
    shift: f64,
    plaintext: Arc<String>,
    ciphertext: Arc<String>,
}

struct ConversionController;

impl<W: Widget<AppData>> Controller<AppData, W> for ConversionController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        let old_data = data.conversion.to_owned();
        child.event(ctx, event, data, env);
        if !data.conversion.same(&old_data) {
            if data.conversion == ConversionType::Encryption {
                data.current_view = 0;
            } else {
                data.current_view = 1;
            }
        }
    }
}

struct ShiftController;

impl<W: Widget<AppData>> Controller<AppData, W> for ShiftController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        let old_data = data.shift.to_owned();
        child.event(ctx, event, data, env);
        if !data.shift.same(&old_data) {
            if data.conversion == ConversionType::Encryption {
                data.ciphertext = Arc::from(encrypt(&data.plaintext.to_owned(), data.shift as i16));
            } else {
                data.plaintext = Arc::from(encrypt(&data.ciphertext.to_owned(), -data.shift as i16));
            }
        }
    }
}

struct PlaintextController;

impl<W: Widget<AppData>> Controller<AppData, W> for PlaintextController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        let old_data = data.plaintext.to_owned();
        child.event(ctx, event, data, env);
        if data.conversion == ConversionType::Encryption && data.plaintext.to_owned().len() != data.ciphertext.to_owned().len() {
            data.ciphertext = Arc::from(encrypt(&old_data, data.shift as i16));
        } else {
            data.plaintext = old_data;
        }
    }
}

struct CiphertextController;

impl<W: Widget<AppData>> Controller<AppData, W> for CiphertextController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppData, env: &Env) {
        let old_data = data.ciphertext.to_owned();
        child.event(ctx, event, data, env);
        if data.conversion == ConversionType::Decryption && data.ciphertext.to_owned().len() != data.plaintext.to_owned().len() {
            data.plaintext = Arc::from(encrypt(&old_data, -data.shift as i16));
        } else {
            data.ciphertext = old_data;
        }
    }
}

fn build_root() -> impl Widget<AppData> {
    let conversion_picker = RadioGroup::new(vec![
        ("Encryption", ConversionType::Encryption),
        ("Decryption", ConversionType::Decryption),
    ])
        .lens(AppData::conversion)
        .controller(ConversionController);

    let shift_input = LensWrap::new(
        Parse::new(TextBox::new()),
        AppData::shift.map(|x| Some(*x), |x, y| *x = y.unwrap_or(5.0)),
    );
    let shift_stepper = Stepper::new()
        .with_range(1.0, 25.0)
        .with_step(1.0)
        .with_wraparound(false)
        .border(theme::DISABLED_BUTTON_DARK, 2.0)
        .lens(AppData::shift)
        .controller(ShiftController);
    let shift_row = Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(shift_input)
        .with_child(shift_stepper)
        .padding(Insets::new(20.0, 0.0, 20.0, 0.0))
        .expand_height();

    let view_switcher = ViewSwitcher::new(
        |data: &AppData, _env| data.current_view,
        |selector, _data, _env| match selector {
            0 => {
                Box::new(
                    build_textbox_view(
                        build_plaintext_input("Input"),
                        build_ciphertext_input("Output"),
                    )
                )
            }
            _ => {
                Box::new(
                    build_textbox_view(
                        build_ciphertext_input("Input"),
                        build_plaintext_input("Output"),
                    )
                )
            }
        },
    );

    Flex::row().cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Flex::row()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_flex_child(
                            Label::new("Caesar Cipher")
                                .with_font(
                                    FontDescriptor::default()
                                        .with_weight(FontWeight::BOLD)
                                        .with_size(26.0)
                                )
                                .padding(Insets::new(20.0, 10.0, 20.0, 0.0))
                                .expand_width()
                                .fix_height(50.0),
                            1.0,
                        )
                )
                .with_default_spacer()
                .with_child(
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(
                            Label::new("Conversion Type:")
                                .expand_width()
                        )
                        .with_default_spacer()
                        .with_child(conversion_picker)
                        .with_default_spacer()
                        .padding(Insets::new(20.0, 30.0, 20.0, 0.0))
                )
                .with_default_spacer()
                .with_child(
                    Label::new("Shift:")
                        .expand_width()
                        .padding(Insets::new(20.0, 0.0, 20.0, 0.0))
                )
                .with_default_spacer()
                .with_flex_child(shift_row, 1.0)
                .fix_width(220.0)
                .background(theme::DISABLED_BUTTON_DARK)
                .border(theme::BORDER_DARK, 1.0)
        )
        .with_flex_child(view_switcher, 1.0)
        .background(theme::DISABLED_BUTTON_DARK)
}

fn build_plaintext_input(secondary_label: &str) -> impl Widget<AppData> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(
                    build_primary_label("Plaintext", Color::rgb8(0, 179, 179)),
                    1.0,
                )
                .with_flex_child(
                    build_secondary_label(secondary_label),
                    1.0,
                )
        )
        .with_default_spacer()
        .with_child(
            TextBox::multiline()
                .lens(AppData::plaintext)
                .controller(PlaintextController)
                .expand_width()
                .fix_height(250.0))
}

fn build_ciphertext_input(secondary_label: &str) -> impl Widget<AppData> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_flex_child(
                    build_primary_label("Ciphertext", Color::rgb8(230, 0, 116)),
                    1.0,
                )
                .with_flex_child(
                    build_secondary_label(secondary_label),
                    1.0,
                )
        )
        .with_default_spacer()
        .with_child(
            TextBox::multiline()
                .lens(AppData::ciphertext)
                .controller(CiphertextController)
                .expand_width()
                .fix_height(250.0))
}

fn build_textbox_view<W: 'static + Widget<AppData>, S: 'static + Widget<AppData>>(top: W, bottom: S) -> impl Widget<AppData> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_default_spacer()
        .with_child(top)
        .with_default_spacer()
        .with_child(bottom)
        .padding(Insets::new(20.0, 10.0, 10.0, 10.0))
        .fix_width(500.0)
        .background(theme::DISABLED_BUTTON_DARK)
}

fn build_primary_label(label: &str, color: Color) -> impl Widget<AppData> {
    Flex::column()
        .with_child(
            Label::new(label)
                .with_text_color(color)
                .with_font(
                    FontDescriptor::default()
                        .with_size(18.0)
                        .with_weight(FontWeight::SEMI_BOLD)
                )
                .align_left()
        )
        .expand_width()
}

fn build_secondary_label(label: &str) -> impl Widget<AppData> {
    Flex::column()
        .with_child(
            Label::new(label)
                .with_font(
                    FontDescriptor::default()
                        .with_size(16.0)
                        .with_weight(FontWeight::SEMI_BOLD)
                )
                .align_right()
        )
        .expand_width()
}

fn encrypt(plaintext: &str, shift: i16) -> String {
    let mut ciphertext = String::with_capacity(plaintext.len());
    for c in plaintext.chars() {
        if c.is_alphabetic() {
            if c.is_uppercase() {
                ciphertext.push((65 + ((c as u8) as i16 + shift - 65).rem_euclid(26)) as u8 as char);
            } else {
                ciphertext.push((97 + ((c as u8) as i16 + shift - 97).rem_euclid(26)) as u8 as char);
            }
        } else {
            ciphertext.push(c);
        }
    }
    ciphertext
}
