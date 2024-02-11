use serde::{Deserialize, Serialize};

use super::{Comment, Mutation};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ExplainerPage {
    pub id: String,
    pub body: Vec<String>,
    #[serde(default)]
    pub comments: Vec<Comment>,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub mutations: Vec<Mutation>,
}

impl PartialEq for ExplainerPage {
    fn eq(&self, other: &Self) -> bool {
        true
        // if !self.id.eq(&other.id) {
        //     false
        // } else if self.body.len() != other.body.len() {
        //     false
        // } else {
        //     self.comments.eq(&other.comments)
        // }
    }
}

impl ExplainerPage {
    pub fn new(id: String, body: Vec<String>) -> Self {
        Self {
            id,
            body,
            comments: Vec::new(),
            locked: false,
            mutations: Vec::new(),
        }
    }

    pub fn get_body(&self) -> &Vec<String> {
        &self.body
    }

    pub fn filter_out_from_body(&mut self, filter: &str) {
        self.body.retain(|line| !line.contains(filter))
    }

    pub fn filter_out_command(&mut self, command: &str) {
        let has_command =
            |line: &String| line.contains(&format!("<#ffff00>>>> <white>{}", command));
        let mut in_command = false;
        self.body.retain(|line| {
            if has_command(line) {
                in_command = true;
                false
            } else if in_command {
                if line.contains("<#ffff00>>>> <white>") {
                    in_command = false;
                    true
                } else {
                    false
                }
            } else {
                true
            }
        });
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }

    pub fn get_comment(&self, line: usize) -> Option<Comment> {
        self.comments
            .iter()
            .filter(|comment| comment.is_for_line(line))
            .cloned()
            .next()
    }

    pub fn get_comment_lines(&self) -> Vec<usize> {
        self.comments
            .iter()
            .map(|comment| comment.get_line())
            .collect()
    }

    pub fn update_comment(&mut self, line: usize, new_val: String) {
        self.comments
            .iter_mut()
            .filter(|comment| comment.is_for_line(line))
            .next()
            .map(move |comment| comment.update_body(new_val));
    }

    pub fn delete_comment(&mut self, line: usize) {
        self.comments.retain(|comment| !comment.is_for_line(line));
    }
}
