#![windows_subsystem = "windows"]

mod clock_widget;
mod simple_rect;
use clock_widget::ClockWidget;
use druid::{widget::Align, AppLauncher, Monitor, PlatformError, Screen, Widget, WindowDesc};
use simple_rect::SimpleRect;

pub const WIN_SIZE: (f64, f64) = (800., 400.);

fn build_ui() -> impl Widget<()> {
    SimpleRect::new(Align::centered(ClockWidget::new()))
}

fn main() -> Result<(), PlatformError> {
    let monitor: &Monitor = &Screen::get_monitors()
        .into_iter()
        .filter(|x| x.is_primary())
        .collect::<Vec<Monitor>>()[0];
    let pos = monitor.virtual_rect().center() - (WIN_SIZE.0 / 2., WIN_SIZE.1 / 2.);
    let main_w = WindowDesc::new(build_ui())
        .title("Clock")
        .set_position(pos)
        .resizable(false)
        .show_titlebar(false)
        .window_size(WIN_SIZE);

    AppLauncher::with_window(main_w).launch(())?;
    Ok(())
}
