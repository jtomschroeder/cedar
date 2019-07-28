use cedar::sml;

type Entries = Vec<Entry>;

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

#[derive(PartialEq)]
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

        &Message::UpdateField(ref s) => model.field = s.clone(),

        &Message::UpdateEntry(id, ref task) => model
            .entries
            .iter_mut()
            .find(|e| e.id == id)
            .iter_mut()
            .for_each(|e| e.description = task.clone()),

        &Message::Delete(id) => model.entries.retain(|e| e.id != id),

        &Message::DeleteComplete => model.entries.retain(|e| !e.completed),

        &Message::Check(id, completed) => model
            .entries
            .iter_mut()
            .find(|e| e.id == id)
            .iter_mut()
            .for_each(|e| e.completed = completed),

        &Message::CheckAll(completed) => model
            .entries
            .iter_mut()
            .for_each(|e| e.completed = completed),

        &Message::ChangeVisibility(ref visibility) => model.visibility = visibility.clone(),
    }

    model
}

type Widget = cedar::dom::Object<Message>;

fn view(model: &Model) -> Widget {
    sml! {
        (div (@ (class "todomvc-wrapper") (style "visibility: hidden"))
            (section (@ (class "todoapp"))
                {view_input(&model.field)}
                {view_entries(&model.visibility, &model.entries)}
                {view_controls(&model.visibility, &model.entries)}
            )
            {info_footer()}
        )
    }
}

fn view_input(task: &str) -> Widget {
    sml! {
        (header (@ (class "header"))
            (h1 { "todos" })
            (input
                (@
                    (class "new-todo")
                    (placeholder "What needs to be done?")
                    (autofocus "true")
                    (value task)
                    (name "newTodo")
                    (input Message::UpdateField)
                    (keydown |code| { if code == 13 /* ENTER */ { Some(Message::Add) } else { None }})
                )
            )
        )
    }
}

fn view_entries(visibility: &str, entries: &[Entry]) -> Widget {
    let all_completed = entries.iter().all(|e| e.completed);

    let vis = if entries.is_empty() {
        "hidden"
    } else {
        "visible"
    };

    let todos: Vec<_> = entries
        .iter()
        .filter(|e| -> bool {
            match visibility {
                "Completed" => e.completed,
                "Active" => !e.completed,
                _ => true,
            }
        })
        .map(view_entry)
        .collect();

    sml! {
        (section (@ (class "main") (style format!("visibility: {}", vis)))
            (input (@
                (class "toggle-all")
                (type "checkbox")
                (name "toggle")
                (checked { if all_completed { "true" } else { "false" } })
                (click Message::CheckAll(!all_completed))
            ))
            (ul (@ (class "todo-list")) {todos})
        )
    }
}

fn view_entry(entry: &Entry) -> Widget {
    let &Entry {
        ref description,
        completed,
        id,
    } = entry;

    sml! {
        (li
            (div (@ (class "view"))
                (input (@
                    (class "toggle")
                    (type "checkbox")
                    (checked { if completed { "true" } else { "false" } })
                    (click Message::Check(id, !completed))
                ))
                (label {description.clone()})
            )
            (button (@ (class "destroy") (click Message::Delete(id))))
            (input (@
                (class "edit")
                (value description.clone())
                (name "title")
                (id format!("todo-{}", id))
                (input { move |s| Message::UpdateEntry(id.clone(), s) })
            ))
        )
    }
}

fn view_controls(visibility: &str, entries: &[Entry]) -> Widget {
    let num_completed = entries.iter().filter(|e| e.completed).count();
    let num_left = entries.len() - num_completed;

    sml! {
        (footer (@ (class "footer")
                   (hidden entries.is_empty()))
            {view_controls_count(num_left)}
            {view_controls_filters(visibility)}
            {view_controls_clear(num_completed)}
        )
    }
}

fn view_controls_count(num_left: usize) -> Widget {
    let item = match num_left {
        1 => "item",
        _ => "items",
    };

    let s = format!("{} {} left", num_left, item);

    sml! {
        (span (@ (class "todo-count")) {s})
    }
}

fn view_controls_filters(visibility: &str) -> Widget {
    sml! {
        (ul (@ (class "filters"))
            {visibility_swap("All", visibility)}
            {" "}
            {visibility_swap("Active", visibility)}
            {" "}
            {visibility_swap("Completed", visibility)}
        )
    }
}

fn visibility_swap(visibility: &str, actual_visibility: &str) -> Widget {
    sml! {
        (li (@ (click Message::ChangeVisibility(visibility.into())))
            (a (@ (class {if visibility == actual_visibility { "selected" } else { "" }}))
                {visibility}
            )
        )
    }
}

fn view_controls_clear(num_completed: usize) -> Widget {
    sml! {
        (button (@ (class "clear-completed")
                   (hidden num_completed == 0)
                   (click Message::DeleteComplete))
            {format!("Clear completed ({})", num_completed)}
        )
    }
}

fn info_footer() -> Widget {
    sml! {
        (footer (@ (class "info"))
            (p {"Written by Tom Schroeder using cedar!"})
        )
    }
}

fn main() {
    cedar::Application::new(Model::empty(), update, view)
        .style(&cedar::sass::compile(include_str!("todo.scss")))
        .run()
}
