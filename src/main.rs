mod game_data;
use game_data::{Room, Player, Direction};
use std::collections::HashMap;
use std::io::{self, Write};

/// 创建游戏世界
fn create_world() -> HashMap<String, Room> {
    let mut world = HashMap::new();
    
    // 创建起始房间
    let mut entrance = Room::new(
        "entrance", 
        "山洞入口", 
        "你站在一个黑暗山洞的入口处。洞口阴森而潮湿，但隐约可见洞内闪烁着微弱的光芒。"
    );
    entrance.add_item("火把".to_string());
    entrance.add_exit(Direction::North, "hall".to_string());
    
    // 创建大厅
    let mut hall = Room::new(
        "hall", 
        "宽阔的大厅", 
        "这是一个宽敞的大厅，岩壁上有古老的壁画。地板上散落着一些碎石和尘土。"
    );
    hall.add_item("古老的硬币".to_string());
    hall.add_exit(Direction::South, "entrance".to_string());
    hall.add_exit(Direction::East, "treasure".to_string());
    hall.add_exit(Direction::West, "dark_passage".to_string());
    
    // 创建宝藏室
    let mut treasure = Room::new(
        "treasure", 
        "宝藏室", 
        "一个金光闪闪的房间，到处都是珍贵的宝物。但是房间中央有一条沉睡的龙！"
    );
    treasure.add_item("金币".to_string());
    treasure.add_item("宝石".to_string());
    treasure.add_exit(Direction::West, "hall".to_string());
    
    // 创建黑暗通道
    let mut dark_passage = Room::new(
        "dark_passage", 
        "黑暗通道", 
        "这是一条幽深狭窄的通道，几乎看不到任何光线。"
    );
    dark_passage.add_exit(Direction::East, "hall".to_string());
    dark_passage.add_exit(Direction::Down, "underground".to_string());
    
    // 创建地下室
    let mut underground = Room::new(
        "underground", 
        "地下室", 
        "一个阴冷的地下空间，墙壁上有水滴不断滑落。"
    );
    underground.add_item("生锈的钥匙".to_string());
    underground.add_exit(Direction::Up, "dark_passage".to_string());
    
    // 将所有房间添加到世界地图
    world.insert(entrance.get_id().to_string(), entrance);
    world.insert(hall.get_id().to_string(), hall);
    world.insert(treasure.get_id().to_string(), treasure);
    world.insert(dark_passage.get_id().to_string(), dark_passage);
    world.insert(underground.get_id().to_string(), underground);
    
    world
}

/// 显示当前房间信息
fn show_room(room: &Room) {
    println!("\n== {} ==", room.get_name());
    println!("{}", room.get_description());
    
    // 显示房间内的物品
    let exits = room.get_exits();
    if !exits.is_empty() {
        println!("\n出口:");
        for (direction, _) in exits {
            println!("- {}", direction.to_chinese());
        }
    }
}

/// 主游戏循环
fn game_loop(world: &HashMap<String, Room>, player: &mut Player) {
    println!("欢迎来到文本冒险游戏，{}！", player.name);
    println!("输入 '帮助' 获取游戏指令列表。\n");
    
    // 游戏主循环
    loop {
        // 获取玩家当前所在的房间
        let current_room = match world.get(&player.current_room_id) {
            Some(room) => room,
            None => {
                println!("错误：找不到当前房间！");
                break;
            }
        };
        
        // 显示当前房间信息
        show_room(current_room);
        
        // 获取用户输入
        print!("\n> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        
        // 处理用户输入
        match input.as_str() {
            "退出" | "exit" | "quit" => {
                println!("感谢游玩！");
                break;
            },
            "帮助" | "help" => {
                println!("\n可用指令:");
                println!("- 北/南/东/西/上/下：移动到指定方向");
                println!("- 查看背包：显示您的物品");
                println!("- 拿取 [物品]：拿取房间中的物品");
                println!("- 帮助：显示此帮助信息");
                println!("- 退出：结束游戏");
            },
            "查看背包" | "背包" | "inventory" => {
                if player.inventory.is_empty() {
                    println!("您的背包是空的。");
                } else {
                    println!("您的背包中有:");
                    for item in &player.inventory {
                        println!("- {}", item);
                    }
                }
            },
            _ => {
                // 检查是否是拿取物品的命令
                if input.starts_with("拿取 ") || input.starts_with("take ") || input.starts_with("get ") {
                    let item_name = if input.starts_with("拿取 ") {
                        input.strip_prefix("拿取 ").unwrap()
                    } else if input.starts_with("take ") {
                        input.strip_prefix("take ").unwrap()
                    } else {
                        input.strip_prefix("get ").unwrap()
                    };
                    
                    // 这里我们假设可以拿取任何物品，实际游戏中应该检查物品是否在房间中
                    player.add_item(item_name.to_string());
                    println!("你拿取了 {}。", item_name);
                }
                // 尝试将输入解析为方向
                else if let Some(direction) = Direction::from_str(&input) {
                    // 尝试向该方向移动
                    if let Some(next_room_id) = current_room.get_exit(&direction) {
                        player.move_to(next_room_id.clone());
                        println!("你向{}方向移动。", direction.to_chinese());
                    } else {
                        println!("那个方向没有出口。");
                    }
                } else {
                    println!("我不明白你的意思。输入'帮助'获取指令列表。");
                }
            }
        }
    }
}

fn main() {
    println!("文本冒险游戏引擎启动中...");
    
    // 创建玩家名称
    print!("请输入你的名字: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    
    // 创建游戏世界
    let world = create_world();
    
    // 创建玩家
    let mut player = Player::new(name, "entrance");
    
    // 开始游戏循环
    game_loop(&world, &mut player);
}