use std::time::Duration;

use druid::widget::Label;
use druid::{Env, Event, TimerToken, Widget, WidgetPod};

static TIMER_INTERVAL: Duration = Duration::from_millis(500);

pub struct ClockWidget {
    timer_id: TimerToken,
    curr_time: String,
    label: WidgetPod<String, Label<String>>,
}

impl<T> Widget<T> for ClockWidget {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, _: &mut T, _env: &Env) {
        match event {
            Event::WindowConnected => {
                self.curr_time = chrono::Local::now().format("%H:%M:%S").to_string();
                self.label.widget_mut().set_text(&*self.curr_time);
                ctx.request_update();

                self.timer_id = ctx.request_timer(TIMER_INTERVAL);
            }
            Event::Timer(id) => {
                if *id == self.timer_id {
                    self.curr_time = chrono::Local::now().format("%H:%M:%S").to_string();
                    self.label.widget_mut().set_text(&*self.curr_time);
                    ctx.request_update();

                    self.timer_id = ctx.request_timer(TIMER_INTERVAL);
                }
            }
            Event::MouseUp(_) => {
                // Idea: change styles?
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        _data: &T,
        env: &Env,
    ) {
        self.label.lifecycle(ctx, event, &self.curr_time, env)
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &T, _data: &T, env: &Env) {
        self.label.update(ctx, &self.curr_time, env)
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &T,
        env: &Env,
    ) -> druid::Size {
        self.label.layout(ctx, bc, &self.curr_time, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &T, env: &Env) {
        self.label.paint(ctx, &self.curr_time, env)
    }
}

impl ClockWidget {
    pub fn new() -> Self {
        ClockWidget {
            timer_id: TimerToken::INVALID,
            curr_time: "".to_string(),
            label: WidgetPod::new(Label::new("").with_text_size(100.)),
        }
    }
}
