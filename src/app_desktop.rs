use makepad_widgets::*;

live_design!{
    import todo_makepad::todo_list::TodoList;
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    TITLE_TEXT = {
        font_size: (40),
        font: {path: dep("crate://self/resources/IBMPlexSans-SemiBold.ttf")}
    }

    TodoPrompt = <View> {
        flow: Down,
        spacing: 10,

        prompt = <Label> {
            draw_text: {
                color: #0,
                text_style: {font_size: (20)},
            },
            text: "What is the next to add?"
        }

        input = <TextInput> {
            instance border_width: 1.0,
            width: 800,
            height: 40,
            draw_bg: {
                color: #223322
            }
            draw_text: {
                text_style: {font_size: (16)},
                color: #aaaaaa
            }
            text: "Write here your next task...",
        }
    }

    AppDesktop = <View> {
        show_bg: true,
        flow: Down,
        spacing: 60,
        align: {
            x: 0.5,
            y: 0.2
        },
        padding: {top: 20},
        width: Fill,
        height: Fill,
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
            draw_text: {
                color: #0,
                text_style: <TITLE_TEXT>{},
            },
            text: "My TODO list"
        }

        todo_prompt = <TodoPrompt> {
            width: Fit,
            height: Fit
        }

        todo_list = <TodoList> {
            width: 800
        }
    }
}