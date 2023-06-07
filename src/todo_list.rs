use makepad_widgets::*;
use crate::todo_item::TodoItem;

live_design!{
    import makepad_widgets::check_box::CheckBox;

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
    }

    TodoList = {{TodoList}} {
        layout: {
            flow: Down,
            spacing: 10,
        },
        walk: {width: Fill, height: Fit},
        checkbox: <CheckBox> {
            draw_check: {
                instance border_width: 1.0
                instance border_color: #223322
                instance border_color2: #229999
                size: 10.0,
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

            label_walk: {margin: {left: 30.0}}
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct CheckBoxId(pub LiveId);

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
    #[rust] todos: Vec<TodoItem>,
    #[rust] items: ComponentMap<CheckBoxId, CheckBoxRef>
}

impl LiveHook for TodoList {
    fn before_live_design(cx:&mut Cx){
        register_widget!(cx, TodoList);
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
        for (id, item) in self.items.iter_mut() {
            // This id is actually the item id set by the server.
            let item_uid = WidgetUid(id.0.0);

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

        for (_id, value) in self.todos.iter().enumerate() {
            let widget_id = LiveId(value.id).into();
            let current_checkbox = self.items.get_or_insert(cx, widget_id, | cx | {
                CheckBoxRef::new_from_ptr(cx, self.checkbox)
            });

            current_checkbox.set_label_text(&value.text);
            current_checkbox.set_selected(cx, value.done);
            let _ = current_checkbox.draw_walk_widget(cx, walk);
        }

        cx.end_turtle();
        self.items.retain_visible();
    }

    pub fn set_todos(&mut self, todos: Vec<TodoItem>) {
        self.todos = todos
    }
}