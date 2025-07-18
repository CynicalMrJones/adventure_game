
use crate::Room;
use crate::Player;

 /* 
  * TODO: Interact is poorly written. Needs rewrite
  * Maybe create a door struct that takes a specific key
  * Maybe change to/add open action
  *
  */
pub fn look(thing: &str, room: &Room, player: &mut Player) -> String{
    if thing == "room" {
        return room.look()
    }
    else if let Some(object) = room.objects.iter().find(|obj| obj.name == thing) {
        player.looking_at = object.clone();
        return format!("{}", object.look());
    }
    else {
        return format!("There is no object named {}", thing);
    }
}

pub fn describe(thing: &str, room: &Room) -> String{
    if thing == "room" {
        return room.describe()
    }
    if let Some(object) = room.objects.iter().find(|obj| obj.name == thing.to_string()) {
        return format!("{}", object.describe());
    }
    else {
        return format!("There is no object named {}", thing);
    }
}

pub fn take(thing: &str, room: &Room, player: &mut Player) -> String{
    if let Some(object) = room.objects.iter().find(|obj| obj.name.to_string() == thing.to_string()) {
        if player.looking_at.name == thing && object.key == true {
            player.inventory.push(object.clone());
            return format!("You took the {}", object.name); 
        }
        else {
            return format!("You cannot take {}", thing);
        }
    }
    else {
        return format!("There is no {}", thing);
    }
}

pub fn inventory(player: &mut Player){
    println!("This is in my inventory: ");
    for i in &player.inventory{
        println!("{}", i.name);
    }
}


pub fn interact(thing: &str, player: &mut Player) -> String{
    if let Some(object) = player.inventory.iter().find(|obj| obj.name == thing) {
        if player.looking_at.name == "door"{
            player.inventory.pop();
            return format!("You used the {} and opened the door", thing);
        }
        else{
            return format!("There is no {} in inventory", thing);
        }
    }
    else {
        return format!("There is no {} in inventory", thing);
    }
}

