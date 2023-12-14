use makepad_widgets::*;
use crate::todo_item::TodoItem;

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    TodoList = {{TodoList}} {
        flow: Down,
        spacing: 10,
        width: Fill,
        height: Fit,
        checkbox: <CheckBox> {
            draw_check: {
                instance border_width: 1.0
                instance border_color: #223322
                instance border_color2: #229999
                size: 10.0,
            }

            draw_text: {
                text_style: {font_size: (12)},
                fn get_color(self) -> vec4 {
                    return mix(
                        (#333333),
                        (#111111),
                        self.selected
                    )
                }
            }

            label_walk: {margin: {left: 30.0}}
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum TodoUpdateAction {
    Changed(u64, bool),
    None,
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct CheckBoxId(pub LiveId);

#[derive(Live, LiveHook, Widget)]
pub struct TodoList {
    // It is mandatory to list here all the fields that are part of the live design block.
    // You need to annotate them with `#[live]`
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    #[rust] #[redraw] area: Area,

    // This is also refered in the live design block, but it is not meant to be rendered automatically.
    // This is like a template element, that is used to create concrete instances that are
    // rendered by the draw_walk function, depending on the list of TODOs.
    #[live] checkbox: Option<LivePtr>,

    // The "rust" attribute is used to indicate there is no field with those names in the
    // "live design" definitions. Those fields are used in our own Rust code.
    #[rust] todos: Vec<TodoItem>,
    #[rust] items: ComponentMap<CheckBoxId, CheckBoxRef>
}

impl Widget for TodoList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let widget_uid = self.widget_uid();
        for (id, item) in self.items.iter_mut() {
            // This id is actually the item id set by the server.
            let item_uid = id.0.0;

            if let Some(mut inner) = item.borrow_mut(){
                for action in cx.capture_actions(|cx| inner.handle_event(cx, event, scope)) {
                    if let CheckBoxAction::Change(value) = action.as_widget_action().cast() {
                        cx.widget_action(widget_uid, &scope.path, TodoUpdateAction::Changed(item_uid, value));
                    }
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // This was needed to apply the layout defined for TodoList in the live design block.
        // Otherwise, it is ignored.
        cx.begin_turtle(walk, self.layout);

        for (_id, value) in self.todos.iter().enumerate() {
            let widget_id = LiveId(value.id).into();
            let current_checkbox = self.items.get_or_insert(cx, widget_id, | cx | {
                let widget_ref = WidgetRef::new_from_ptr(cx, self.checkbox);
                widget_ref.as_check_box()
            });
            current_checkbox.set_text(&value.text);
            current_checkbox.set_selected(cx, value.done);
            let _ = current_checkbox.draw_all(cx, scope);
        }

        cx.end_turtle();
        self.items.retain_visible();

        DrawStep::done()
    }
}

impl TodoListRef {
    pub fn set_todos(&mut self, todos: Vec<TodoItem>) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.todos = todos;
        }
    }
}