use makepad_widgets::*;

live_design!{
    import todo_makepad::todo_list::TodoList;
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    TITLE_TEXT = {
        font_size: (20),
        font: {path: dep("crate://self/resources/IBMPlexSans-SemiBold.ttf")}
    }
    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/IBMPlexSans-Text.ttf")}
    }

    TodoPrompt = <View> {
        flow: Down,
        spacing: 10,
        width: Fill,
        height: Fit,

        prompt = <Label> {
            // // width: Fill,
            width: 350, 
            height: Fit,
            draw_text: {
                color: #223322,
                text_style: <REGULAR_TEXT>{},
            },
            text: "What is the next to add?"
        }

        input = <TextInput> {
            instance border_width: 1.0,
            width: Fill,
            height: Fit,
            draw_bg: {
                color: #223322
            }
            draw_text: {
                text_style:<REGULAR_TEXT>{},
                color: #x219EBC
            }
            text: "Write here your next task...",
        }
    }

    AppMobile = <View> {
        show_bg: true,
        flow: Down,
        spacing: 50,
        align: {
            x: 0.5,
            y: 0.2
        },
        padding: {left: 30, top: 100},
        width: Fill,
        height: Fill
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
            draw_text: {
                color: #0,
                text_style: <TITLE_TEXT>{},
            },
            text: "My TODO list Mobile"
        }

        todo_prompt = <TodoPrompt> {
            width: Fit,
            height: Fit
        }

        todo_list = <TodoList> {}
    }
}