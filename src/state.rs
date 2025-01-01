use reqwest;
// set default state on start
pub struct State {
    pub counter: u8,
    pub data: String,
    pub exit: bool,
    pub path: String,
    pub method: String,
    pub input_mode: InputMode,
}
// set default state
impl Default for State {
    fn default() -> Self {
        State {
            counter: 0,
            exit: false,
            data: String::new(),
            path: String::new(),
            method: String::new(),
            input_mode: InputMode::Normal,
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
            counter: 0,
            exit: false,
            data: String::new(),
            path: String::new(),
            method: String::new(),
            input_mode: InputMode::Normal,
        }
    }
    // increment counter
    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }
    // decrement counter
    pub fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
    // set exit for the app
    pub fn exit(&mut self) {
        self.exit = true;
    }
    // set path for the app
    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }
    // set path for the app
    pub fn set_path(&mut self, path: String) {
        self.path = path;
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
}
