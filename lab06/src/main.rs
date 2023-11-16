use std::fs;

trait Command
{
    fn get_name(&self) -> &'static str;
    fn exec(&mut self, args: Vec<&str>);
}

struct Terminal
{
    commands: Vec<Box<dyn Command>>,
}

impl Terminal
{
    fn new() -> Terminal
    {
        Terminal { commands: Vec::new() }
    }

    fn register(&mut self, command: Box<dyn Command>)
    {
        self.commands.push(command);
    }

    fn run(&mut self)
    {
        let input =fs::read_to_string("src/input.txt").expect("Something went wrong reading the file");
        // let mut line=input.lines();
        // let mut cmd=line.next().unwrap();
        for cmd in input.lines() {
            if cmd==""
            {
                //cmd=line.next().unwrap();
                continue;
            }
            if cmd=="stop"
            {
                return;
            }
            let mut args = cmd.split_whitespace();
            let command_name = args.next().unwrap();
            let command_args = args.collect::<Vec<&str>>();

            let command = self.commands.iter_mut().find(|c| c.get_name() == command_name);
            match command
            {
                Some(command) => command.exec(command_args),
                None => println!("command not found"),
            }
            //cmd=line.next().unwrap();
    }
    }
}


struct PingCommand
{

}

impl Command for PingCommand
{
    fn get_name(&self) -> &'static str
    {
        "ping"
    }

    fn exec(&mut self, args: Vec<&str>)
    {
        println!("pong");
    }
}

struct CountCommand
{

}

impl Command for CountCommand
{
    fn get_name(&self) -> &'static str
    {
        "count"
    }

    fn exec(&mut self, args: Vec<&str>)
    {
        let mut count = 0;
        for _ in &args
        {
            count += 1;
        }
        println!("counted {} args", count);
    }
}

struct TimesCommand
{
    count: u32,
}

impl Command for TimesCommand
{
    fn get_name(&self) -> &'static str
    {
        "times"
    }

    fn exec(&mut self, args: Vec<&str>)
    {
        self.count += 1;
        println!("{} times", self.count);
    }
}
fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run();
}
