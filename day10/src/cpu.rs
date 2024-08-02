use crate::parser::Command;

pub struct Cpu<'a> {
    cycle: i64,
    x: i64,
    on_tick: Box<dyn FnMut(i64, i64) -> () + 'a>,
}

impl<'a> Cpu<'a> {
    pub fn new<F>(on_tick: F) -> Cpu<'a>
    where
        F: FnMut(i64, i64) -> () + 'a,
    {
        Cpu {
            cycle: 1,
            x: 1,
            on_tick: Box::new(on_tick),
        }
    }

    fn tick(&mut self) {
        self.on_tick.as_mut()(self.cycle, self.x);
        self.cycle += 1
    }

    pub fn exec_command(&mut self, cmd: Command) {
        match cmd {
            Command::Noop => self.tick(),
            Command::Addx(x) => {
                self.tick();
                self.tick();
                self.x += x
            }
        }
    }
}
