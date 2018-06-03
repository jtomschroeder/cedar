#![feature(proc_macro)]
#![feature(proc_macro_non_items)]

extern crate cedar;

use cedar::hypertext;

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
    (hypertext! { |model: &Model|
        <div class={"todomvc-wrapper"} style={"visibility: hidden"}>
            <section class={"todoapp"}>
                {view_input(&model.field)}
                {view_entries(&model.visibility, &model.entries)}
                {view_controls(&model.visibility, &model.entries)}
            </section>
            {info_footer()}
        </div>
    })(model)
}

fn view_input(task: &str) -> Widget {
    (hypertext! { |task: &str|
        <header class={"header"}>
            <h1>todos</h1>
            <input class={"new-todo"}
                   placeholder={"What needs to be done?"}
                   autofocus={"true"}
                   value={task}
                   name={"newTodo"}
                   input={Message::UpdateField}
                   keydown={|code| { if code == 13 /* ENTER */ { Some(Message::Add) } else { None }}}>
            </input>
        </header>
    })(task)
}

fn view_entries(visibility: &str, entries: &[Entry]) -> Widget {
    let all_completed = entries.iter().all(|e| e.completed);

    let vis = if entries.is_empty() {
        "hidden"
    } else {
        "visible"
    };

    let todos: cedar::dom::List<_> = entries
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

    (hypertext! { |vis, all_completed, todos|

        <section class={"main"} style={format!("visibility: {}", vis)}>
            <input class={"toggle-all"}
                   type={"checkbox"}
                   name={"toggle"}
                   checked={if all_completed { "true" } else { "false" }}
                   click={Message::CheckAll(!all_completed)}>
            </input>
            <ul class={"todo-list"}>{todos}</ul>
        </section>

    })(vis, all_completed, todos)
}

fn view_entry(entry: &Entry) -> Widget {
    let &Entry {
        ref description,
        completed,
        id,
    } = entry;

    (hypertext! { |id, completed, description|
        <li>
            <div class={"view"}>
                <input class={"toggle"}
                       type={"checkbox"}
                       checked={if completed { "true" } else { "false" }}
                       click={Message::Check(id, !completed)}>
                </input>
                <label>{description}</label>
            </div>
            <button class={"destroy"} click={Message::Delete(id)}></button>
            <input class={"edit"}
                   value={description}
                   name={"title"}
                   id={format!("todo-{}", id)}
                   input={move |s| Message::UpdateEntry(id.clone(), s)}>
            </input>
        </li>
    })(id, completed, description)
}

fn view_controls(visibility: &str, entries: &[Entry]) -> Widget {
    let num_completed = entries.iter().filter(|e| e.completed).count();
    let num_left = entries.len() - num_completed;

    (hypertext! { |entries: &[Entry], num_left, visibility, num_completed|
        <footer class={"footer"}
                hidden={entries.is_empty()}>
            {view_controls_count(num_left)}
            {view_controls_filters(visibility)}
            {view_controls_clear(num_completed)}
        </footer>
    })(entries, num_left, visibility, num_completed)
}

fn view_controls_count(num_left: usize) -> Widget {
    let item = match num_left {
        1 => "item",
        _ => "items",
    };

    let s = format!("{} {} left", num_left, item);

    (hypertext! { |s|
        <span class={"todo-count"}>{s}</span>
    })(s)
}

fn view_controls_filters(visibility: &str) -> Widget {
    (hypertext! { |visibility|
        <ul class={"filters"}>
            {visibility_swap("All", visibility)}
            {" "}
            {visibility_swap("Active", visibility)}
            {" "}
            {visibility_swap("Completed", visibility)}
        </ul>
    })(visibility)
}

fn visibility_swap(visibility: &str, actual_visibility: &str) -> Widget {
    (hypertext! { |visibility: &str, actual_visibility: &str|
        <li click={Message::ChangeVisibility(visibility.into())}>
            <a class={if visibility == actual_visibility { "selected" } else { "" }}>
                {visibility}
            </a>
        </li>
    })(visibility, actual_visibility)
}

fn view_controls_clear(num_completed: usize) -> Widget {
    (hypertext! { |num_completed|
        <button class={"clear-completed"}
                hidden={num_completed == 0}
                click={Message::DeleteComplete}>
            {format!("Clear completed ({})", num_completed)}
        </button>
    })(num_completed)
}

fn info_footer() -> Widget {
    (hypertext! { ||
        <footer class={"info"}>
            <p>{"Written by Tom Schroeder using cedar!"}</p>
        </footer>
    })()
}

fn main() {
    cedar::program(Model::empty(), update, view)
}
