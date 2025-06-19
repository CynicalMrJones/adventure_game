
use std::io;
mod actions;

struct Player {
    name: String,
    looking_at: Object,
    hp: i32,
    inventory: Vec<Object>,
}

struct Room {
    objects: Vec<Object>,
    name: String,
    description: String
}

#[derive(Clone, Debug)]
struct Object {
    name: String,
    //if key item
    key: bool, 
    holdable: bool,
    description: String
}

impl Object {
    fn look(&self) -> String{
        return format!("You are looking at {}", self.name).to_string();
    }

    fn describe(&self) -> String{
        return format!("{}", self.description).to_string();
    }
}


fn runner(commands: Vec<&str>, room: &Room, player: &mut Player){
    if commands[0] == "look" {
        println!("{}", actions::look(commands[1], room, player));
    }
    else if commands[0] == "describe" {
        println!("{}", actions::describe(commands[1], room));
    }
    else if commands[0] == "take" {
        println!("{}", actions::take(commands[1], room, player));
    }
    else if commands[0] == "inventory" {
        actions::inventory(player);
    }
    else if commands[0] == "use" {
        println!("{}", actions::interact(commands[1], player));
    }
}


fn main() {
    let nothing = Object {
        name: String::from("nothing"),
        key: true,
        holdable: true,
        description: "Nothing to see here".to_string()
    };
    
    let player_inventory = Vec::new();
    let mut player = Player {
        name: "Jimmy".to_string(),
        looking_at: nothing,
        hp: 100,
        inventory: player_inventory
    };
    
    let vase = Object {
        name: String::from("vase"),
        key: false,
        holdable: true,
        description: "This vase looks really old".to_string()
    };

    let chest = Object {
        name: String::from("chest"),
        key: false,
        holdable: true,
        description: "This chest is definitly not a mimic".to_string()
    };

    let door = Object {
        name: String::from("door"),
        key: false,
        holdable: false,
        description: "This is a door. Its locked".to_string()
    };

    let key = Object {
        name: String::from("key"),
        key: true,
        holdable: true,
        description: "This key looks like it would open the chest".to_string()
    };

    //Array of objects
    let room1_array = vec![chest, vase, door, key];

    let room1 = Room {
        objects: room1_array,
        name: String::from("dungeon"),
        description: "This is a very dark room with not alot in it, you see a vase and a chest and the door".to_string()
    };

    // main game loop
    println!("Welcome {} to the game", player.name);
    println!("{}", room1.description);
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let test: Vec<_> = input.split_whitespace().collect();
        runner(test, &room1, &mut player);
    }
}
