mod app;
mod banner;
mod event;
mod ui;
use crate::event::Key;
use app::{App, Route};

use clap::Args;
use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    tty::IsTty,
};

// use banner::BANNER;
use emoji;
use emoji::symbols::math::PLUS;
use emoji::symbols::other_symbol::CHECK_MARK;
use serde_json::{Result as Rs, Value};
use std::{
    backtrace::Backtrace,
    env,
    error::Error,
    io::{self},
    process,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
pub const PL: &'static str = PLUS.glyph;
pub const CHK: &'static str = CHECK_MARK.glyph;
// use clap::{Command};

fn main() -> Result<(), Box<dyn Error>> {
    // println!("Custom backtrace: {}", Backtrace::capture());

    // From Args
    let args: Vec<String> = env::args().collect();

    // From ClipBoard
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let contents = ctx.get_contents().unwrap();

    // As Strem from another process
    let mut input: String = String::new();

    // println!("The lines are {:#?}",io::stdin().read_line(&mut input) );
    // // spawn a thread to read from /dev/stdin
    // // entering raw mode in crossterm seems to break std::io::stdin
    // thread::spawn(move || {
    //     let file = File::open("/dev/stdin").unwrap();
    //     for line in BufReader::new(file).lines() {
    //         tx.send(line.unwrap()).ok();
    //     }
    // });

    if !io::stdin().is_tty() {
        //TODO : THIS IS IMP
        // input = fs::read_to_string("/dev/stdin")?.parse()?;

        for line in io::stdin().lines() {
            if let Ok(s) = line {
                input.push_str(&s)
            } else {
                break;
            }
        }
    }

    // println!("items is {:#?}",input);
    // process::exit(1);

    //End Stdin
    // process::Command::new("exec")
    // .arg("0<&-")
    // .spawn()
    // .expect("FAILED COMMAND");

    // Displays Command in command line - jt
    // todo()

    // let matches = Command::new(env!("CARGO_PKG_NAME"))
    //     .version(env!("CARGO_PKG_VERSION"))
    //     .author(env!("CARGO_PKG_AUTHORS"))
    //     .about("Command Line utility to view Json Objects")
    //     .before_help(BANNER)
    //     .after_help("Have Fun!!");

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    //Initialize app and Draw

    let mut app = match args.len() {
        2 => App::init(args.get(1).unwrap()),
        _ => {
            if input.len() > 0 {
                App::init(&input)
            } else {
                App::init(&contents)
            }
        }
    };

    //Set Json
    let js = Some(get_json_from_string(&app)?);
    app.set_json(js);

    //Set Display Elements
    // process::exit(1);
    app.set_elements();

    // process::exit(1);
    let events = event::Events::new(200);

    // RUN app
    let res = run_app(&mut terminal, &mut app, events);

    if let Err(e) = res {
        panic!("App failed at {:#?}", e);
    }

    //Disable the raw mode upon exit from app
    //Leave alternate screen
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    // terminal.set_cursor(x, y)
    // terminal.se
    Ok(())
}

// Runs the App
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    events: event::Events,
) -> Result<(), Box<dyn Error>> {
    terminal.hide_cursor()?;
    // terminal.set_cursor(2,2).unwrap();

    loop {
        let nav_stack = app.get_current_navigation_stack();
        let current_route = app.get_current_route();
        terminal.draw(|f| ui::draw_ui(f, app))?;
            // terminal.draw(|f| ui::draw_routed_ui(f,  app))?;
        // if current_route == Route::Search {
        //     terminal.show_cursor().unwrap();    
        // }
        match events.next()? {
            event::Event::Input(key) => {
                if current_route == Route::Search {
                    if key == Key::Ctrl('c') {
                        break Ok(());
                    }
                    handle_input(key, app);
                } else {
                    if key == Key::Ctrl('c') {
                        break Ok(());
                    } else if key == Key::Char('/') {
                        app.set_current_route(Route::Search);
                        app.set_fuzzy_elements();
                    } else if key == Key::Down {
                        app.elements.as_mut().unwrap().next();
                    } else if key == Key::Up {
                        app.elements.as_mut().unwrap().previous();
                    } else if key == Key::Enter {
                        app.set_route();
                        app.set_elements();
                    } else if key == Key::Esc && nav_stack.len() > 0 {
                        app.pop_route();
                        app.set_elements();
                    }
                }
            }
            event::Event::Tick => {} // }
        }
    }
}

// Draws UI

fn get_json_from_string(app: &App) -> Rs<Value> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(app.data)?;
    Ok(v)
    // eprint!("Value is {}  " , v);
}
fn handle_input(key: Key, app: &mut App) {
    //Set input
    //Fuzzy match
    //Set data
    
    // terminal.show_cursor().unwrap();
    if let Key::Char(charac) = key {
        app.user_input.push(charac);
        // terminal.set_cursor(x+1, y).unwrap();
        app.search_and_set_fuzzy_data();
    } else if key == Key::Down {
        app.fuzzy_elements.as_mut().unwrap().next();
    } else if key == Key::Up {
        app.fuzzy_elements.as_mut().unwrap().previous();
    } else if key == Key::Backspace {
        app.user_input.pop();
        app.search_after_pop();
    } else {
        match key {
            Key::Ctrl(c) => if c == 'q' {app.user_input.clear(); app.set_current_route(Route::Main)}
            _ => {}
        }
    }
}
