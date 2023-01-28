use sscanf::sscanf;

pub enum Command{
    noop,
    addx(i32)
}

pub struct Device{
    pub register_history: Vec<i32>
}

impl Device{
    pub fn new() -> Self{
        Device{register_history: vec![1]}
    }

    /// Executes the passed command, storing the register history
    /// # Examples
    /// ```
    /// use advent_of_code_2022_10::Device;
    /// use advent_of_code_2022_10::Command;
    /// let mut device = Device::new();
    ///
    /// device.execute(Command::noop);
    /// device.execute(Command::addx(3));
    /// device.execute(Command::addx(-5));
    /// assert_eq!(device.register_history, vec![1,1,1,4,4,-1]);
    /// ```
    pub fn execute(&mut self, command: Command){
        match command{
            Command::noop => self.register_history
                                .push(*self.register_history.last().unwrap()),
            Command::addx(V) => {
                self.register_history
                    .push(*self.register_history.last().unwrap());
                self.register_history
                    .push(*self.register_history.last().unwrap() + V);
            }
        }
    }
}

/// Executes the commands in the passed string and returns the resulting Device state
fn execute_commands(input_commands: &str) -> Device{
    let mut device = Device::new();

    for command in input_commands.lines(){
        match sscanf!(command, "addx {i32}") {
            Ok(V) => device.execute(Command::addx(V)),
            Error => device.execute(Command::noop)
        }
    }
    
    return device;
}

/// Draws the screen represented by the passed commands
/// # Examples
/// ```
/// use advent_of_code_2022_10::render;
/// use std::fs;
/// use std::env;
///
/// let contents = fs::read_to_string("example_input.txt").unwrap();
/// assert_eq!(render(&contents),
///     concat!(
///         "##..##..##..##..##..##..##..##..##..##..\n",
///         "###...###...###...###...###...###...###.\n",
///         "####....####....####....####....####....\n",
///         "#####.....#####.....#####.....#####.....\n",
///         "######......######......######......####\n",
///         "#######.......#######.......#######....."
///     )
/// );
///
/// ```
pub fn render(input_commands: &str) -> String{
    let device = execute_commands(input_commands);

    let mut screen = String::new();

    for (i, val) in device.register_history.iter().enumerate(){
        let col: i32 = i as i32 % 40;
        if (val - col).abs() <= 1{
            screen.push('#');
        }else{
            screen.push('.');
        }

        if col == 39{
            screen.push('\n');
        }
    }

    // Remove trailing newline and pixel
    screen.pop();
    screen.pop();
    
    return screen;
}

/// Sums the signal strength at 20 and every 40 cycles after
/// # Examples
/// ```
/// use advent_of_code_2022_10::sum_strength;
/// use std::fs;
/// use std::env;
///
/// let contents = fs::read_to_string("example_input.txt").unwrap();
/// assert_eq!(sum_strength(&contents), 13140);
/// ```
pub fn sum_strength(input_commands: &str) -> i32{
    let device = execute_commands(input_commands);

    [20,60,100,140,180,220].iter().map(|n| (*n as i32) * device.register_history[n-1]).sum()
}
