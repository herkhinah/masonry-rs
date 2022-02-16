use crate::testing::{Harness, ModularWidget};
use crate::*;
use instant::Duration;
use std::cell::Cell;
use std::rc::Rc;
use test_log::test;

#[test]
fn basic_timer() {
    let timer_handled: Rc<Cell<bool>> = Rc::new(false.into());

    let widget = ModularWidget::new((None, timer_handled.clone()))
        .lifecycle_fn(move |state, ctx, event, _| match event {
            LifeCycle::WidgetAdded => {
                ctx.init();
                state.0 = Some(ctx.request_timer(Duration::from_secs(3)));
            }
            _ => {}
        })
        .event_fn(|state, ctx, event, _| {
            ctx.init();
            if let Event::Timer(token) = event {
                if *token == state.0.unwrap() {
                    state.1.set(true);
                }
            }
        });

    let mut harness = Harness::create(widget);

    assert_eq!(timer_handled.get(), false);

    harness.move_timers_forward(Duration::from_secs(1));
    assert_eq!(timer_handled.get(), false);

    harness.move_timers_forward(Duration::from_secs(2));
    assert_eq!(timer_handled.get(), true);
}