
extern crate cedar;

// TODO: move 'on enter' to JS front-end

type Entries = Vec<Entry>;

#[derive(Debug)]
struct Model {
    entries: Entries,
    visibility: String,
    field: String,
    uid: u32,
}

impl Model {
    fn empty() -> Self {
        Model {
            entries: Entries::new(),
            visibility: "All".into(),
            field: "".into(),
            uid: 0,
        }
    }
}

#[derive(Debug)]
struct Entry {
    description: String,
    completed: bool,
    id: u32,
}

impl Entry {
    fn new(description: String, id: u32) -> Self {
        Entry {
            description,
            completed: false,
            id,
        }
    }
}

#[derive(PartialEq, Debug)]
enum Message {
    UpdateField(String),
    UpdateEntry(u32, String),
    Add,
    Delete(u32),
    DeleteComplete,
    Check(u32, bool),
    CheckAll(bool),
    ChangeVisibility(String),
}

fn update(mut model: Model, message: &Message) -> Model {
    match message {
        &Message::Add => {
            let uid = model.uid;
            let field = model.field.split_off(0);

            if !field.is_empty() {
                model.entries.push(Entry::new(field, uid));
            }

            model.uid += 1;
        }

        &Message::UpdateField(ref s) => {
            model.field = s.clone();
        }

        &Message::UpdateEntry(id, ref task) => {
            if let Some(entry) = model.entries.iter_mut().find(|e| e.id == id) {
                entry.description = task.clone();
            }
        }

        &Message::Delete(id) => {
            model.entries.retain(|e| e.id != id);
        }

        &Message::DeleteComplete => {
            model.entries.retain(|e| !e.completed);
        }

        &Message::Check(id, completed) => {
            if let Some(entry) = model.entries.iter_mut().find(|e| e.id == id) {
                entry.completed = completed;
            }
        }

        &Message::CheckAll(completed) => {
            for entry in model.entries.iter_mut() {
                entry.completed = completed;
            }
        }

        &Message::ChangeVisibility(ref visibility) => {
            model.visibility = visibility.clone();
        }
    }

    model
}

use cedar::dom::*;
type Widget = Object<Message>;

fn view(model: &Model) -> Widget {
    div()
        .class("todomvc-wrapper")
        .style("visibility: hidden")
        .add(
            section()
                .class("todoapp")
                .add(view_input(&model.field))
                .add(view_entries(&model.visibility, &model.entries))
                .add(view_controls(&model.visibility, &model.entries)),
        )
        .add(info_footer())
}

fn view_input(task: &str) -> Widget {
    header().class("header").add(h1().add(text("todos"))).add(
        input()
            .class("new-todo")
            .placeholder("What needs to be done?")
            .attr("autofocus", "true")
            .attr("value", task)
            .attr("name", "newTodo")
            .input(Message::UpdateField)
            .keydown(|code| {
                if code == 13 /* ENTER */ { Some(Message::Add) } else { None }
            }),
    )
}

fn view_entries(visibility: &str, entries: &[Entry]) -> Widget {
    let all_completed = entries.iter().all(|e| e.completed);

    let vis = if entries.is_empty() {
        "hidden"
    } else {
        "visible"
    };

    section()
        .class("main")
        .style(format!("visibility: {}", vis))
        .add(
            input()
                .class("toggle-all")
                .attr("type", "checkbox")
                .attr("name", "toggle")
                .attr("checked", if all_completed { "true" } else { "false" })
                .click(Message::CheckAll(!all_completed)),
        )
        .add(
            ul().class("todo-list").children(
                entries
                    .iter()
                    .filter(|todo| -> bool {
                        match visibility {
                            "Completed" => todo.completed,
                            "Active" => !todo.completed,
                            _ => true,
                        }
                    })
                    .map(view_entry)
                    .collect(),
            ),
        )
}

fn view_entry(
    &Entry {
        ref description,
        completed,
        id,
    }: &Entry,
) -> Widget {
    li()
        .add(
            div()
                .class("view")
                .add(
                    input()
                        .class("toggle")
                        .attr("type", "checkbox")
                        .attr("checked", if completed { "true" } else { "false" })
                        .click(Message::Check(id, !completed)),
                )
                .add(label().add(text(description))),
        ).add(button().class("destroy").click(Message::Delete(id)))
        .add(
            input()
                .class("edit")
                .attr("value", description)
                .attr("name", "title")
                .attr("id", &format!("todo-{}", id))
                .input(move |s| Message::UpdateEntry(id.clone(), s)),
        )
}

fn view_controls(visibility: &str, entries: &[Entry]) -> Widget {
    let num_completed = entries.iter().filter(|e| e.completed).count();
    let num_left = entries.len() - num_completed;

    footer()
        .class("footer")
        .hidden(entries.is_empty())
        .add(view_controls_count(num_left))
        .add(view_controls_filters(visibility))
        .add(view_controls_clear(num_completed))
}

fn view_controls_count(num_left: usize) -> Widget {
    let item = match num_left {
        1 => "item",
        _ => "items",
    };

    let s = format!("{} {} left", num_left, item);
    span().class("todo-count").add(text(&s))
}

fn view_controls_filters(visibility: &str) -> Widget {
    ul()
        .class("filters")
        .add(visibility_swap("All", visibility))
        .add(text(" "))
        .add(visibility_swap("Active", visibility))
        .add(text(" "))
        .add(visibility_swap("Completed", visibility))
}

fn visibility_swap(visibility: &str, actual_visibility: &str) -> Widget {
    let a = a().add(text(visibility));
    let a = if visibility == actual_visibility {
        a.class("selected")
    } else {
        a
    };

    li()
        .click(Message::ChangeVisibility(visibility.into()))
        .add(a)
}

fn view_controls_clear(num_completed: usize) -> Widget {
    button()
        .class("clear-completed")
        .hidden(num_completed == 0)
        .add(text(&format!("Clear completed ({})", num_completed)))
        .click(Message::DeleteComplete)
}

fn info_footer() -> Widget {
    footer().class("info").add(p().add(text(
        "Written by Tom Schroeder using `cedar`!",
    )))
}

fn main() {
    cedar::program(Model::empty(), update, view)
}
