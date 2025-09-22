use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderWeight {
    Light,
    Heavy,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderStyle {
    Solid(BorderWeight),
    Dashed,
    Dotted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Connections {
    pub up: Option<BorderStyle>,
    pub down: Option<BorderStyle>,
    pub left: Option<BorderStyle>,
    pub right: Option<BorderStyle>,
}

impl Connections {
    pub const fn none() -> Self {
        Self {
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }

    pub const fn horizontal(style: BorderStyle) -> Self {
        Self {
            up: None,
            down: None,
            left: Some(style),
            right: Some(style),
        }
    }

    pub const fn vertical(style: BorderStyle) -> Self {
        Self {
            up: Some(style),
            down: Some(style),
            left: None,
            right: None,
        }
    }

    pub const fn corner(dir1: Direction, dir2: Direction, style: BorderStyle) -> Self {
        Self::none()
            .direction(dir1, Some(style))
            .direction(dir2, Some(style))
    }

    pub const fn t_junction(main_dir: Direction, style: BorderStyle) -> Self {
        let mut connections = Self::none();
        match main_dir {
            Direction::Up => {
                connections.up = Some(style);
                connections.left = Some(style);
                connections.right = Some(style);
            }
            Direction::Down => {
                connections.down = Some(style);
                connections.left = Some(style);
                connections.right = Some(style);
            }
            Direction::Left => {
                connections.left = Some(style);
                connections.up = Some(style);
                connections.down = Some(style);
            }
            Direction::Right => {
                connections.right = Some(style);
                connections.up = Some(style);
                connections.down = Some(style);
            }
        }
        connections
    }

    pub const fn cross(style: BorderStyle) -> Self {
        Self {
            up: Some(style),
            down: Some(style),
            right: Some(style),
            left: Some(style),
        }
    }

    pub const fn direction(mut self, direction: Direction, style: Option<BorderStyle>) -> Self {
        self.set_direction(direction, style);
        self
    }

    /// Set a connection in a specific direction
    pub const fn set_direction(&mut self, direction: Direction, style: Option<BorderStyle>) {
        match direction {
            Direction::Up => self.up = style,
            Direction::Down => self.down = style,
            Direction::Left => self.left = style,
            Direction::Right => self.right = style,
        }
    }

    pub const fn get_direction(&self, direction: Direction) -> Option<BorderStyle> {
        match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }

    pub fn has_connections(&self) -> bool {
        self.up.is_some() || self.down.is_some() || self.left.is_some() || self.right.is_some()
    }

    /// Count the number of connections
    pub fn connection_count(&self) -> usize {
        [self.up, self.down, self.left, self.right]
            .iter()
            .filter(|c| c.is_some())
            .count()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BorderChar {
    pub connections: Connections,
    pub unicode: char,
}

impl BorderChar {
    /// Create a new BoxChar
    pub fn new(connections: Connections) -> Self {
        let unicode = connections_to_unicode(&connections);
        Self {
            connections,
            unicode,
        }
    }

    /// Create a BoxChar from a Unicode character
    pub fn from_char(c: char) -> Option<Self> {
        unicode_to_connections(c).map(|connections| Self {
            connections,
            unicode: c,
        })
    }
}

impl fmt::Display for BorderChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.unicode)
    }
}

/// A complete border style specification
#[derive(Debug, Clone, PartialEq)]
pub struct BorderChars {
    pub top_left: BorderChar,
    pub top: BorderChar,
    pub top_right: BorderChar,
    pub left: BorderChar,
    pub right: BorderChar,
    pub bottom_left: BorderChar,
    pub bottom: BorderChar,
    pub bottom_right: BorderChar,
}

impl BorderChars {
    /// Create a border style with the given line style
    pub fn new(style: BorderStyle) -> Self {
        Self {
            top_left: BorderChar::new(Connections::corner(
                Direction::Down,
                Direction::Right,
                style,
            )),
            top: BorderChar::new(Connections::horizontal(style)),
            top_right: BorderChar::new(Connections::corner(
                Direction::Down,
                Direction::Left,
                style,
            )),
            left: BorderChar::new(Connections::vertical(style)),
            right: BorderChar::new(Connections::vertical(style)),
            bottom_left: BorderChar::new(Connections::corner(
                Direction::Up,
                Direction::Right,
                style,
            )),
            bottom: BorderChar::new(Connections::horizontal(style)),
            bottom_right: BorderChar::new(Connections::corner(
                Direction::Up,
                Direction::Left,
                style,
            )),
        }
    }

    /// Create a light border style
    pub fn light() -> Self {
        Self::new(BorderStyle::Solid(BorderWeight::Light))
    }

    /// Create a heavy border style
    pub fn heavy() -> Self {
        Self::new(BorderStyle::Solid(BorderWeight::Heavy))
    }

    /// Create a double border style
    pub fn double() -> Self {
        Self::new(BorderStyle::Solid(BorderWeight::Double))
    }

    /// Create a dashed border style
    pub fn dashed() -> Self {
        Self::new(BorderStyle::Dashed)
    }
}

/// Convert connections to the appropriate Unicode character
fn connections_to_unicode(connections: &Connections) -> char {
    use BorderStyle::*;
    use BorderWeight::*;

    match (
        connections.up,
        connections.down,
        connections.left,
        connections.right,
    ) {
        // No connections
        (None, None, None, None) => ' ',

        // Single lines
        (None, None, Some(Solid(Light)), Some(Solid(Light))) => '─', // horizontal
        (Some(Solid(Light)), Some(Solid(Light)), None, None) => '│', // vertical

        // Light corners
        (None, Some(Solid(Light)), None, Some(Solid(Light))) => '┌', // down-right
        (None, Some(Solid(Light)), Some(Solid(Light)), None) => '┐', // down-left
        (Some(Solid(Light)), None, None, Some(Solid(Light))) => '└', // up-right
        (Some(Solid(Light)), None, Some(Solid(Light)), None) => '┘', // up-left

        // Light T-junctions
        (Some(Solid(Light)), Some(Solid(Light)), None, Some(Solid(Light))) => '├', // vertical-right
        (Some(Solid(Light)), Some(Solid(Light)), Some(Solid(Light)), None) => '┤', // vertical-left
        (None, Some(Solid(Light)), Some(Solid(Light)), Some(Solid(Light))) => '┬', // horizontal-down
        (Some(Solid(Light)), None, Some(Solid(Light)), Some(Solid(Light))) => '┴', // horizontal-up

        // Light cross
        (Some(Solid(Light)), Some(Solid(Light)), Some(Solid(Light)), Some(Solid(Light))) => '┼',

        // Heavy horizontal/vertical
        (None, None, Some(Solid(Heavy)), Some(Solid(Heavy))) => '━',
        (Some(Solid(Heavy)), Some(Solid(Heavy)), None, None) => '┃',

        // Heavy corners
        (None, Some(Solid(Heavy)), None, Some(Solid(Heavy))) => '┏',
        (None, Some(Solid(Heavy)), Some(Solid(Heavy)), None) => '┓',
        (Some(Solid(Heavy)), None, None, Some(Solid(Heavy))) => '┗',
        (Some(Solid(Heavy)), None, Some(Solid(Heavy)), None) => '┛',

        // Heavy T-junctions
        (Some(Solid(Heavy)), Some(Solid(Heavy)), None, Some(Solid(Heavy))) => '┣',
        (Some(Solid(Heavy)), Some(Solid(Heavy)), Some(Solid(Heavy)), None) => '┫',
        (None, Some(Solid(Heavy)), Some(Solid(Heavy)), Some(Solid(Heavy))) => '┳',
        (Some(Solid(Heavy)), None, Some(Solid(Heavy)), Some(Solid(Heavy))) => '┻',

        // Heavy cross
        (Some(Solid(Heavy)), Some(Solid(Heavy)), Some(Solid(Heavy)), Some(Solid(Heavy))) => '╋',

        // Double lines
        (None, None, Some(Solid(Double)), Some(Solid(Double))) => '═',
        (Some(Solid(Double)), Some(Solid(Double)), None, None) => '║',

        // Double corners
        (None, Some(Solid(Double)), None, Some(Solid(Double))) => '╔',
        (None, Some(Solid(Double)), Some(Solid(Double)), None) => '╗',
        (Some(Solid(Double)), None, None, Some(Solid(Double))) => '╚',
        (Some(Solid(Double)), None, Some(Solid(Double)), None) => '╝',

        // Double T-junctions
        (Some(Solid(Double)), Some(Solid(Double)), None, Some(Solid(Double))) => '╠',
        (Some(Solid(Double)), Some(Solid(Double)), Some(Solid(Double)), None) => '╣',
        (None, Some(Solid(Double)), Some(Solid(Double)), Some(Solid(Double))) => '╦',
        (Some(Solid(Double)), None, Some(Solid(Double)), Some(Solid(Double))) => '╩',

        // Double cross
        (Some(Solid(Double)), Some(Solid(Double)), Some(Solid(Double)), Some(Solid(Double))) => '╬',

        // Dashed lines (approximation)
        (None, None, Some(Dashed), Some(Dashed)) => '┄',
        (Some(Dashed), Some(Dashed), None, None) => '┆',

        // Dotted lines (approximation)
        (None, None, Some(Dotted), Some(Dotted)) => '┈',
        (Some(Dotted), Some(Dotted), None, None) => '┊',

        // Default case - return a placeholder
        _ => '?',
    }
}

/// Convert a Unicode character back to connections (reverse lookup)
fn unicode_to_connections(c: char) -> Option<Connections> {
    use BorderStyle::*;
    use BorderWeight::*;

    let light = Solid(Light);
    let heavy = Solid(Heavy);
    let double = Solid(Double);

    match c {
        // Light lines
        '─' => Some(Connections::horizontal(light)),
        '│' => Some(Connections::vertical(light)),
        '┌' => Some(Connections::corner(
            Direction::Down,
            Direction::Right,
            light,
        )),
        '┐' => Some(Connections::corner(Direction::Down, Direction::Left, light)),
        '└' => Some(Connections::corner(Direction::Up, Direction::Right, light)),
        '┘' => Some(Connections::corner(Direction::Up, Direction::Left, light)),
        '├' => Some(Connections::t_junction(Direction::Right, light)),
        '┤' => Some(Connections::t_junction(Direction::Left, light)),
        '┬' => Some(Connections::t_junction(Direction::Down, light)),
        '┴' => Some(Connections::t_junction(Direction::Up, light)),
        '┼' => Some(Connections::cross(light)),

        // Heavy lines
        '━' => Some(Connections::horizontal(heavy)),
        '┃' => Some(Connections::vertical(heavy)),
        '┏' => Some(Connections::corner(
            Direction::Down,
            Direction::Right,
            heavy,
        )),
        '┓' => Some(Connections::corner(Direction::Down, Direction::Left, heavy)),
        '┗' => Some(Connections::corner(Direction::Up, Direction::Right, heavy)),
        '┛' => Some(Connections::corner(Direction::Up, Direction::Left, heavy)),
        '┣' => Some(Connections::t_junction(Direction::Right, heavy)),
        '┫' => Some(Connections::t_junction(Direction::Left, heavy)),
        '┳' => Some(Connections::t_junction(Direction::Down, heavy)),
        '┻' => Some(Connections::t_junction(Direction::Up, heavy)),
        '╋' => Some(Connections::cross(heavy)),

        // Double lines
        '═' => Some(Connections::horizontal(double)),
        '║' => Some(Connections::vertical(double)),
        '╔' => Some(Connections::corner(
            Direction::Down,
            Direction::Right,
            double,
        )),
        '╗' => Some(Connections::corner(
            Direction::Down,
            Direction::Left,
            double,
        )),
        '╚' => Some(Connections::corner(Direction::Up, Direction::Right, double)),
        '╝' => Some(Connections::corner(Direction::Up, Direction::Left, double)),
        '╠' => Some(Connections::t_junction(Direction::Right, double)),
        '╣' => Some(Connections::t_junction(Direction::Left, double)),
        '╦' => Some(Connections::t_junction(Direction::Down, double)),
        '╩' => Some(Connections::t_junction(Direction::Up, double)),
        '╬' => Some(Connections::cross(double)),

        // Dashed/dotted (approximate)
        '┄' => Some(Connections::horizontal(Dashed)),
        '┆' => Some(Connections::vertical(Dashed)),
        '┈' => Some(Connections::horizontal(Dotted)),
        '┊' => Some(Connections::vertical(Dotted)),

        _ => None,
    }
}
