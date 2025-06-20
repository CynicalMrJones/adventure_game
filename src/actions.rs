
use crate::Room;
use crate::Player;

pub fn look(thing: &str, room: &Room, player: &mut Player) -> String{
    if thing == "room" {
        return room.look()
    }
    else if let Some(object) = room.objects.iter().find(|obj| obj.name == thing.to_string()) {
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
    if let Some(object) = room.objects.iter().find(|obj| obj.name == thing.to_string()) {
        if player.looking_at.name == thing && object.key == true {
            player.inventory.push(object.clone());
            return format!("You took the {}", object.name); 
        }
        else {
            return format!("There is no {}", thing);
        }
    }
    else {
        return format!("There is no {}", thing);
    }
}

pub fn inventory(player: &mut Player){
    for i in &player.inventory{
        println!("{}", i.name);
    }
}

pub fn interact(thing: &str, player: &mut Player) -> String{
    if let Some(object) = player.inventory.iter().find(|obj| obj.name == "key") {
        if player.looking_at.name == "door"{
            let name = object.name.clone();
            player.inventory.pop();
            return format!("You used the {} and opened the door", name);
        }
        else{
            return format!("There is no {} in inventory", thing);
        }
    }
    else {
        return format!("There is no {} in inventory", thing);
    }
}

