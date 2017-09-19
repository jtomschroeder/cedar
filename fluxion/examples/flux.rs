
extern crate fluxion;

// use cedar::dom;
// use cedar::dom::Builder;

type Model = i32;

#[derive(PartialEq, Debug, Clone)]
enum Message {
    Increment,
    Decrement,
}

fn update(model: Model, message: Message) -> Model {
    match message {
        Message::Increment => model + 1,
        Message::Decrement => model - 1,
    }
}

// subscriptions : Model -> Sub Msg
// subscriptions model =
//   Time.every second Tick

fn main() {
    // cedar::program(0, update, view)

    println!("Hello, Flux!");
}