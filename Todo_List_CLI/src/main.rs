///
/// TODOList CLI
///
/// It is a to do that  propose you some options as add a list, modify... there are 4.
/// The element that manage the list in this program are done inside vecs of string and vec of Vec<String> which are temporary but saved permanently inside files .txt
/// I have use clone and marked it to review because it says it is not recommended cause of memory leak I guess even if not inside this program
/// Still about that clone I made. I would like to know how I can't use it if possible
/// Also a warning appear when run the program. It works, but I don't know how can I change it

use std::{fs, io};//For being able to manipulate files and user's input
use std::str::FromStr;

//To manipulate files
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
enum Todo{
    Show,
    Add,
    Rename,
    Remove,
    Exit,
}

#[derive(Debug)]
struct ParsePointError;
impl FromStr for Todo{//Implementing FromStr trait to Todo enum
    type Err = ParsePointError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opt = s.parse::<i32>().map_err(|_| ParsePointError)?;
        // let opt = s.parse();
        println!("{:?}", opt);
        if opt == 1 {//Showing list0
            Ok(Todo::Show)
        }else if opt == 2 {//Adding a list
            Ok(Todo::Add)
        }else if opt == 3 {//Renaming a list
            Ok(Todo::Rename)
        }else if opt == 4 { //Removing a list
            Ok(Todo::Remove)
        }else if opt == 0 { //Exit
            Ok(Todo::Exit)
        }else {//Exit too
            println!("The option doesn't exit here at all. Please choose a valid option. ");
            Err(ParsePointError)
        }

    }
}

// The option required by number
pub fn require_option() -> Option<i32> {//Control that the user has entered a valid number option required

    //TOP OPTION REQUIREMENT.
    let mut option = Default::default();
    io::stdin().read_line(&mut option).expect("Please enter a value");

    let option: i32 = match option.trim().parse() { //take the user option
        Ok(num) =>
            { //Control that the value is an option list
                if num >= 0 && num <= 4 {//Range of options
                    num
                } else {
                    println!("\n〜〜〜Please! The option value should be from the option list between the range 0-4〜〜〜\n\n");
                    return None
                }
            }
        ,
        Err(_) => {
            println!("\n \t \t〜〜〜Enter a valid number Option〜〜〜\n\n");
            return None
        },
    };

    Some(option)

}

//Saving the list inside the file of lists
fn rm_lists_file() -> std::io::Result<()>{//Removing the file of lists of todo
    fs::remove_file("lists.txt")?; //The old list is removed
    let _ = File::create("lists.txt");//Creation of new lists.txt file that will hold the list names
    Ok(())
}

// Saving inside the lists.txt file the lists of Todo_List_CLI app
fn save_list(list_name : Vec<String>) -> std::io::Result<()>{
    let _ = rm_lists_file();//We remove the old file and recreate it void
    let mut l_f = File::options().append(true).open("lists.txt")?;//Open the file
    for name in list_name {//Saving each list from the lists Vec into the lists.txt file
        writeln!(&mut l_f,"{}", name.trim())?;//Write each list in the file
    }
    Ok(())

}

// read_todos_list
fn read_list(lists : &mut Vec<String>, todo_lists : &mut Vec<Vec<String>>)->std::io::Result<()> {//Reading contacts from carnet list

    let mut file = File::open("lists.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut id_ = 0;

    for line in contents.lines() {//Get line into the whole text

        id_ += 1;

        lists.push(line.into());
        let  line = line.trim();
        let list_p : String = format!("./todo_lists/{}_{line}.txt", id_);//List path file

        let mut sub_file = File::open(list_p).unwrap();

        let mut sub_contents = String::new();
        let mut vecc: Vec<String> = Vec::new();
        sub_file.read_to_string(&mut sub_contents)?;

        for todo in sub_contents.lines() {
            vecc.push(todo.into());
        }

        todo_lists.push(vecc);

    }

    Ok(())
}

// Get the list saved from the lists.txt and push them inside Vec of list for user to use it
fn read_todos_list(lists : &mut Vec<String>, list_name : &String, len : usize)->std::io::Result<()> {//Reading contacts from carnet list

    let list_p : String = format!("./todo_lists/{}_{list_name}.txt", len);
    let mut file = File::open(list_p)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    for line in contents.lines() {//Get line into the whole text
        lists.push(line.into());
    }

    Ok(())
}

//Saving the todo for a precise list of todo
fn save_todo(todo : String, todo_list : i32, list_name : &String )->std::io::Result<()>{
    let list_p : String = format!("./todo_lists/{}_{}.txt", todo_list, list_name.trim()).into();
    let mut l_f = File::options().append(true).open(list_p.trim())?;
    writeln!(&mut l_f,"{}", todo)?;//Write and add new Todo in the file
    Ok(())
}

//Creating file of todos list for each list that will be created
fn add_todo_in_list(list_name : String, todo_lists : &mut Vec<Vec<String>>, len : usize)-> i32{//Return zero for stop the loop

    let list_p = format!("./todo_lists/{}_{}.txt", len, list_name.trim());/*Looking for the leng for starting at the end of each list of todo*/

    let _ = File::create_new(list_p);//Path of specific todo_list and created

    loop {//Creation of each list until ending it by entering 0 to exit it

        let mut todo : String = Default::default();
        io::stdin().read_line(&mut todo).expect("Error!");
        let todo : String = todo.trim().into();

        if todo.trim()  == "0"  {
            todo_lists.push(vec![todo.clone()]); //Entering "0" to exit the option
            break 0
        }

        //Calling save_todo the write into file of the list the todos inside
        let _ = save_todo(todo.into(), (todo_lists.len()+1) as i32, &list_name);

    }

}

/*RENAMING A LIST */
fn rename_list(id : usize, old_list : String, list : String, list_name : Vec<String>) -> std::io::Result<()>{

    let old : String = format!("./todo_lists/{}_{old_list}.txt", id);
    let new : String = format!("./todo_lists/{}_{}.txt",id , list.trim()) ;

    println!("{old} != {new}");

    fs::rename(old, new).unwrap();

    let _ = rm_lists_file();
    let mut l_f = File::options().append(true).open("lists.txt")?;

    for name in list_name {
        writeln!(&mut l_f,"{}", name.trim())?;//Write and add new contact in the file
    }

    Ok(())

}

// Removing each todos file for a list that will be deleted
fn remove_todo(list_name : &Vec<String>, to_remove : String, id : i32)-> std::io::Result<()>{

    let to_remove = format!("./todo_lists/{id}_{to_remove}\n\n.txt");

    let _ = fs::remove_file(to_remove)?;

    //UPDATING THE NEW LIST
    let _ = rm_lists_file();
    let mut l_f = File::options().append(true).open("lists.txt")?;
    for name in list_name {
        writeln!(&mut l_f,"{}", name.trim())?;//Write and add new contact in the file
    }

    Ok(())

}

fn main() {

    let mut todo_lists : Vec<Vec<String>> = Vec::new();//List of sublist(_todo) from each list. Task by list
    let mut lists : Vec<String> = Vec::new();//List that store the _todo notes from each List

    let _ = read_list(&mut lists, &mut todo_lists);//Getting the list of todo_ from saving file and put inside list of Vec


    println!("\n################################################## \
               \n#\t\tWELCOME TO TodoList CLI\t\t #\
               \n##################################################\n");

    loop {

        println!("\n-----------LISTS OF TODOLIST CLI------------\n");

        let mut count_show = 0;

        for list in &lists{//Displaying the list
            count_show += 1;
            println!("{count_show} - {}", list.trim());
        }

        //Handle the options of the TodoList
        println!("\n1- Show a list; \t 2- Add a list; \t 3- Rename a list; \t 4- Remove a list \t  0- Exit the Todo list\
                \nChoose an option :
                ");

        let mut option_from_user = Default::default();
        io::stdin().read_line(&mut option_from_user).expect("Please enter a value");
        let option = Todo::from_str(&option_from_user.trim());//For managing option form user input

        match option {//Catching and executing the option that matches

            Ok(Todo::Show) => {//Showing todo for a list

                println!("\nEnter the number of the list of element : \n");

                let mut op_list_element = Default::default();
                io::stdin().read_line(&mut op_list_element).expect("Enter a number!");

                let op_list_element : i32 = match op_list_element.trim().parse(){ //Take the user option of the list that its todos will be show
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a number !");
                        continue
                    }
                };

                if op_list_element  <= 0 || op_list_element  > lists.len() as i32 { println!("List doesn't exist!"); continue; } //If there is not to_do inside a list

                let _ = read_todos_list(&mut todo_lists[(op_list_element as usize) -1], &lists[(op_list_element -1) as usize], op_list_element as usize);//Getting the list of todo from saving file and put inside list of Vec

                // If the list if found then the whole todo for that is displayed
                while let Some(list) = &todo_lists.get((op_list_element as usize) - 1){
                    for l in *list {
                        println!("{}", l);
                    }
                    break;
                }

                println!("\n\n\n");

            },
            Ok(Todo::Add) => {

                println!("\nEnter the name of the list : ");

                let mut list_name : String = Default::default();
                io::stdin().read_line(&mut list_name).expect("Enter a value");//Handle the case when there is no value

                let list_name = format!("{list_name}\n");
                lists.push(list_name.clone());//Adding the list. Todo : {To review}

                println!("Add a todo to the {} list. Tape 0 to exit ", list_name);
                add_todo_in_list(list_name, &mut todo_lists, lists.len());//Adding to_do elements inside a list

                let _ = save_list(lists.clone());//Saving in file list.txt

            },
            Ok(Todo::Rename) => {//Renaming list

                println!("Choose the list number you want to rename :");

                let mut op_list_element = Default::default();
                io::stdin().read_line(&mut op_list_element).expect("Enter a number!");
                let op_list_element : i32 = match op_list_element.trim().parse(){ //take the user option
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a number !");
                        continue
                    }
                };

                println!("Enter the name you want to rename : ");
                let mut list_name : String = Default::default();
                io::stdin().read_line(&mut list_name).expect("Enter a value");//Handle the case when there is no value

                if op_list_element <= 0 || op_list_element >= (lists.len() +1) as i32 { println!("List doesn't exist!"); continue; };
                let old_list : String = lists[(op_list_element - 1) as usize].clone().into();
                lists[(op_list_element - 1) as usize] = String::from(list_name);//Rename the list name into the Vec that will gonna be use for update files

                // Update files here
                let _ = rename_list(op_list_element  as usize, old_list, lists[(op_list_element - 1) as usize].clone().into(), lists.clone());

            },
            Ok(Todo::Remove) =>{//Removing a list

                println!("-------Enter the number label of the list that you wan to delete---------");

                let mut op_list_element = Default::default();
                io::stdin().read_line(&mut op_list_element).expect("Enter a number!");
                let op_list_element : i32 = match op_list_element.trim().parse(){ //take the user option
                    Ok(num) => num,
                    Err(_) => {
                        println!("Enter a number !");
                        continue
                    }
                };

                if op_list_element > 0 && op_list_element <= lists.len() as i32  { //If the list we want to remove its Id exist and doesn't outrange the list available

                    let to_remove = lists[(op_list_element - 1) as usize].clone();//The name that will gonna to be removed. I.e. for remove the todolist file
                    lists.remove((op_list_element - 1) as usize);//Removing from Vec(temporary files)
                    todo_lists.remove((op_list_element - 1) as usize);//Removing in Todo

                    let _ = remove_todo(&lists, to_remove, op_list_element);//Removing from files .txt then

                    println!("********List removed successfully**********\n\n");

                }else {
                    println!("********The list you want to delete doesn't exist*********\n\n");
                };

            },
            Ok(Todo::Exit) =>{//Exit thing option

                println!("TodoList exited successfully !");
                break;

            }
            Err(_) =>{
                continue //If wrong option input, restart!
            }
        }

    }

}
