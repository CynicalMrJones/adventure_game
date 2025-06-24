
use std::io;
use std::io::Write;
use std::{thread, time};
mod actions;


/*
 * TODO: Figure out how to only take items if you are looking at the object they 
 * are in
 */

struct Player {
    name: String,
    looking_at: Object,
    hp: i32,
    inventory: Vec<Object>,
}

struct Room {
    objects: Vec<Object>,
    name: String,
    look: String, //What you can see
    description: String //A deeper look
}

#[derive(Clone, Debug)]
struct Object {
    name: String,
    key: bool, //key item
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
    fn new(name: String, key: bool, holdable: bool, description: String) -> Self{
        Self{name, key, holdable, description}
    }
}

impl Room{
    fn look(&self) -> String{
        return self.look.to_string()
    }
    fn describe(&self) -> String{
        return self.description.to_string()
    }
    fn new(objects: Vec<Object>, name: String, look: String, description: String) -> Self{
        Self{objects, name, look, description}
    }
}


fn runner(commands: Vec<&str>, room: &Room, player: &mut Player){
    // for single word commands
    if commands.len() < 2{
        if commands[0] == "inventory" {
            actions::inventory(player);
        }
        else {
            println!("I can't do that");
        }
    }
    else {
        if commands[0] == "inventory" {
            actions::inventory(player);
        }
        else if commands[0] == "look" {
            println!("{}", actions::look(commands[1], room, player));
        }
        else if commands[0] == "describe" {
            println!("{}", actions::describe(commands[1], room));
        }
        else if commands[0] == "take" {
            println!("{}", actions::take(commands[1], room, player));
        }
        else if commands[0] == "use" {
            println!("{}", actions::interact(commands[1], player));
        }
    }
}

fn text_roll(text: String){
    let test = time::Duration::from_millis(20);
    let pause = time::Duration::from_millis(20);
    for c in text.chars(){
        print!("{c}");
        std::io::stdout().flush().expect("Flushing to succeed");
        thread::sleep(test);
    }
}

fn main() {
    let nothing = Object::new("nothing".to_string(), true, true, "Nothing to see here".to_string());
    
    //The player
    let player_inventory = Vec::new();
    let mut player = Player {
        name: "Jimmy".to_string(),
        looking_at: nothing,
        hp: 100,
        inventory: player_inventory
    };
    
    //object list
    let vase = Object::new("vase".to_string(), false, true, "This vase is very old. Wait... Apon further inspection I can see a key inside".to_string());
    let chest = Object::new("chest".to_string(), false, false, "This chest is not locked and looking inside I can see a ring".to_string());
    let door = Object::new("door".to_string(), false, false, "This is a door. Its locked".to_string());
    let key = Object::new("key".to_string(), true, true, "This key look like it would open a door".to_string());
    let ring = Object::new("ring".to_string(), true, true, "This ring looks important. Past the ornate gem is a picture of a little girl.".to_string());

    //Array of objects
    let room1_array = vec![chest, vase, door, key, ring];

    let room1 = Room::new(room1_array, "closet".to_string(), "This room has stone walls, a chest, a vase and a door.".to_string(), "Judging from the cold stone walls and lack of any decoration I would say that this is a closet, I wonder why somebody would throw me in a closet?".to_string());

    //Getting player name
    let mut name = String::new();
    println!("Hello what is your name?: ");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    player.name = name.trim_end().to_string();
    text_roll(format!("Thank you {} enjoy the game\n", player.name));

    // main game loop
    println!("Wow my head hurts. How did I end up here. I should find a way out. I should look around I think I'm in a {}", room1.name);
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let text: Vec<_> = input.split_whitespace().collect();
        runner(text, &room1, &mut player);
    }
}
