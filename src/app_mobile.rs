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
        font_size: (40),
        font: {path: dep("crate://self/resources/IBMPlexSans-SemiBold.ttf")}
    }
    REGULAR_TEXT = {
        font_size: (20),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
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

    AppMobile = <Frame> {
        show_bg: true,
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
                return mix(#00aab1, #0, self.geom_pos.x / 3);
            }
        }
        // A label to display the counter.
        title = <Label> {
            draw_label: {
                color: #4,
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