use makepad_widgets::*;
use serde_json::{json, Result, Value};
use makepad_platform::network::*;

// The live_design macro generates a function that registers a DSL code block with the global
// context object (`Cx`).
//
// DSL code blocks are used in Makepad to facilitate live design. A DSL code block defines
// structured data that describes the styling of the UI. The Makepad runtime automatically
// initializes widgets from their corresponding DSL objects. Moreover, external programs (such
// as a code editor) can notify the Makepad runtime that a DSL code block has been changed, allowing
// the runtime to automatically update the affected widgets.
live_design!{
    import makepad_draw::shader::std::*;

    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::frame::Frame;
    import makepad_widgets::frame::Image;
    import makepad_widgets::label::Label;
    import makepad_widgets::check_box::CheckBox;
    import makepad_widgets::text_input::TextInput;

    TITLE_TEXT = {
        font_size: (40),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
    }
    REGULAR_TEXT = {
        font_size: (20),
        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-Text.ttf")}
    }

    TodoPrompt = <Frame> {
        layout: {
            flow: Down,
            spacing: 10,
        },

        prompt = <Label> {
            draw_label: {
                color: #0,
                text_style: <REGULAR_TEXT>{},
            },
            label: "What is the next to add?"
        }

        input = <TextInput> {
            instance border_width: 1.0,
            walk: {width: 800, height: 40},
            draw_bg: {
                color: #223322
            }
            draw_label: {
                text_style:<REGULAR_TEXT>{font_size: (16)},
                color: #aaaaaa
            }
            text: "Write here your next task...",
        }
    }

    TodoList = {{TodoList}} {
        layout: {
            flow: Down,
            spacing: 10,
            padding: {left: 100}
        },
        walk: {width: Fill, height: Fit},
        checkbox: <CheckBox> {
            draw_check: {
                instance border_width: 1.0
                instance border_color: #223322
                instance border_color2: #229999
                size: 20.0,
            }
    
            draw_label: {
                text_style: <REGULAR_TEXT>{},
                fn get_color(self) -> vec4 {
                    return mix(
                        (#333333),
                        (#111111),
                        self.selected
                    )
                }
            }
    
            walk: {margin: {left: 50.0}, width: 800},
            label_walk: {margin: {left: 50.0}, width: 800} 
        }
    }

    // The `{{App}}` syntax is used to inherit a DSL object from a Rust struct. This tells the
    // Makepad runtime that our DSL object corresponds to a Rust struct named `App`. Whenever an
    // instance of `App` is initialized, the Makepad runtime will obtain its initial values from
    // this DSL object.
    App = {{App}} {
        // The `ui` field on the struct `App` defines a frame widget. Frames are used as containers
        // for other widgets. Since the `ui` property on the DSL object `App` corresponds with the
        // `ui` field on the Rust struct `App`, the latter will be initialized from the DSL object
        // here below.
        ui: <DesktopWindow>{<Frame>{
            show_bg: true
            layout: {
                flow: Down,
                spacing: 100,
                align: {
                    x: 0.5,
                    y: 0.2
                }
            },
            // The `walk` property determines how the frame widget itself is laid out. In this
            // case, the frame widget takes up the entire window.
            walk: {
                width: Fill,
                height: Fill
            },
            draw_bg: {
                fn pixel(self) -> vec4 {
                    // Gradient color effect starting from a yellow tone
                    // The final color would be black, however the x value is divided to 3
                    // so the color gets darker slower.
                    return mix(#b0d1b1, #0, self.geom_pos.x / 3);
                }
            }
            // A label to display the counter.
            title = <Label> {
                draw_label: {
                    color: #0,
                    text_style: <TITLE_TEXT>{},
                },
                label: "My TODO list"
            }

            todo_prompt = <TodoPrompt> {
                walk: {width: Fit, height: Fit}
            }

            todo_list = <TodoList> {}
        }}
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

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct CheckBoxId(pub LiveId);

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

    #[rust] todos: Vec<String>,

    #[rust] todo_load_request_id: Option<LiveId>,
}

impl App {
    fn load_todos(cx: &mut Cx) -> LiveId {
        let server_url = "https://cholee-todo-app.fly.dev/api/todos/".to_string();
        let id = LiveId::unique();
        let mut request = HttpRequest::new(id, server_url, Method::GET);
        request.set_header("Content-Type".to_string(), "application/json".to_string());
        cx.http_request(request);

        id
    }

    fn save_todo(cx: &mut Cx, todo_label: &String) -> LiveId {
        let server_url = "https://cholee-todo-app.fly.dev/api/todos/".to_string();
        let id = LiveId::unique();
        let mut request = HttpRequest::new(id, server_url, Method::POST);
        request.set_header("Content-Type".to_string(), "application/json".to_string());

        let body = format!(r#"{{
            "todo": {{
                "text": "{}",
                "done": false
            }}
        }}"#, todo_label);

        request.set_string_body(body);
        cx.http_request(request);

        id
    }
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
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

        // Retrieve todos only when the app is loaded
        if self.todo_load_request_id.is_none() {
            self.todo_load_request_id = Some(Self::load_todos(cx));
        }

        if let Event::HttpResponse(event) = event {  
            log!("Error loading todos: {:?}", event.response.status_code);
            if let Some(todo_load_request_id) = self.todo_load_request_id {
                // Check the request id to make sure we are handling the correct response
                // TODO event.response.id maybe must be renamed to event.response.requets_id
                if event.response.id != todo_load_request_id {
                    if event.response.status_code != 200 {
                        log!("Error loading todos: {:?}", event.response);
                    } else {
                        let todos_response = event.response.get_string_body().unwrap();
                        let todos: Value = serde_json::from_str(&todos_response).unwrap();

                        // Only take the first 3 todos for now
                        let labels: Vec<String> = todos["data"]
                            .as_array()
                            .unwrap()
                            .iter().take(3).map({ |todo|
                                todo["text"].as_str().unwrap().to_string()
                            }).collect();

                        self.todos = labels.to_vec();

                        self.ui.redraw(cx);
                    }
                }
            }
        }

        let mut new_todo:Option<String> = None;
        for widget_action in self.ui.handle_widget_event(cx, event) {
            if let TextInputAction::Return(value) = widget_action.action::<TextInputAction>() {
                if !value.is_empty() {
                    new_todo = Some(value);
                    break
                }
            }
        }

        if let Some(new_todo_value) = new_todo {
            let text_input = self.ui.get_text_input(id!(input));
            text_input.set_text("");

            // This redraw is needed to have this element and the todo list updated upon pressing Enter
            text_input.redraw(cx); 

            // Save the new todo to the server
            Self::save_todo(cx, &new_todo_value);

            // Add the new todo to the locally-stored list
            self.todos.push(new_todo_value);
        }

        if let Some(mut todo_list) = self.ui.get_widget(id!(todo_list)).borrow_mut::<TodoList>() {
            todo_list.set_todos(self.todos.clone());
        }
    }
}

#[derive(Live)]
pub struct TodoList {
    // It is mandatory to list here all the fields that are part of the live design block.
    // You need to annotate them with `#[live]`
    #[live] walk: Walk,
    #[live] layout: Layout,

    // This is also refered in the live design block, but it is not meant to be rendered automatically.
    // This is like a template element, that is used to create concrete instances that are
    // rendered by the draw_walk function, depending on the list of TODOs.
    #[live] checkbox: Option<LivePtr>,

    // The "rust" attribute is used to indicate there is no field with those names in the
    // "live design" definitions. Those fields are used in our own Rust code.
    #[rust] todos: Vec<String>,
    #[rust] items: ComponentMap<CheckBoxId, CheckBoxRef>
}

impl LiveHook for TodoList {
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx, TodoList)
    }
}

impl Widget for TodoList {  
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem)
    ) {
        self.handle_event_with(cx, event, &mut | cx, action | {
            dispatch_action(cx, action);
        });
    }

    fn get_walk(&self)->Walk{ self.walk }
    
    fn redraw(&mut self, _cx:&mut Cx){
        // Not sure how I can implement this method if I don't have an Area
        // (which is what I see in many examples).
        // I don't know what is an Area used for.

        //self.area.redraw(cx)
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl TodoList {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        for (_id, item) in self.items.iter_mut() {
            let item_uid = item.widget_uid();
            if let Some(mut inner) = item.borrow_mut(){
                inner.handle_event_with(cx, event, &mut | cx_imm, action | {
                    dispatch_action(cx_imm, WidgetActionItem::new(action.into(), item_uid));
                });
            }
        }
    }

    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        // This was needed to apply the layout defined for TodoList in the live design block.
        // Otherwise, it is ignored.
        cx.begin_turtle(walk, self.layout);

        for (i, value) in self.todos.iter().enumerate() {
            let item_id = LiveId(i as u64).into();
            let current_checkbox = self.items.get_or_insert(cx, item_id, | cx | {
                CheckBoxRef::new_from_ptr(cx, self.checkbox)
            });
            
            current_checkbox.set_label_text(value);
            let _ = current_checkbox.draw_walk_widget(cx, walk);
        }

        cx.end_turtle();
        self.items.retain_visible();
    }

    pub fn set_todos(&mut self, todos: Vec<String>) {
        self.todos = todos
    }
}