use axum::{routing::get, Router};
use maud::{html, Markup, PreEscaped, DOCTYPE};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8888").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title { "Example" }
                script
                    src="https://unpkg.com/htmx.org@2.0.3"
                    integrity="sha384-0895/pl2MU10Hqc6jd4RvrthNlDiE9U1tWmX7WRESftEDRosgxNsQG/Ze9YMRzHq"
                    crossorigin="anonymous" {}
                style { (PreEscaped(include_str!("style.css"))) }
            }
            body {
                h1 { "Ultimate Connect 4" }
                div class="boards" {
                    div class="alpha-boards" {
                        @for _alpha in 0..7 {
                            div class="board" {
                                @for col in 0..7 {
                                    div class="board-col" {
                                        @for row in 0..6 {
                                            button class="square" { }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div class="board" {
                        @for col in 0..7 {
                            div class="board-col" {
                                @for row in 0..6 {
                                    button class="square" { }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
