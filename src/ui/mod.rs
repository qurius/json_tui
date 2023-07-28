use crate::app::{Element, Index, Route};

use super::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use emoji::symbols::math::PLUS;
use emoji::symbols::other_symbol::CHECK_MARK;
pub const PL: &'static str = PLUS.glyph;
pub const CHK: &'static str = CHECK_MARK.glyph;

// pub fn draw_span_of_spans(app : App) -> Spans {

// }

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // //Todo : Draw Search UI
    // draw_search_ui(f , app);

    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(7),
            ]
            .as_ref(),
        )
        .margin(1)
        .split(f.size());

    // Draw Search
    draw_search_ui(f, app, parent_layout[0]);
    // Draw Route
    draw_routes(f, app, parent_layout[1]);

    //Todo : Draw Route UI
}

fn draw_routes<B: Backend>(
    f: &mut Frame<'_, B>,
    app: &mut App<'_>,
    parent_layout: tui::layout::Rect,
) -> () {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(parent_layout);

    let input = Block::default().title("Input").borders(Borders::ALL);
    let inputpara = Paragraph::new(app.data)
        .wrap(Wrap { trim: true })
        .block(input);

    f.render_widget(inputpara, chunks[0]);

    match app.current_route {
        Route::Main => draw_main_routes(f, app, chunks[1]),
        Route::Search => draw_search_route(f, app, chunks[1])
    }

    // DRAW Output

}
fn draw_main_routes<B: Backend>(f: &mut Frame<'_, B>, app : &mut App, area: Rect) -> () {
    let output = Block::default().title("Output").borders(Borders::ALL);

    match app.elements.as_mut() {
        Some(v) => {
            // let vec_list = Vec::new();
            let vec_list: Vec<ListItem<'_>> = v.items.iter().map(|f| get_list_item(f)).collect();

            // println!("Vector is {:#?}", vec_list);
            let out_put_list = List::new(vec_list)
                .block(output)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");
            f.render_stateful_widget(out_put_list, area, &mut v.state)
        }
        None => {
            panic!("hereere")
        }
    }
}
fn draw_search_route<B: Backend>(f: &mut Frame<'_, B>, app : &mut App, area: Rect)  {
    let output = Block::default().title("Output").borders(Borders::ALL);

    match app.fuzzy_elements.as_mut() {
        Some(v) => {
            // let vec_list = Vec::new();
            let vec_list: Vec<ListItem<'_>> = v.items.iter().map(|i| ListItem::new(vec![Spans::from(Span::raw(i))])).collect();

            // println!("Vector is {:#?}", vec_list);
            let out_put_list = List::new(vec_list)
                .block(output)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">> ");
            f.render_stateful_widget(out_put_list, area, &mut v.state);
        }
        None => {
            panic!("hereere")
        }
    }
}
fn draw_search_ui<B: Backend>(f: &mut Frame<B>, app: &App, layout_chunk: Rect) -> () {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .horizontal_margin(1)
        .split(layout_chunk);

    let search = Block::default().title("Search").borders(Borders::ALL);
    let searchpara;
    if app.user_input.len() > 0  {
        searchpara = Paragraph::new(app.user_input.to_owned())
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::LightMagenta))
        .block(search);

    } else {
        searchpara = Paragraph::new(Text::from("Type / to Search") )
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC))
        .block(search);
    }


    f.render_widget(searchpara, chunks[0]);

    f.set_cursor(                
        layout_chunk.x + app.user_input.len() as u16 + 2,
    // Move one line down, from the border to the input line
        layout_chunk.y + 1,
    );

    let help = Block::default().title("Help").borders(Borders::ALL);

    let text = vec![Spans::from(vec![
        Span::raw("Type ?"),
        Span::styled("line", Style::default().add_modifier(Modifier::ITALIC)),
    ])];

    // let block = Block::default()
    //     .title("Help")
    //     .borders(Borders::ALL)
    //     .border_style(Style::default().fg(Color::Gray));

    let help_para = Paragraph::new(text)
        .block(help)
        .style(Style::default().fg(Color::Gray));

    f.render_widget(help_para, chunks[1]);

}

pub fn get_list_item(element: &Element) -> ListItem {
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
        Element::Number(k, v) => match k {
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
                Span::styled(v.to_string(), Style::default().fg(Color::Blue)),
            ])),
        },
        Element::String(k, v) => match k {
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
                Span::raw(v.to_string()),
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