use makepad_widgets::*;

live_design!{
    import todo_makepad::todo_list::TodoList;
    import makepad_draw::shader::std::*;
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::frame::Frame;
    import makepad_widgets::frame::Image;
    import makepad_widgets::label::Label;
    import makepad_widgets::text_input::TextInput;

    TITLE_TEXT = {
        font_size: (48),
        font: {path: dep("crate://self/resources/IBMPlexSans-SemiBold.ttf")}
    }
    REGULAR_TEXT = {
        font_size: (30),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
    }

    TodoPrompt = <Frame> {
        layout: {
            flow: Down,
            spacing: 10,
        },
        walk: { width: Fill, height: Fit },

        prompt = <Label> {
            walk: { width: 500, height: Fit },
            draw_label: {
                color: #x223322,
                text_style: <REGULAR_TEXT>{},
            },
            label: "What is the next to add?"
        }

        input = <TextInput> {
            instance border_width: 1.0,
            walk: { width: Fill, height: Fit },
            draw_bg: {
                color: #x223322
            }
            draw_label: {
                text_style:<REGULAR_TEXT>{font_size: (22)},
                color: #x219EBC
            }
            text: "Write here your next task...",
        }
    }

    AppMobile = <Frame> {
        show_bg: true,
        layout: {
            flow: Down,
            spacing: 50,
            align: {
                x: 0,
                y: 0
            },
            padding: {left: 50, top: 150, right: 50, bottom: 50},
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
                return mix(#x8ecae6, #0, self.geom_pos.x / 3);
            }
        }
        // A label to display the counter.
        title = <Label> {
            draw_label: {
                color: #x023047,
                text_style: <TITLE_TEXT>{},
            },
            label: "My TODO list Mobile"
        }

        todo_prompt = <TodoPrompt> {
            walk: {width: Fit, height: Fit}
        }

        todo_list = <TodoList> {}
    }
}