use std::collections::btree_map::Values;

// This is where I'll  have Ai Generate me problems and I have to come up with logic to solve in rust so I could get comfortable with the syntax
/*
    Requirements

    1. Create a variable that owns exactly 5 animal type strings
        - Example categories: mammal, bird, reptile, fish, insect
        - Use owned strings (String), not &str

    2. Create a second variable that starts empty
    - This will be your final storage

    3. Write a loop that:
    - Takes ownership of each animal type
    - Inserts it into the second variable
    - The first variable must be unusable afterward

    4. fter the loop:
    -  Print the second variable
    - Demonstrate (mentally or via comment) that the first variable cannot be used
*/
pub fn main() {
    categorizing_animals();
    let command_rules = command_log_processor(); // return array of message called rules

    processing_logic(&command_rules);
}

// Problem: Categorizing Animals ====== Goal : Practice ownership transfer, vectors, and iteration.

pub fn categorizing_animals() {
    // creating a vector full of strings that are own with categories of animals eaching string being owned
    let category: Vec<String> = vec![
        "mammal".to_string(),
        "bird".to_string(),
        "reptile".to_string(),
        "fish".to_string(),
        "insect".to_string(),
    ];

    // create a variable that starts empty but needs to be able to intake all the animal types so I assume it has to be a vector ( changing array )
    let mut inserting_category: Vec<String> = Vec::new();

    for animal in category {
        inserting_category.push(format!("{} class", animal));
    }
    // the category variable has now become useless and cannot be printed out! data moved!
    println!(
        "Moved ownership to new vector variable {:?}",
        inserting_category
    );
} // Succesfully completed code problem 1

// =============== RUST ASSIGNMENT 2: Command Log Processor =========================================
/*
    You are building a simple command log processor. Your program receives a sequence of textual commands. Each command represents an action taken by a system. You must parse these commands into structured data and then process them.

    Requirements:

    1. Define an Enum
        Create an enum called Command with the following variants:
            - Start(String)
            - Stop
            - Pause(u32)
            - Message(String)
        Each variant represents:
            - Start(String): system started with a name
            - Stop: system stopped
            - Pause(u32): system paused for N seconds
            - Message(String): log message

    2. Raw Input Data
        Create a variable that owns a list of raw command strings.
        Example input strings (you must choose your own values):
        - "START engine"
        - "PAUSE 10"
        - "MESSAGE overheating"
        - "STOP"
        Use owned strings (Vec<String>).

    3. Parsing Logic
        Write a function that:
        - Takes ownership of the raw strings
        - Converts each string into a Command
        - Returns a Vec<Command>
        Rules:
        - "START X" → Command::Start(X)
        - "PAUSE N" → Command::Pause(N)
        - "MESSAGE X" → Command::Message(X)
        - "STOP" → Command::Stop
        If the command is invalid:
        - Ignore it (do not panic)

    4. Processing Logic
        Write another function that:
        - Takes a slice of commands (&[Command])
        - Iterates over them
        - Prints meaningful output using match
        Example (conceptual, not exact output):
        - "System started: engine"
        - "Paused for 10 seconds"
        - "Log: overheating"
        - "System stopped"

*/
#[derive(Debug)]
// im declaring key terms that will be used to display
enum Message {
    Start(String),   // system started with a name
    Stop,            // system stopped
    Pause(u32),      // system paused for N seconds
    Message(String), // log message
}

// Im basically creating a function just to store the second part of the commands so "engine" | "10" | "overheaing"
pub fn command_log_processor() -> Vec<Message> {
    let command_list: Vec<String> = vec![
        "START engine".to_string(),
        "PAUSE 10".to_string(),
        "MESSAGE overheating".to_string(),
        "STOP".to_string(),
    ];

    let mut rules: Vec<Message> = Vec::new();

    for commanding in command_list {
        let splinter: Vec<&str> = commanding.split(" ").collect();
        match splinter[0] {
            "START" => rules.push(Message::Start(splinter[1].to_string())),
            "PAUSE" => rules.push(Message::Pause(splinter[1].parse::<u32>().unwrap())),
            "MESSAGE" => rules.push(Message::Message(splinter[1].to_string())),
            "STOP" => rules.push(Message::Stop),

            _ => println!("ignore it"),
        }

        // rules.push(commanding);
    }
    println!("commands {:?}", rules);

    rules
}
// im relaying that second string to a another function
// the output of one of the items in rules is going to be thats Message::Start("engine")

// Im looking at the enum for the key terms
pub fn processing_logic(commands: &Vec<Message>) {
    for command in commands {
        match command {
            // im manually inputting the key term Start and so it'll
            Message::Start(value) => println!("Starting the {}", value),
            Message::Pause(value) => println!("Paused for {}", value),
            Message::Message(values) => println!("Log: {}", values),
            Message::Stop => println!("System Stopped"),
        }
    }
}
