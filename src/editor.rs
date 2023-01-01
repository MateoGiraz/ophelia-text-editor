use crate::Terminal;
use termion::event::Key;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    quit: bool,
    terminal: Terminal,
    cursor_pos: Position,
}

impl Editor {
    pub fn run(&mut self) {

        loop {
            if let Err(error) = self.clear_screen() { die(&error) }
            if let Err(error) = self.key_input() { die(&error) }
            if self.quit { break }
        }
    }

    pub fn default() -> Self {
        Self { 
            quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_pos: Position { x: 0, y: 0 },
        }
        
    }

    fn key_input(&mut self) -> Result<(), std::io::Error> {
        let key = Terminal::read_key()?;
        if let Key::Ctrl('w') = key { self.quit = true }
        if key == Key::Up || key == Key::Down || key == Key::Left || key == Key::Right {
            self.move_cursor(key)
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut y, mut x } = self.cursor_pos;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;       
        let width = size.width.saturating_sub(1) as usize;       
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < 2 * height { y = y.saturating_add(1) }
            },
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < 2 * width { x = x.saturating_add(1) }
            },
            _ => (),
        }
        self.cursor_pos = Position { x, y }
    }

    fn clear_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        if self.quit { 
            Terminal::clear_screen();
            println!("bye!.\r");
        } else {
            self.print_rows();
            Terminal::cursor_position(&self.cursor_pos);
        }
        Terminal::flush()?;
        Terminal::cursor_show();
        Ok(())
    }

    fn print_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0.. height - 1 {
            Terminal::clear_curr_line();
            if row == 0 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Ophelia text editor -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;            
        let len = welcome_message.len();            
        let padding = width.saturating_sub(len) / 2;            
        let spaces = " ".repeat(padding.saturating_sub(1));            
        welcome_message = format!("~{}{}", spaces, welcome_message);            
        welcome_message.truncate(width);            
        println!("{}\r", welcome_message);
    }
}

fn die(e: &std::io::Error) {
    //print!("{}", termion::clear::All);
    Terminal::clear_screen();
    panic!("{e}");
}