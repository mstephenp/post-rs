
use serde::{Deserialize, Serialize};
use yew::{format::{Json, Nothing}, prelude::*, services::fetch::{FetchService, FetchTask, Request, Response}};
// use post_server::CreatePostRequest;


#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    post_id: u64,
    content: String
}

#[derive(Debug)]
pub enum PostMsg {
    GetPosts,
    AddPost(String),
    SetInfo(String),
    ReceiveResponse(Result<Vec<Post>, anyhow::Error>)
}

#[derive(Serialize)]
pub struct CreatePostRequest {
    content: String
}

#[derive(Debug)]
pub struct PostClient {
    fetch_task: Option<FetchTask>,
    posts: Option<Vec<Post>>,
    link: ComponentLink<Self>,
    error: Option<String>,
    info: Option<String>
}

impl PostClient {
    fn view_post_list(&self) -> Html {
        match self.posts {
            Some(ref post_list) => {
                html! {
                    <>
                        <button onclick=self.link.callback(|_| PostMsg::GetPosts)> { "get posts" } </button>
                        <button onclick=self.link.callback(|_| PostMsg::AddPost("adding content".to_string()))> { "add post" } </button>
                        
                        <ul>
                            {
                                post_list.iter().map(|post| html! { 
                                    <p> { format!("{}: {}", post.post_id.clone(), post.content.clone()) } </p>
                                }).collect::<Html>()
                            }
                        </ul>
                    </>
                }
            },
            None => {
                html! {
                    <button onclick=self.link.callback(|_| PostMsg::GetPosts)> { "get posts" } </button>
                }
            },
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { }
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ format!("ERROR: {} ", error.clone()) }</p> }
        } else {
            html! { }
        }
    }

    fn view_info(&self) -> Html {
        if let Some(ref info) = self.info {
            html! { <p>{ format!("INFO: {}", info.clone()) }</p> }
        } else {
            html! { }
        }
    }
}

impl Component for PostClient {
    type Message = PostMsg;

    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            posts: None,
            fetch_task: None,
            link,
            error: None,
            info: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use PostMsg::*;

        match msg {
            GetPosts => {
                let request = Request::get("http://localhost:3000/posts")
                    .body(Nothing)
                    .expect("could not build request");

                let callback = self.link
                    .callback(| response: Response<Json<Result<Vec<Post>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        PostMsg::ReceiveResponse(data)
                    });

                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);

                true
            },
            AddPost(content) => {

                let body = CreatePostRequest {
                    content
                };

                let request = Request::post("http://localhost:3000/addPost")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("could not make request");

                let callback = self.link
                    .callback(| response: Response<Json<Result<u64, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        match data {
                            Ok(id) => {
                                PostMsg::SetInfo(format!("Added new post id {}", id.to_string()))
                            },
                            Err(error) => {
                                PostMsg::SetInfo(error.to_string())
                            }
                        }
                    });

                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);

                true
            },
            ReceiveResponse(response) => {
                match response {
                    Ok(post_list) => {
                        self.posts = Some(post_list)
                    },
                    Err(error) => {
                        self.error = Some(error.to_string())
                    },
                }
                self.fetch_task = None;
                true
            },
            SetInfo(msg) => {
                self.info = Some(msg);
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_fetching() }
                { self.view_post_list() }
                { self.view_error() }
                { self.view_info() }
            </>
        }
    }
}

fn main() {
    yew::start_app::<PostClient>();
}
