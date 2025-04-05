use std::io::{stdout, Write};

use crossterm::{event::KeyCode, ExecutableCommand};

pub struct MainMenu;

impl MainMenu{
    pub fn print_menu(&self){
        stdout().execute(crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        stdout().execute(crossterm::style::ResetColor).unwrap();

        let (width, height) = crossterm::terminal::size().unwrap();

        self.draw_border(width, height);   

        // stdout().execute(crossterm::cursor::MoveTo(2, 0)).unwrap();
        // print!("[ Main Menu ]");
        self.draw_title("Main Menu");
        stdout().flush().unwrap();
    }
    pub fn process_menu(&self){
        self.print_menu();
        loop {
            match crossterm::event::read().unwrap(){
                crossterm::event::Event::Resize(..) => self.print_menu(),
                crossterm::event::Event::Key(k) => match k.code {
                    KeyCode::Char('q') => break,
                    _ => ()                    
                }
                _ => ()
            }
        }
    }
    fn draw_border(&self, width: u16, height: u16){
        stdout().execute(crossterm::cursor::MoveTo(0, 0)).unwrap();
        print!("╔");
        for _ in 0..(width - 2){
            print!("═");
        }
        print!("╗\n\r");
        for _ in 0..(height - 2){
            print!("║");
            stdout().execute(crossterm::cursor::MoveRight((width - 2) as u16)).unwrap();
            print!("║\n\r");
        }
        print!("╚");
        for _ in 0..(width - 2){
            print!("═");
        }
        print!("╝");
    }
    fn draw_title(&self, title: &str){
        let (width, _) = crossterm::terminal::size().unwrap();
        let pos = width as usize - (title.len() / 2);
        stdout().execute(crossterm::cursor::MoveTo(pos as u16, 0)).unwrap();
        print!("[ {title} ]");
        let _ = stdout().flush();
        
    }
}
