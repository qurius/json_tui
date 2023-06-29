use std::vec;
use tui::widgets::{ListState};

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}


pub enum Index {
    Key(String),
}
pub enum Element{
    Bool(Index , serde_json::Value),
    String(Index , serde_json::Value),
    Array(Index , serde_json::Value),
    Object(Index , serde_json::Value),
    Number(Index , serde_json::Value),
    Null(Index),
}

// impl Into for  {
    
// }

// impl Element {
//     fn getSpan(&self) -> Spans {
//         match self {
//             Element::Bool(value) => {

//             Element::String(value) => {
//                 Spans::from(Span::from("Parth"))
//             },
//             Element::Array(value) => {
//                 Spans::from(Span::from("Parth"))
//             },
//             Element::Object(value) => {

//             Element::Number(value) => {
//             },

//         }
//     }
// }

// pub struct Item<'a> {
//     pub pre_format : Vec<Span<'a>>,
//     pub post_format : Vec<Span<'a>>,
//     pub item : &'a str
// }

// fn elementStyle(e : Element ) -> Span {

// }

pub struct App<'a> {
    pub data: &'a str,
    pub tabs: TabsState<'a>,
    pub user_input: String,
    pub input_cursor_position: u16,
    pub json: Option<serde_json::Value>,
    pub navigation_stack: Vec<String>,
    pub elements : Option<StatefulList<Element>>
}

pub struct Route {
    pub route: String,
}

impl<'a> App<'a> {
    pub fn init(data: &str) -> App {
        App {
            data,
            tabs: TabsState::new(vec!["Tab0", "Tab1"]),
            user_input: String::new(),
            input_cursor_position: 0,
            json: None,
            navigation_stack: vec![String::new()], 
            elements : None
        }
    }
    pub fn get_current_route(&self) -> String {
        match self.navigation_stack.last() {
            Some(route) => self.navigation_stack.join("."),
            None => "".to_owned(), // if for some reason there is no route return the default
        }
    }
    pub fn set_json(& mut self,js: Option<serde_json::value::Value>) {
        self.json = js;
    }
    pub fn set_elements(&mut self) -> () {
        let  mut vec_list = Vec::new();
        if self.json.as_ref().unwrap().is_object() {
            self.json
                .as_mut()
                .unwrap()
                .as_object()
                .unwrap()
                .iter()
                .for_each(|(f,j)| {
                    vec_list.push(
                        
                            if j.is_array() {
                                Element::Array(Index::Key(String::from(f)),j.to_owned())
                            } else if j.is_object() {
                                Element::Object(Index::Key(String::from(f)),j.to_owned())
                            } else if j.is_boolean() {
                                Element::Bool(Index::Key(String::from(f)),j.to_owned())
                            } else if j.is_string() {
                                Element::String(Index::Key(String::from(f)),j.to_owned())
                            } else if j.is_number() {
                                Element::Number(Index::Key(String::from(f)),j.to_owned())
                            } else {
                                Element::Null(Index::Key(String::from(f)))
                            }
                    )
                });
        }else {
            for (k,j) in self.json.as_ref().unwrap().as_array().unwrap().iter().enumerate() {
                vec_list.push( if j.is_array() {
                    Element::Array(Index::Key(String::from(k.to_string())),j.to_owned())
                } else if j.is_object() {
                    Element::Object(Index::Key(String::from(k.to_string())),j.to_owned())
                } else if j.is_boolean() {
                    Element::Bool(Index::Key(String::from(k.to_string())),j.to_owned())
                } else if j.is_string() {
                    Element::String(Index::Key(String::from(k.to_string())),j.to_owned())
                } else if j.is_number() {
                    Element::Number(Index::Key(String::from(k.to_string())),j.to_owned())
                } else {
                    Element::Null(Index::Key(String::from(k.to_string())))
                });
            }
        }
        // self.elements = Some(StatefulList::with_items(vec_list));
        self.elements = Some(StatefulList::with_items(vec_list));
    }
}
    