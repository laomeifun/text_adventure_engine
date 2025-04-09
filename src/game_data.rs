use std::collections::HashMap;

/// Room 结构体代表游戏中的一个房间或位置
#[derive(Debug, Clone)]  // 移除了 Hash 特性派生，因为 HashMap 无法实现 Hash
pub struct Room {
    /// 房间的唯一标识符
    id: String,
    /// 房间的名称
    name: String,
    /// 房间的详细描述
    description: String,
    /// 房间内的物品列表
    items: Vec<String>,
    /// 房间的出口，键为方向，值为目标房间ID
    exits: HashMap<Direction, String>,
}

impl Room {
    /// 创建一个新的房间
    pub fn new(id: &str, name: &str, description: &str) -> Self {
        Room {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            items: Vec::new(),
            exits: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// 添加物品到房间
    pub fn add_item(&mut self, item: String) {
        self.items.push(item);
    }

    /// 添加出口到房间
    pub fn add_exit(&mut self, direction: Direction, room_id: String) {
        self.exits.insert(direction, room_id);
    }
    
    /// 获取指定方向的出口
    pub fn get_exit(&self, direction: &Direction) -> Option<&String> {
        self.exits.get(direction)
    }
    
    /// 获取所有出口
    pub fn get_exits(&self) -> &HashMap<Direction, String> {
        &self.exits
    }
}

/// Player 结构体表示游戏中的玩家
#[derive(Debug, Clone)]
pub struct Player {
    /// 玩家名称
    pub name: String,
    /// 玩家当前所在房间的ID
    pub current_room_id: String,
    /// 玩家的物品背包
    pub inventory: Vec<String>,
}

impl Player {
    /// 创建一个新的玩家
    pub fn new(name: &str, starting_room: &str) -> Self {
        Player {
            name: name.to_string(),
            current_room_id: starting_room.to_string(),
            inventory: Vec::new(),
        }
    }
    
    /// 添加物品到玩家背包
    pub fn add_item(&mut self, item: String) {
        self.inventory.push(item);
    }
    
    /// 移动到另一个房间
    pub fn move_to(&mut self, room_id: String) {
        self.current_room_id = room_id;
    }
}

/// Direction 枚举表示房间的出口方向
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

/// 将方向转换为中文描述
#[allow(dead_code)]
impl Direction {
    /// 将方向转换为中文描述
    pub fn to_chinese(&self) -> &str {
        match self {
            Direction::North => "北",
            Direction::South => "南",
            Direction::East => "东",
            Direction::West => "西",
            Direction::Up => "上",
            Direction::Down => "下",
        }
    }
    
    /// 从字符串解析方向
    /// 
    /// # 参数
    /// 
    /// * `s` - 要解析的字符串，支持中英文方向名称
    /// 
    /// # 返回值
    /// 
    /// 如果解析成功，返回 Some(Direction)，否则返回 None
    pub fn from_str(s: &str) -> Option<Direction> {
        match s.to_lowercase().as_str() {
            "north" | "n" | "北" => Some(Direction::North),
            "south" | "s" | "南" => Some(Direction::South),
            "east" | "e" | "东" => Some(Direction::East),
            "west" | "w" | "西" => Some(Direction::West),
            "up" | "u" | "上" => Some(Direction::Up),
            "down" | "d" | "下" => Some(Direction::Down),
            _ => None,
        }
    }
}