use makepad_widgets::*;
use serde_json::Value;

use crate::todo_list::TodoList;
use crate::todo_item::TodoItem;

// The live_design macro generates a function that registers a DSL code block with the global
// context object (`Cx`).
//
// DSL code blocks are used in Makepad to facilitate live design. A DSL code block defines
// structured data that describes the styling of the UI. The Makepad runtime automatically
// initializes widgets from their corresponding DSL objects. Moreover, external programs (such
// as a code editor) can notify the Makepad runtime that a DSL code block has been changed, allowing
// the runtime to automatically update the affected widgets.
live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import todo_makepad::app_desktop::AppDesktop
    import todo_makepad::app_mobile::AppMobile 

    // The `{{App}}` syntax is used to inherit a DSL object from a Rust struct. This tells the
    // Makepad runtime that our DSL object corresponds to a Rust struct named `App`. Whenever an
    // instance of `App` is initialized, the Makepad runtime will obtain its initial values from
    // this DSL object.
    App = {{App}} {
        // The `ui` field on the struct `App` defines a frame widget. Frames are used as containers
        // for other widgets. Since the `ui` property on the DSL object `App` corresponds with the
        // `ui` field on the Rust struct `App`, the latter will be initialized from the DSL object
        // here below.
        ui: <Window> {
            pass: {clear_color: #2A}
            block_signal_event: true;
            body = { <AppDesktop> {} }
            // body = { <AppMobile> {} } // Switch to this for Android build. It's manual for now.
        }
    }
}

// This app_main macro generates the code necessary to initialize and run your application.
//
// This code is almost always the same between different applications, so it is convenient to use a
// macro for it. The two main tasks that this code needs to carry out are: initializing both the
// main application struct (`App`) and the global context object (`Cx`), and setting up event
// handling. On desktop, this means creating and running our own event loop. On web, this means
// creating an event handler function that the browser event loop can call into.
app_main!(App);

// The main application struct.
//
// The #[derive(Live, LiveHook)] attribute implements a bunch of traits for this struct that enable
// it to interact with the Makepad runtime. Among other things, this enables the Makepad runtime to
// initialize the struct from a DSL object.
#[derive(Live)]
pub struct App {
    // A chromeless window for our application. Used to contain our frame widget.
    // A frame widget. Used to contain our button and label.
    #[live] ui: WidgetRef,
    #[rust] todos: Vec<TodoItem>,
}

impl App {
    fn fetch_todos(cx: &mut Cx) {
        let server_url = "https://cholee-todo-app.fly.dev/api/todos/".to_string();
        let request_identifier = live_id!(InitialTodoFetch);
        let mut request = HttpRequest::new(server_url, HttpMethod::GET);
        request.set_header("Content-Type".to_string(), "application/json".to_string());
        cx.http_request(request_identifier, request);
    }

    fn save_todo(cx: &mut Cx, todo_label: &String) {
        let server_url = "https://cholee-todo-app.fly.dev/api/todos/".to_string();
        let request_identifier = live_id!(SaveTodo);
        let mut request = HttpRequest::new(server_url, HttpMethod::POST);
        request.set_header("Content-Type".to_string(), "application/json".to_string());

        let body = format!(r#"{{
            "todo": {{
                "text": "{}",
                "done": false
            }}
        }}"#, todo_label);

        request.set_string_body(body);
        cx.http_request(request_identifier, request);
    }

    fn update_todo(cx: &mut Cx, todo_id: u64, todo_done: bool) {
        let server_url = format!(
            "https://cholee-todo-app.fly.dev/api/todos/{}",
            todo_id
        );
        let request_identifier = live_id!(UpdateTodo);
        let mut request = HttpRequest::new(server_url, HttpMethod::PUT);
        request.set_header("Content-Type".to_string(), "application/json".to_string());

        let body = format!(r#"{{
            "todo": {{
                "done": {}
            }}
        }}"#, todo_done);

        request.set_string_body(body);
        cx.http_request(request_identifier, request);
    }
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::todo_list::live_design(cx);
        crate::app_desktop::live_design(cx);
        crate::app_mobile::live_design(cx);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        Self::fetch_todos(cx);
    }
}

impl AppMain for App{
    
    // This function is used to handle any incoming events from the host system. It is called
    // automatically by the code we generated with the call to the macro `main_app` above.
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            // This is a draw event, so create a draw context and use that to draw our application.
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }
        for event in event.network_responses() {
            match &event.response {
                NetworkResponse::HttpResponse(response) => {
                match event.request_id {
                    live_id!(InitialTodoFetch) => {
                        if response.status_code == 200 {
                            let todos_response = response.get_string_body().unwrap();
                            let todos: Value = serde_json::from_str(&todos_response).unwrap();

                            // Only take the first 5 todos for now
                            let todo_items: Vec<TodoItem> = todos["data"]
                                .as_array()
                                .unwrap()
                                .iter().take(10).map({ |todo|
                                    TodoItem {
                                        id: todo["id"].as_u64().unwrap() as u64,
                                        text: todo["text"].as_str().unwrap().to_string(),
                                        done: todo["done"].as_bool().unwrap()
                                    }
                                }).collect();

                            self.todos = todo_items.to_vec();

                            self.ui.redraw(cx);
                        } else {
                            log!("Error loading todos: {:?}", response);
                        }
                    },
                    live_id!(SaveTodo) => {
                        if response.status_code == 201 {
                            let todo_response = response.get_string_body().unwrap();
                            let todo: Value = serde_json::from_str(&todo_response).unwrap();

                            let new_todo = TodoItem {
                                id: todo["data"]["id"].as_u64().unwrap() as u64,
                                text: todo["data"]["text"].as_str().unwrap().to_string(),
                                done: todo["data"]["done"].as_bool().unwrap()
                            };

                            self.todos.push(new_todo);
                            self.ui.redraw(cx);
                        } else {
                            log!("Error saving todo: {:?}", response);
                        }
                    },
                    live_id!(UpdateTodo) => {
                        if response.status_code == 200 {
                            let todo_response = response.get_string_body().unwrap();
                            let todo: Value = serde_json::from_str(&todo_response).unwrap();

                            let updated_todo = TodoItem {
                                id: todo["data"]["id"].as_u64().unwrap() as u64,
                                text: todo["data"]["text"].as_str().unwrap().to_string(),
                                done: todo["data"]["done"].as_bool().unwrap()
                            };

                            if let Some(todo_index) = self.todos.iter().position(|todo| todo.id == updated_todo.id) {
                                self.todos[todo_index] = updated_todo;
                                log!("Updated todo: {:?}", self.todos[todo_index]);

                                self.ui.redraw(cx);
                            }
                        } else {
                            log!("Error updating todo: {:?}", response);
                        }
                    },
                    _ => (),
                }},
                _ => (),
            }
        }

        let mut new_todo:Option<String> = None;
        for widget_action in self.ui.handle_widget_event(cx, event) {
            if let TextInputAction::Return(value) = widget_action.action::<TextInputAction>() {
                if !value.is_empty() {
                    new_todo = Some(value);
                    break
                }
            } else if let CheckBoxAction::Change(value) = widget_action.action::<CheckBoxAction>() {
                let todo_item_id = widget_action.widget_uid.0;
                if let Some(updated_todo_item) = self.todos.iter_mut().find(|todo| todo.id == todo_item_id) {
                    updated_todo_item.done = value;

                    // Save the updated todo to the server
                    Self::update_todo(cx, todo_item_id, value);
                }
            }
        }

        if let Some(new_todo_value) = new_todo {
            let text_input = self.ui.text_input(id!(input));
            text_input.set_text("");

            // This redraw is needed to have this element and the todo list updated upon pressing Enter
            text_input.redraw(cx); 

            // Save the new todo to the server
            Self::save_todo(cx, &new_todo_value);

            // TODO can we display it before waiting for the server (but we would need to generate an id beforehands)
            //self.todos.push(TodoItem{id: ???, text: new_todo_value, done: false});
        }

        if let Some(mut todo_list) = self.ui.widget(id!(todo_list)).borrow_mut::<TodoList>() {
            todo_list.set_todos(self.todos.clone());
        }
    }
}
