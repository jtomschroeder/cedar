
use std::thread;

use facade;
use dom;
use phantom::Phantom;
use renderer::Renderer;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub enum Action<S> {
    Update(S),
    Layout(f32, f32),
}

pub fn program<S, M>(mut model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq,
    M: Send + 'static,
{
    let renderer = facade::Renderer::new();

    //
    // TODO: separate `model` and `update` from `view` and `renderer`
    // - `model` & `update` => fluxion (flux)
    // - `view` & `renderer` => phantom (i.e. shadow DOM)
    //
    // - use single (blocking) queue for events from backend & events from 'effects'
    //   - effects => commands, subscriptions
    //

    // TODO: pass initial state to renderer
    // - e.g. initial width & height

    let (mut width, mut height) = (500., 500.);

    {
        let renderer = renderer.clone();

        thread::spawn(move || {
            let (mut phantom, commands) = Phantom::initialize(&model, view, width, height);

            for event in commands.into_iter() {
                renderer.send(event);
            }

            // Receive messages from 'renderer'

            loop {
                let event = renderer.recv(); // blocking!

                // println!("event: {:?}", event);

                // translate events from backend renderer to actions
                let action = phantom.translate(event);

                // TODO: `translate` could return (Action?, Commands?) to decouple layout from message

                let action = match action {
                    Some(a) => a,
                    _ => continue,
                };

                let commands = match action {
                    Action::Update(message) => {
                        model = update(model, message);

                        // TODO: might be better to change Update to fn(Model, &Message)
                        // TODO: inject middleware here: middleware.handlers(&model, &message)

                        phantom.update(&model, view, width, height)
                    }

                    Action::Layout(w, h) => {
                        width = w;
                        height = h;

                        let cmd = phantom.layout(width, height);
                        vec![cmd]
                    }
                };

                for event in commands.into_iter() {
                    renderer.send(event);
                }
            }
        });
    }

    facade::run(renderer) // run renderer on 'main' thread!
}
