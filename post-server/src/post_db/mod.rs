//! Post Database Module
//!
//! this is a simple container for posts

use serde::Serialize;

/// Post struct
#[derive(Serialize, Clone)]
pub struct Post {
    post_id: u64,
    content: String,
}

/// PostDb struct - just a list of Posts
///
/// Example:
/// ```
/// use post_server::{Post, PostDb, PostDbStatus};
///
/// let mut db = PostDb::new();
///
/// let result = db.get_posts();
/// assert!(result.len() == 0);
///
/// let result = db.create_post("some content".to_string());
/// assert!(result.status == PostDbStatus::Ok);
/// assert!(result.value == 1);
///
/// let result = db.get_post(1);
/// assert!(result.status == PostDbStatus::Ok);
/// ```
pub struct PostDb {
    pub posts: Vec<Post>,
}

/// Status returned as part of the response
#[derive(Serialize, PartialEq, Debug)]
pub enum PostDbStatus {
    Ok,
    Err,
}

/// Response that contains the status and any returned values
#[derive(Serialize)]
pub struct PostDbResponse<T> {
    pub status: PostDbStatus,
    pub value: T,
}

/// PostDb default implementation
impl Default for PostDb {
    fn default() -> Self {
        Self::new()
    }
}

/// PostDb implementation
impl PostDb {
    pub fn new() -> Self {
        PostDb { posts: vec![] }
    }

    /// return all posts from the database
    pub fn get_posts(&self) -> Vec<Post> {
        self.posts.clone()
    }

    /// create a new post
    pub fn create_post(&mut self, content: String) -> PostDbResponse<u64> {
        let id: u64 = self.get_post_id((self.posts.len() + 1).try_into().unwrap());
        let post = Post {
            content,
            post_id: id,
        };

        self.posts.push(post);
        PostDbResponse {
            status: PostDbStatus::Ok,
            value: id,
        }
    }

    /// get a post by id
    pub fn get_post(&self, id: u64) -> PostDbResponse<Option<Post>> {
        for post in self.posts.clone().into_iter() {
            if post.post_id == id {
                return PostDbResponse {
                    status: PostDbStatus::Ok,
                    value: Some(post),
                };
            }
        }
        PostDbResponse {
            status: PostDbStatus::Err,
            value: None,
        }
    }

    /// delete a post by id
    pub fn delete_post(&mut self, id: u64) -> PostDbResponse<Option<u64>> {
        for (post_index, post) in self.posts.clone().into_iter().enumerate() {
            if post.post_id == id {
                let found_post = self.posts.remove(post_index);
                return PostDbResponse {
                    status: PostDbStatus::Ok,
                    value: Some(found_post.post_id),
                };
            }
        }
        PostDbResponse {
            status: PostDbStatus::Err,
            value: None,
        }
    }

    /// update a post by id with updated content
    pub fn update_post(&mut self, id: u64, updated_content: String) -> PostDbResponse<Option<u64>> {
        for (index, post) in self.posts.clone().iter_mut().enumerate() {
            if post.post_id == id {
                self.posts[index].content = updated_content;
                return PostDbResponse {
                    status: PostDbStatus::Ok,
                    value: Some(id),
                };
            }
        }
        PostDbResponse {
            status: PostDbStatus::Err,
            value: None,
        }
    }

    /// get the next available post id
    /// based on the number of posts
    fn get_post_id(&self, id: u64) -> u64 {
        for post in self.posts.clone().into_iter() {
            if post.post_id == id {
                return self.get_post_id(id + 1);
            }
        }
        id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_all_posts() {
        let mut db = PostDb::new();
        assert!(db.posts.is_empty());

        let response1 = db.create_post("post content".to_string());
        let response2 = db.create_post("different post content".to_string());

        assert_eq!(PostDbStatus::Ok, response1.status);
        assert_eq!(PostDbStatus::Ok, response2.status);

        assert!(!db.posts.is_empty());
        assert!(db.posts.len() == 2);
    }

    #[test]
    fn add_post() {
        let mut db = PostDb::new();
        assert!(db.posts.is_empty());

        let response = db.create_post("post content".to_string());

        assert_eq!(PostDbStatus::Ok, response.status);
        assert!(db.posts.len() == 1);
    }

    #[test]
    fn get_post_by_id() {
        let mut db = PostDb::new();
        assert!(db.posts.is_empty());

        let response = db.create_post("post content".to_string());
        assert_eq!(PostDbStatus::Ok, response.status);
        assert_eq!(response.value, 1);
        let response = db.get_post(1);
        assert_eq!(PostDbStatus::Ok, response.status);
        if let Some(post) = response.value {
            assert_eq!("post content", post.content);
        }

        let response = db.create_post("post content 2".to_string());
        assert_eq!(PostDbStatus::Ok, response.status);
        assert_eq!(response.value, 2);
        let response = db.get_post(2);
        assert_eq!(PostDbStatus::Ok, response.status);
        if let Some(post) = response.value {
            assert_eq!("post content 2", post.content);
        }
    }

    #[test]
    fn update_post() {
        let mut db = PostDb::new();
        assert!(db.posts.is_empty());

        let response = db.create_post("post content".to_string());
        assert_eq!(PostDbStatus::Ok, response.status);
        let created_post_id = response.value;
        assert_eq!(created_post_id, 1);

        let response = db.update_post(created_post_id, "post content updated".to_string());
        assert_eq!(PostDbStatus::Ok, response.status);
        if let Some(updated_post_id) = response.value {
            assert_eq!(created_post_id, updated_post_id);
        }
    }

    #[test]
    fn delete_post() {
        let mut db = PostDb::new();
        assert!(db.posts.is_empty());

        let response = db.create_post("post content".to_string());
        assert_eq!(PostDbStatus::Ok, response.status);
        let created_post_id = response.value;
        assert_eq!(created_post_id, 1);

        let response = db.delete_post(created_post_id);
        assert_eq!(PostDbStatus::Ok, response.status);
        if let Some(removed_id) = response.value {
            assert_eq!(1, removed_id);
        }
    }
}
