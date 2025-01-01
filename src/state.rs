use reqwest;
// set default state on start
pub struct State {
    pub data: String,
    pub exit: bool,
    pub path: String,
    pub url: String,
    //pub method: String,
    pub input_mode: InputMode,
    pub cursor: usize,
}
// set default state
impl Default for State {
    fn default() -> Self {
        State {
            exit: false,
            data: String::new(),
            path: String::from("https://jsonplaceholder.typicode.com/posts/1"),
            url: String::from("https://jsonplaceholder.typicode.com/posts/1"),
            // method: String::new(),
            input_mode: InputMode::Normal,
            cursor: usize::default(),
        }
    }
}

pub enum InputMode {
    Normal,
    Editing,
}
// function to update state
impl State {
    pub fn default() -> Self {
        State {
            exit: false,
            data: String::new(),
            path: String::from("https://jsonplaceholder.typicode.com/posts/1"),
            url: String::from("https://jsonplaceholder.typicode.com/posts/1"),
            //method: String::new(),
            input_mode: InputMode::Normal,
            cursor: usize::default(),
        }
    }
    // set exit for the app
    pub fn exit(&mut self) {
        self.exit = true;
    }
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }
    // set path for the app
    pub fn set_mode(&mut self, input: InputMode) {
        self.input_mode = input;
    }
    // fetch data from the server
    pub fn get_data(&mut self, path: String) -> String {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        // Run the async code using `block_on`
        let response = runtime.block_on(async {
            reqwest::get(path)
                .await // Await the response inside the async block
                .unwrap() // Unwrap the result
        });
        // Print the response as text
        let body = runtime.block_on(response.text()).unwrap();
        self.data = body.clone();
        return body;
    }

    // Allow user to enter the path
    pub fn enter_char(&mut self, c: char) {
        let index: usize = self.byte_index();
        let mut path: String = String::from(&self.path);
        path.insert(index, c);
        self.path = path;
        self.move_cursor_right();
    }
    fn byte_index(&self) -> usize {
        self.path
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor)
            .unwrap_or(self.path.len())
    }
    pub fn clamp_cursor(&self, cursor: usize) -> usize {
        cursor.clamp(0, self.path.chars().count())
    }
    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.path.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.path.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.path = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }
    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor.saturating_sub(1);
        self.cursor = self.clamp_cursor(cursor_moved_left);
    }
    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor.saturating_add(1);
        self.cursor = self.clamp_cursor(cursor_moved_right);
    }
}
