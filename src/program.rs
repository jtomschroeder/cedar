use std::thread;

use facade;
use dom;
use phantom::Phantom;
use renderer::Renderer;
use browser;

use serde_json as json;

pub type Update<M, S> = fn(M, S) -> M;
pub type View<M, S> = fn(&M) -> dom::Object<S>;

pub enum Action<S> {
    Update(S),
}

pub fn program<S, M>(mut model: M, update: Update<M, S>, view: View<M, S>)
where
    S: Clone + Send + 'static + PartialEq,
    M: Send + 'static,
{
    browser::log("Hello, world!");

//    let renderer = facade::Renderer::new();

    //
    // TODO: separate `model` and `update` from `view` and `renderer`
    // - `model` & `update` => fluxion (flux)
    // - `view` & `renderer` => phantom (i.e. shadow DOM)
    //
    // - use single (blocking) queue for events from backend & events from 'effects'
    //   - effects => commands, subscriptions
    //

    {
//        let renderer = renderer.clone();

        //        thread::spawn(move || {
        //            let (mut phantom, commands) = Phantom::initialize(&model, view);
        //
        //            for event in commands.into_iter() {
        //                renderer.send(event);
        //            }
        //
        //            // Receive messages from 'renderer'
        //
        //            loop {
        //                let event = renderer.recv(); // blocking!
        //
        //                // translate events from backend renderer to actions
        //                let action = phantom.translate(event);
        //
        //                // TODO: `translate` could return (Action?, Commands?) to decouple layout from message
        //
        //                let action = match action {
        //                    Some(a) => a,
        //                    _ => continue,
        //                };
        //
        //                let commands = match action {
        //                    Action::Update(message) => {
        //                        model = update(model, message);
        //
        //                        // TODO: might be better to change Update to fn(Model, &Message)
        //                        // TODO: inject middleware here: middleware.handlers(&model, &message)
        //
        //                        phantom.update(&model, view)
        //                    }
        //                };
        //
        //                for event in commands.into_iter() {
        //                    renderer.send(event);
        //                }
        //            }
        //        });
    }

    //    facade::run(renderer) // run renderer on 'main' thread!

    {
        let (mut phantom, commands) = Phantom::initialize(&model, view);

        for cmd in commands.into_iter() {
            browser::log(&format!("Event: {:?}", cmd));

            let cmd = json::to_string(&cmd).unwrap();
            browser::command(&cmd);
        }
    }
}
