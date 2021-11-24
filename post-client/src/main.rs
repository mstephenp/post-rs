use post_lib::{CreatePostRequest, Post};

use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};

#[derive(Debug)]
pub enum PostMsg {
    GetPosts,
    AddPost(String),
    SetInfo(String),
    RemovePost(u64),
    ReceiveResponse(Result<Vec<Post>, anyhow::Error>),
}

#[derive(Debug)]
pub struct PostClient {
    fetch_task: Option<FetchTask>,
    posts: Option<Vec<Post>>,
    link: ComponentLink<Self>,
    error: Option<String>,
    info: Option<String>,
}

impl PostClient {
    fn view_post_list(&self) -> Html {
        match self.posts {
            Some(ref post_list) => {

                let add_post_callback = 
                    self.link.callback(|event: ChangeData|
                        if let ChangeData::Value(content) = event {
                            if !content.is_empty() {
                                PostMsg::AddPost(content)
                            } else {
                                PostMsg::SetInfo("empty post added".to_string())
                            }
                        } else {
                            PostMsg::SetInfo("could not get content from ChangeData".to_string())
                        }
                    );

                let delete_post_callback = |id: u64| 
                    self.link.callback(move |_| PostMsg::RemovePost(id));

                html! {
                    <div class="main">
                        <div class="flex three grow">
                            <div>
                                <button class="success"
                                    onclick=self.link.callback(|_| PostMsg::GetPosts)> 
                                    { "get posts" } 
                                </button>
                            </div>
                            <div>
                                <label for="addPost">{ "Add New Post" }</label>
                                <input id="addPost" type="text" onchange={add_post_callback}/>
                            </div>
                        </div>
                        <div class="flex">
                            <ul>
                                {
                                    post_list.iter().map(|post| html! {
                                        <div>
                                            <span>{ format!("{}: {}", post.post_id.clone(), post.content.clone()) }</span>
                                            <button class="warning" onclick={delete_post_callback(post.post_id)}>{"delete post"}</button>
                                        </div>
                                    }).collect::<Html>()
                                }
                            </ul>
                        </div>
                    </div>
                }
            }
            None => {
                html! {
                    <div class="main">
                        <button class="success" 
                            onclick=self.link.callback(|_| PostMsg::GetPosts)> 
                            { "get posts" } 
                        </button>
                    </div>
                }
            }
        }
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! {}
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ format!("ERROR: {} ", error.clone()) }</p> }
        } else {
            html! {}
        }
    }

    fn view_info(&self) -> Html {
        if let Some(ref info) = self.info {
            html! { <p>{ format!("INFO: {}", info.clone()) }</p> }
        } else {
            html! {}
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
            info: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use PostMsg::*;

        match msg {
            GetPosts => {
                let request = Request::get("http://localhost:3000/posts")
                    .body(Nothing)
                    .expect("could not build request");

                let callback = self.link.callback(
                    |response: Response<Json<Result<Vec<Post>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        PostMsg::ReceiveResponse(data)
                    },
                );

                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);

                true
            }
            AddPost(content) => {
                let body = CreatePostRequest { content };

                let request = Request::post("http://localhost:3000/addPost")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("could not make request");

                let callback =
                    self.link
                        .callback(|response: Response<Json<Result<u64, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            match data {
                                Ok(id) => PostMsg::SetInfo(format!(
                                    "Added new post id {}",
                                    id.to_string()
                                )),
                                Err(error) => PostMsg::SetInfo(error.to_string()),
                            }
                        });

                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);

                true
            }
            RemovePost(post_id) => {
                let request = Request::post(format!("http://localhost:3000/deletePost/{}", post_id))
                    .body(Nothing)    
                    .expect("could not make delete request");

                let callback = 
                    self.link
                        .callback(|response: Response<Json<Result<u64, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            match data {
                                Ok(id) => PostMsg::SetInfo(format!(
                                    "Deleted Post id {}", 
                                    id.to_string()
                                )),
                                Err(error) => PostMsg::SetInfo(format!("ERROR! {}", error.to_string()))
                            }
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");

                self.fetch_task = Some(task);
                
                true
            },
            ReceiveResponse(response) => {
                match response {
                    Ok(post_list) => self.posts = Some(post_list),
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
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
                { self.view_post_list() }
                { self.view_fetching() }
                { self.view_error() }
                { self.view_info() }
            </>
        }
    }
}

fn main() {
    yew::start_app::<PostClient>();
}
