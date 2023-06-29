use crate::app::{Element, Index};

use super::app::App;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use emoji::symbols::math::PLUS;
use emoji::symbols::other_symbol::CHECK_MARK;
pub const PL: &'static str = PLUS.glyph;
pub const CHK: &'static str = CHECK_MARK.glyph;

// pub fn draw_span_of_spans(app : App) -> Spans {

// }

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // TODO: Draw User Input Box
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(40),
                Constraint::Percentage(10),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(f.size());

    let input = Block::default().title("Input").borders(Borders::ALL);
    let inputpara = Paragraph::new(app.data)
        .wrap(Wrap { trim: true })
        .block(input);

    f.render_widget(inputpara, chunks[0]);
    let output = Block::default().title("Output").borders(Borders::ALL);
    // let outputpara = Paragraph::new("Hello \n world").wrap(Wrap { trim: true }).block(output);

    // let val: Result<Value, serde_json::Error> = app.json

    match app.elements.as_mut() {
        Some(v) => {
            // let vec_list = Vec::new();
            let vec_list : Vec<ListItem<'_>> =  v.items.iter().map(|f| {get_list_item(f)}).collect();

            // println!("Vector is {:#?}", vec_list);
            let out_put_list = List::new(vec_list)
                .block(output)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");
            f.render_stateful_widget(out_put_list, chunks[2], &mut v.state)
        }
        None => {panic!("hereere")}
    }
}

pub fn get_list_item(element: &Element) -> ListItem{
    match element {
        Element::Array(k, v) => match k {
            Index::Key(s) => ListItem::new(Spans::from(vec![
                Span::styled(PL, Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::styled(
                    String::from(
                        "(x)"
                            .replace("x", &v.as_array().unwrap().len().to_string())
                            .as_str(),
                    ),
                    Style::default().fg(Color::Red),
                ),
                Span::raw(" "),
                Span::raw(s),
                Span::raw(" "),
                Span::raw(":"),
                Span::raw(" "),
                Span::raw("[...]"),
            ])),
        },
        Element::Object(k, v) => match k {
            Index::Key(s) => ListItem::new(Spans::from(vec![
                Span::styled(PL, Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::styled(
                    String::from(
                        "(x)"
                            .replace("x", &v.as_object().unwrap().len().to_string())
                            .as_str(),
                    ),
                    Style::default().fg(Color::Red),
                ),
                Span::raw(" "),
                Span::raw(s),
                Span::raw(" "),
                Span::raw(":"),
                Span::raw(" "),
                Span::raw("{...}"),
            ])),
        },

        Element::Bool(k, v) => match k {
            Index::Key(s) => ListItem::new(Spans::from(vec![
                Span::styled(CHK, Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::raw(" "),
                Span::raw("(-)"),
                Span::raw(" "),
                Span::raw(s),
                Span::raw(" "),
                Span::raw(":"),
                Span::raw(" "),
                Span::from(match v.as_bool() {
                    Some(true) => "true",
                    Some(false) => "false",
                    None => "false",
                }),
            ])),
        },
        Element::Number(k,v ) => match k {
            Index::Key(s) => ListItem::new(Spans::from(vec![
                Span::styled(CHK, Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::raw(" "),
                Span::raw("(-)"),
                Span::raw(" "),
                Span::raw(s),
                Span::raw(" "),
                Span::raw(":"),
                Span::raw(" "),
                Span::styled(v.to_string(), Style::default().fg(Color::Blue))
            ])),
        },
        Element::String(k,v ) => match k {
            Index::Key(s) => ListItem::new(Spans::from(vec![
                Span::styled(CHK, Style::default().fg(Color::Red)),
                Span::raw(" "),
                Span::raw(" "),
                Span::raw("(-)"),
                Span::raw(" "),
                Span::raw(s),
                Span::raw(" "),
                Span::raw(":"),
                Span::raw(" "),
                Span::raw(v.to_string())
            ])),
        },
        Element::Null(k) => match k {
            Index::Key(s) => ListItem::new(Spans::from(vec![
                Span::raw(" "),
                Span::raw("(-)"),
                Span::raw(" "),
                Span::raw(s),
                Span::raw(" "),
                Span::raw(":"),
                Span::raw(" "),
                Span::styled("NULL", Style::default().fg(Color::LightYellow)),
            ])),
        },

    }
}

/* 
pub fn draw_output<'a>(app: &'a App) -> Vec<ListItem<'a>> {
    let mut vec_list: Vec<ListItem> = vec![];
    if app.json.as_ref().unwrap().is_array() {
        let l = app.json.as_ref().unwrap().as_array().unwrap().iter().len();
        vec_list.push(ListItem::new(Spans::from(vec![
            Span::styled(PL, Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled(
                String::from("(x)".replace("x", &l.to_string()).as_str()),
                Style::default().fg(Color::Red),
            ),
            Span::raw(" "),
            Span::raw(":"),
            Span::raw(" "),
            Span::raw("[...]"),
        ])));
    } else if app.json.as_ref().unwrap().is_object() {
        app.json
            .as_ref()
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .for_each(|f| {
                vec_list.push(ListItem::new(match app.json.as_ref().unwrap().get(f) {
                    Some(j) => {
                        /*  match j {
                            Value::Array(items) => "SDSD"
                            Value::Bool(i) => if i {

                            } else {

                            }
                            _ =>
                        }*/

                        if j.is_array() {
                            Spans::from(vec![
                                Span::styled(PL, Style::default().fg(Color::Red)),
                                Span::raw(" "),
                                Span::styled(
                                    String::from(
                                        "(x)"
                                            .replace("x", &j.as_array().unwrap().len().to_string())
                                            .as_str(),
                                    ),
                                    Style::default().fg(Color::Red),
                                ),
                                Span::raw(" "),
                                Span::raw(f.as_str()),
                                Span::raw(" "),
                                Span::raw(":"),
                                Span::raw(" "),
                                Span::raw("[...]"),
                            ])
                        } else if j.is_object() {
                            Spans::from(vec![
                                Span::styled(PL, Style::default().fg(Color::Red)),
                                Span::raw(" "),
                                Span::styled(
                                    String::from(
                                        "(x)"
                                            .replace("x", &j.as_object().unwrap().len().to_string())
                                            .as_str(),
                                    ),
                                    Style::default().fg(Color::Red),
                                ),
                                Span::raw(" "),
                                Span::raw(f.as_str()),
                                Span::raw(" "),
                                Span::raw(":"),
                                Span::raw(" "),
                                Span::raw("{...}"),
                            ])
                        } else if j.is_boolean() {
                            Spans::from(vec![
                                Span::styled(CHK, Style::default().fg(Color::Red)),
                                Span::raw(" "),
                                Span::raw(" "),
                                Span::raw("(-)"),
                                Span::raw(" "),
                                Span::raw(f.as_str()),
                                Span::raw(" "),
                                Span::raw(":"),
                                Span::raw(" "),
                                Span::from(match j.as_bool() {
                                    Some(true) => "true",
                                    Some(false) => "false",
                                    None => "false",
                                }),
                            ])
                        } else if j.is_string() {
                            Spans::from(vec![
                                Span::styled(CHK, Style::default().fg(Color::Red)),
                                Span::raw(" "),
                                Span::raw(" "),
                                Span::raw("(-)"),
                                Span::raw(" "),
                                Span::raw(f.as_str()),
                                Span::raw(" "),
                                Span::raw(":"),
                                Span::raw(" "),
                                Span::raw(j.to_string()),
                            ])
                        } else if j.is_number() {
                            Spans::from(vec![
                                Span::styled(CHK, Style::default().fg(Color::Red)),
                                Span::raw(" "),
                                Span::raw(" "),
                                Span::raw("(-)"),
                                Span::raw(" "),
                                Span::raw(f.as_str()),
                                Span::raw(" "),
                                Span::raw(":"),
                                Span::raw(" "),
                                Span::styled(j.to_string(), Style::default().fg(Color::Blue)),
                            ])
                        } else {
                            Spans::from(vec![
                                Span::styled(CHK, Style::default().fg(Color::Red)),
                                Span::raw(" "),
                                Span::raw("(-)"),
                                Span::raw(" "),
                                Span::raw(f.as_str()),
                                Span::raw(" "),
                                Span::raw(":"),
                                Span::raw(" "),
                                Span::styled("NULL", Style::default().fg(Color::LightYellow)),
                            ])
                        }
                    }
                    _ => Spans::from(vec![
                        Span::raw(" "),
                        Span::raw("(-)"),
                        Span::raw(" "),
                        Span::raw(f.as_str()),
                        Span::raw(" "),
                        Span::raw(":"),
                        Span::raw(" "),
                        Span::styled("NULL", Style::default().fg(Color::LightYellow)),
                    ]),
                }))
            });
    } else {
    }
    return vec_list;
}
*/