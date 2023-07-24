use serde_json::{Value, Map};
use std::vec;
use std::io::Cursor;
use tui::widgets::ListState;
use skim::prelude::*;

// use rayon::prelude::*;
// use dashmap::DashMap;
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Route {
    Search,
    Main
}
#[derive(Debug)]
pub enum Index {
    Key(String),
}
#[derive(Debug)]
pub enum Element {
    Bool(Index, serde_json::Value),
    String(Index, serde_json::Value),
    Array(Index, serde_json::Value),
    Object(Index, serde_json::Value),
    Number(Index, serde_json::Value),
    Null(Index),
}
pub struct App<'a> {
    pub data: &'a str,
    pub tabs: TabsState<'a>,
    pub user_input: String,
    pub input_cursor_position: u16,
    pub json: Option<serde_json::Value>,
    pub navigation_stack: Vec<String>,
    pub elements: Option<StatefulList<Element>>,
    pub current_route : Route
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
            elements: None,
            current_route : Route::Main,
        }
    }
    pub fn get_current_navigation_stack(&self) -> String {
        match self.navigation_stack.last() {
            Some(_) => self.navigation_stack.join("."),
            None => "".to_owned(), // if for some reason there is no route return the default
        }
    }
    pub fn get_current_route(&self) -> Route {
        self.current_route
    }
    pub fn set_current_route(&mut self) -> () {
        self.current_route = Route::Search
    }
    pub fn set_json(&mut self, js: Option<serde_json::value::Value>) {
        self.json = js;
    }
    pub fn set_elements(&mut self) -> () {
        let mut vec_list = Vec::new();
        let mut js;
        if self.navigation_stack.len() > 0 {
            let s: String = self.navigation_stack.join("/");
            js = self.json.as_ref().unwrap().pointer(&s);
        } else {
            js = self.json.as_ref();
        }

        // let dashmap: DashMap<String, Value> = DashMap::new();
        if js.as_ref().unwrap().is_object() {
            js.as_mut()
                .unwrap()
                .as_object()
                .unwrap()
                .iter()
                .for_each(|(f, j)| {
                    // dashmap.insert(f.clone(), j.clone());
                    vec_list.push(get_element(f, j))
                });
        } else {
            for (k, j) in js.as_ref().unwrap().as_array().unwrap().iter().enumerate() {
                // dashmap.insert(k.to_string(), j.clone());
                vec_list.push(get_element(&k.to_string(), j));
            }
        }

        // dashmap.into_read_only().iter().filter_map(|key, val| )
        // panic!("The value of dashmap is {:#?}",dashmap);
        self.elements = Some(StatefulList::with_items(vec_list));
    }
    pub fn set_route(&mut self) -> () {
        let list_state = &self.elements.as_ref().unwrap().state;
        let selected = list_state.selected().unwrap();
        let element = self.elements.as_ref().unwrap().items.get(selected);

        // panic!("Selected Element is {:#?}",element);

        match element {
            Some(e) => match e {
                Element::Array(k, _v) => match k {
                    Index::Key(key) => {
                        self.navigation_stack.push(key.to_owned());
                        // panic!("Selected Vec is {:#?}",self.navigation_stack);
                    }
                },
                Element::Object(k, _v) => match k {
                    Index::Key(key) => self.navigation_stack.push(key.to_owned()),
                },
                _ => {}
            },
            _ => {}
        }
    }
    pub fn pop_route(&mut self) -> () {
        self.navigation_stack.pop();
    }
    pub fn set_fuzzy_elements(&mut self) {
        // borrows json
        // sets josn as 'pointer : item'
        // ex 'x/0/whatever : orange'
        let mut vec : String = String::new();
        let mut fuzzy_data = Vec::new();
        if self.json.as_ref().unwrap().is_object() {
            get_pointer_object(self.json.as_ref().unwrap().as_object().unwrap(), &mut vec, &mut fuzzy_data);
        }else if self.json.as_ref().unwrap().is_array() {
            get_pointer_array(self.json.as_ref().unwrap().as_array().unwrap(), &mut vec, &mut fuzzy_data);
        }

        eprintln!("fuzzy data is {:#?}", fuzzy_data);

        panic!("fuzzy data is {:#?}", fuzzy_data);

    }

    // self.elements = Some(StatefulList::with_items(vec_list));
}
fn get_element(f: &String, j: &Value) -> Element {
    if j.is_array() {
        Element::Array(Index::Key(String::from(f.to_string())), j.to_owned())
    } else if j.is_object() {
        Element::Object(Index::Key(String::from(f.to_string())), j.to_owned())
    } else if j.is_boolean() {
        Element::Bool(Index::Key(String::from(f.to_string())), j.to_owned())
    } else if j.is_string() {
        Element::String(Index::Key(String::from(f.to_string())), j.to_owned())
    } else if j.is_number() {
        Element::Number(Index::Key(String::from(f.to_string())), j.to_owned())
    } else {
        Element::Null(Index::Key(String::from(f.to_string())))
    }
}

fn get_pointer_object(val : &Map<String, Value>, vec : &mut String, fuzzy_data : &mut Vec<String>) {
    let mut original_vec: String = String::new();
    println!("Outer Map is {:#?}", val);
    println!("Outer vec is {:#?}", vec);
    println!("Fuzzy vec is {:#?}", fuzzy_data); 
    val.iter().for_each(|item|{
        println!("Inner Map is {:#?}", item);
        println!("Inner vec is {:#?}", vec);
        println!("Fuzzy vec is {:#?}", fuzzy_data); 
        match item.1 {
            Value::Object(map) => { original_vec = vec.to_string();vec.push('/'); vec.push_str(item.0) ;  get_pointer_object(map, vec, fuzzy_data);vec.clear();vec.push_str(&original_vec)},
            Value::Array(i) => { vec.push('/'); vec.push_str(item.0); get_pointer_array(i, vec, fuzzy_data)},
            Value::Null => {vec.push_str("/"); vec.push_str(item.0); vec.push_str(" : NULL"); fuzzy_data.push(vec.to_owned());vec.clear(); vec.push_str(&original_vec) } ,
            Value::Bool(i) => {original_vec = vec.to_string(); vec.push_str("/"); vec.push_str(item.0); vec.push_str(" : ");vec.push_str(&i.to_string()); fuzzy_data.push(vec.to_owned());vec.clear(); vec.push_str(&original_vec)},
            Value::Number(i) => {original_vec = vec.to_string(); vec.push_str("/"); vec.push_str(item.0); vec.push_str(" : ");vec.push_str(&i.to_string()); fuzzy_data.push(vec.to_owned());vec.clear(); vec.push_str(&original_vec) },
            Value::String(i) => {original_vec = vec.to_string(); vec.push_str("/"); vec.push_str(item.0); vec.push_str(" : ");vec.push_str(&i.to_string()); fuzzy_data.push(vec.to_owned()); vec.clear(); vec.push_str(&original_vec) },
        }
    });
    
}

// fn get_pointer_object(val : &Map<String, Value>, vec : &mut String) {
//     val.iter().map(|item|{
//         match item.1 {
//             Value::Object(map) => { vec.push('/'); vec.push_str(item.0) ;  get_pointer_object(map, vec)},
//             Value::Array(i) => { vec.push('/'); vec.push_str(item.0); get_pointer_array(i, vec)},
//             _ => {vec.push_str("/"); vec.push_str(item.0); vec } 

//         }

//     }).fold(Vec::new(), |acc, item|{

//     });
    
// }


fn get_pointer_array (val: &Vec<Value>, vec : &mut String, fuzzy_data : &mut  Vec<String>) {
    let mut original_vec: String = String::new();
    println!("Outer Array is {:#?}", val);
    println!("Outer Array vec is {:#?}", vec);
    println!("Fuzzy Array vec is {:#?}", fuzzy_data); 

    val.iter().enumerate().for_each(|(k,v)|{
        println!("Inner Array is {:#?}", (k,v));
        println!("Inner Array Vec is {:#?}", vec);
        println!("Fuzzy Array vec is {:#?}", fuzzy_data); 
        match v {
            Value::Array(item) => {original_vec = vec.to_string();vec.push('/'); vec.push_str(&k.to_string()); get_pointer_array(item, vec, fuzzy_data);vec.clear();vec.push_str(&original_vec)}
            Value::Object(map) => {original_vec = vec.to_string();vec.push('/'); vec.push_str(&k.to_string()); get_pointer_object(map, vec, fuzzy_data); vec.clear();vec.push_str(&original_vec)},
            Value::Null => {original_vec = vec.to_string();vec.push_str("/"); vec.push_str(&k.to_string()); vec.push_str(" : NULL"); fuzzy_data.push(vec.to_owned()); vec.clear(); vec.push_str(&original_vec) } ,
            Value::Bool(i) => {original_vec = vec.to_string();vec.push_str("/"); vec.push_str(&k.to_string()); vec.push_str(" : ");vec.push_str(&i.to_string()); fuzzy_data.push(vec.to_owned());vec.clear(); vec.push_str(&original_vec) },
            Value::Number(i) => {original_vec = vec.to_string();;vec.push_str("/"); vec.push_str(&k.to_string()); vec.push_str(" : ");vec.push_str(&i.to_string()); fuzzy_data.push(vec.to_owned()) ;vec.clear(); vec.push_str(&original_vec)},
            Value::String(i) => {original_vec = vec.to_string();vec.push_str("/"); vec.push_str(&k.to_string()); vec.push_str(" : ");vec.push_str(&i.to_string()); fuzzy_data.push(vec.to_owned());vec.clear(); vec.push_str(&original_vec) },

        }
    });
    
}  