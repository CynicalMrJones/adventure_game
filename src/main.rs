
use std::io;
use std::process::exit;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};
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
    inside: bool,
    description: String
}

impl Object {
    fn look(&self) -> String{
        return format!("You are looking at {}", self.name).to_string();
    }
    fn describe(&self) -> String{
        return format!("{}", self.description).to_string();
    }
    fn new(name: String, key: bool, inside: bool, description: String) -> Self{
        Self{name, key, inside, description}
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
    if commands.len() == 0{
        return
    }
    // for single word commands
    else if commands.len() < 2{
        if commands[0] == "inventory" {
            actions::inventory(player);
        }
        else if commands[0] == "quit" {
            exit(0);
        }
        else {
            text_roll("I can't do that\n".to_string());
        }
    }
    else {
        if commands[0] == "look" {
            text_roll(actions::look(commands[1], room, player));
        }
        else if commands[0] == "describe" {
            text_roll(actions::describe(commands[1], room));
        }
        else if commands[0] == "take" {
            text_roll(actions::take(commands[1], room, player));
        }
        else if commands[0] == "use" {
            text_roll(actions::interact(commands[1], player));
        }
        else {
            text_roll("I can't do that".to_string());
        }
    }
}

fn text_roll(mut text: String){
    let test = time::Duration::from_millis(30);
    text.push('\n');
    for c in text.chars(){
        print!("{c}");
        std::io::stdout().flush().expect("Flush that boi");
        thread::sleep(test);
    }
}

fn clear(){
    print!("\x1B[2J\x1B[1;1H");
}

fn paint(pic: &str){
    let content = fs::read_to_string(format!("pictures/{}", pic))
        .expect("Picture failed to load");
    println!("{}", content);
}

fn audio_player(song: &str){
    let audio = song.to_string();
    thread::spawn(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(audio).unwrap());
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = Decoder::new(file).unwrap();
        sink.set_volume(0.4);
        sink.append(source);
        std::thread::sleep(std::time::Duration::from_secs(10000));
    });
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
    clear();
    paint("castle.pic");
    audio_player("song.mp3");
    let mut name = String::new();
    text_roll("Hello what is your name?: ".to_string());
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    player.name = name.trim_end().to_string();
    text_roll(format!("Thank you {} enjoy the game\n", player.name));

    // main game "loop"
    text_roll(format!("Wow my head hurts. How did I end up here. I should find a way out. I should look around I think I'm in a {}", room1.name));
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let text: Vec<_> = input.split_whitespace().collect();
        runner(text, &room1, &mut player);
    }
}
