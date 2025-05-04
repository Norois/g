use std::io::{stdout, Write};

use crossterm::{event::KeyCode, ExecutableCommand};

use crate::data::{self, DataBase};

enum MainMenuSelect{
    Start,
    Quit
}
impl MainMenuSelect{
    fn go_down(&self) -> MainMenuSelect{
        match self{
            MainMenuSelect::Start => MainMenuSelect::Quit,
            MainMenuSelect::Quit => MainMenuSelect::Quit
        }
    }
    fn go_up(&self) -> MainMenuSelect{
        match self{
            MainMenuSelect::Start => MainMenuSelect::Start,
            MainMenuSelect::Quit => MainMenuSelect::Start
        }
    }
}

pub struct MainMenu{
    selected: MainMenuSelect,
    window: Window
}

impl MainMenu{
    pub fn new() -> MainMenu{
        let window = Window {
            position_x: 0,
            position_y: 0,
            width: 21,
            height: 10,
            title: "Main Menu"
        };
        MainMenu { selected: MainMenuSelect::Start, window}
    }
    pub fn print_menu(&self){
        stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        stdout().execute(crossterm::style::ResetColor).unwrap();

        self.window.draw_window();

        let selected = match self.selected{
            MainMenuSelect::Start => (true, false),
            MainMenuSelect::Quit => (false, true),
        };

        MenuTools::draw_option(4, 3, "Start", selected.0);
        MenuTools::draw_option(4, 6, "Quit", selected.1);

        // println!("{:?}", data::get_savefiles());
        stdout().flush().unwrap();
    }
    fn process_selected(){
        
    }
    pub fn process_menu(&mut self){
        self.print_menu();
        loop {
            match crossterm::event::read().unwrap(){
                crossterm::event::Event::Resize(..) => self.print_menu(),
                crossterm::event::Event::Key(k) => match k.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up => {self.selected = self.selected.go_up(); self.print_menu();},
                    KeyCode::Down => {self.selected = self.selected.go_down(); self.print_menu();},
                    _ => ()                    
                }
                _ => ()
            }
        }
    }
}

struct Window{
    position_x: u16,
    position_y: u16,
    width: u16,
    height: u16,
    title: &'static str
}
impl Window{
    fn new(position_x: u16, position_y: u16, width: u16, height: u16, title: &'static str) -> Window{
        Window{position_x, position_y, width, height, title}
    }
    fn draw_window(&self){
        stdout().execute(crossterm::cursor::MoveTo(self.position_x, self.position_y)).unwrap();

        let side_size = ((self.width - 2) - (self.title.len() as u16 + 4)) / 2;

        print!("╔");
        for _ in 0..(side_size){
            print!("═");
        }
        print!("[ {} ]", self.title);
        for _ in 0..(side_size + (((self.width - 2) - (self.title.len() as u16 + 4)) % 2)){
            print!("═");
        }
        print!("╗\n");
        stdout().execute(crossterm::cursor::MoveToColumn(self.position_x)).unwrap();
        for _ in 0..(self.height - 2){
            print!("║");
            stdout().execute(crossterm::cursor::MoveRight((self.width - 2) as u16)).unwrap();
            print!("║\n");
            stdout().execute(crossterm::cursor::MoveToColumn(self.position_x)).unwrap();
        }
        print!("╚");
        for _ in 0..(self.width - 2){
            print!("═");
        }
        print!("╝");
        stdout().execute(crossterm::cursor::MoveTo(0, 0)).unwrap();
    }
}

struct SelectableOption{
    position_x: u16,
    position_y: u16,
    text: &'static str,
    
}

struct MenuTools;
impl MenuTools {
    fn draw_option(pos_x: u16, pos_y: u16, text: &str, selected: bool){
        stdout().execute(crossterm::cursor::MoveTo(pos_x, pos_y)).unwrap();
        if selected {
            stdout().execute(crossterm::cursor::MoveLeft(2)).unwrap();
            print!("> [{text}]");
        } else{
            print!("[{text}]");
        }
    }
}