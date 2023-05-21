pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content = self.state.as_ref().unwrap().add_text(&self.content, text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }

    fn add_text(&self, original_text: &str, _text_to_add: &str) -> String {
        original_text.to_string()
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {..Default::default()})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        // We can't approve a post in Draft state
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn add_text(&self, original_text: &str, text_to_add: &str) -> String {
        format!("{}{}", original_text, text_to_add)
    }
}

struct PendingReview {
    num_reviews: i32,
}

impl Default for PendingReview {
    fn default() -> Self {
        PendingReview { num_reviews: 0 }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        self.num_reviews += 1;
        if self.num_reviews >= 2 {
            Box::new(Published {})
        } else {
            self
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Can't ask for review after publishing
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Can't approve after publishing
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

