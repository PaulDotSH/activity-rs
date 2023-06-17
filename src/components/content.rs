#![allow(non_snake_case)]
use dioxus::prelude::*;


pub fn Content(cx: Scope) -> Element {
    let mut teams = use_state(cx, || Vec::<&str>::new());
    let mut timer = use_state(cx, || 60);


    cx.render(rsx! {
        main {
            form {
                prevent_default: "onsubmit",
                onsubmit: move |event| {
                    let name = event.values.clone();
                    println!("{:?}", name);
                },

                div {
                    label {
                        r#for: "explain",
                        "Explain"
                    }

                    input {
                        r#type: "checkbox",
                        id: "explain",
                    }
                }

                div {
                    label {
                        r#for: "mime",
                        "Mime"
                    }

                    input {
                        r#type: "checkbox",
                        id: "mime",
                    }
                }

                div {
                    label {
                        r#for: "draw",
                        "Draw"
                    }

                    input {
                        r#type: "checkbox",
                        id: "draw",
                    }
                }
    
                button {
                    r#type: "submit",
                    "Start Activity"
                }
            }



            form {
                label {
                    "Timer: {timer}"
                }

                input {
                    onchange: move |event| {
                        timer.set(event.data.value.parse::<i32>().unwrap());
                    },
                    r#type: "range",
                    min: 10,
                    max: 130,
                    value: "{timer}"
                }

                button {
                    r#type: "submit",
                    "Start Timer"
                }

                
                button {
                    r#type: "submit",
                    "Stop Timer"
                }
            }


            form {
                prevent_default: "onsubmit",
                onsubmit: move |event| {
                    let name = event.data;
                    println!("{:?}", name);

                    let mut newTeams: Vec<&str> = teams.to_vec().clone();
                    newTeams.push("fds");
                    teams.set(newTeams);
                },


                label {
                    r#for: "teamname",
                    "Add Team"
                }

                input {
                    name: "teamname",
                    r#type: "text",
                    id: "teamname"
                }
                
                button {
                    r#type: "submit",
                    "Add Team"
                }
            }


            for team in teams.iter() {
                div {
                    "{team}"
                }
            }
        }
    })
}