use std::f64::NAN;

use druid::{Data, Event, Point, Widget, WidgetPod};

use crate::WIN_SIZE;

pub struct SimpleRect<T, W>
where
    T: Data,
    W: Widget<T>,
{
    child: WidgetPod<T, W>,
    mov_start_pos: Point,
    is_mouse_down: bool,
}

impl<T, W> SimpleRect<T, W>
where
    T: Data,
    W: Widget<T>,
{
    pub fn new(child: W) -> Self {
        SimpleRect {
            child: WidgetPod::new(child),
            mov_start_pos: Point::new(NAN, NAN),
            is_mouse_down: false,
        }
    }
}

impl<T, W> Widget<T> for SimpleRect<T, W>
where
    T: Data,
    W: Widget<T>,
{
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut T,
        env: &druid::Env,
    ) {
        match event {
            Event::MouseMove(m) => {
                if m.buttons.has_left() {
                    ctx.window()
                        .set_position(ctx.window().get_position() + (m.window_pos - self.mov_start_pos));
                }
            }
            Event::MouseDown(m) => {
                if !self.is_mouse_down {
                    self.mov_start_pos = m.window_pos;
                    self.is_mouse_down = true
                }
            }
            Event::MouseUp(_m) => self.is_mouse_down = false,
            _ => (),
        }
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        data: &T,
        env: &druid::Env,
    ) {
        self.child.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, _old_data: &T, data: &T, env: &druid::Env) {
        self.child.update(ctx, data, env)
    }

    fn layout(
        &mut self,
        ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        data: &T,
        env: &druid::Env,
    ) -> druid::Size {
        bc.constrain(WIN_SIZE);
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, env: &druid::Env) {
        self.child.paint(ctx, data, env);
    }
}
