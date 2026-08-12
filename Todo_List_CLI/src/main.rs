use std::ffi::NulError;
///
/// TODOList CLI
///
/// It is a to do that  propose you some options as add a list, modify... there are 4.
/// The element that manage the list in this program are done inside vecs of string and vec of Vec<String>
/// I have use clone and marked it to review because it says it is not recommended cause of memory leak I guess even if not inside this program
/// Still about that clone I made. I would like to know how I can't use it if possible
/// Also the main elements are all inside some loops. Make me know if it feels ugly the way it is done
/// Also a warning appear when run the program. It works, but I don't know how can I change it

use std::io;
use std::str::FromStr;
use std::fs::File;

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
fn add_todo_in_list(mut todo_list : Vec<String>, todo_lists : &mut Vec<Vec<String>>)-> i32{//Return zero for stop the loop

    loop {

        let mut todo : String = Default::default();
        io::stdin().read_line(&mut todo).expect("Error!");

        if todo.trim()  == "0"  {
            todo_lists.push(todo_list); //Entering "0" to exit the option
            break 0
        }

        todo_list.push(String::from(&todo));

    }
}

fn main() {

    let mut todo_lists : Vec<Vec<String>> = Vec::new();//List of sublist(todo) from each list. Task by list
    let mut lists : Vec<String> = Vec::new();//List that store the todo notes from each List


    println!("\n################################################## \
               \n#\t\tWELCOME TO TodoList CLI\t\t #\
               \n##################################################\n");

    loop {

        println!("\n-----------Lists of TodoList------------\n");

        let mut count_show = 0;

        //Handle the options of the TodoList

        for list in &lists{
            count_show += 1;
            println!("{count_show} - {list}");
        }


        println!("1- Show a list; \t 2- Add a list; \t 3- Rename a list; \t 4- Remove a list \t  0- Exit the Todo list\
                \nChoose an option :
                ");

        let mut option_from_user = Default::default();
        io::stdin().read_line(&mut option_from_user).expect("Please enter a value");
        let option = Todo::from_str(&option_from_user.trim());//For managing option form user input

        match option {//Catching and executing the option that matches

            Ok(Todo::Show) => {

                println!("\nEnter the number of the list of element : \n");

                let op_list_element : i32 = match require_option() { //take the user option
                    Some(num) => num,
                    None => {
                        println!("Enter a number !");
                        continue
                    }
                };

                if op_list_element  <= 0 || op_list_element  > lists.len() as i32 { println!("List doesn't exist!"); continue; } //If there is not to_do inside a list

                for list in &todo_lists.get((op_list_element as usize) - 1){
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

                lists.push(list_name.clone());//Adding the list. Todo : {To review}
                let mut count = 0;

                for i in &lists {
                    count += 1;
                    println!("{count} - {i}");
                }


                let todo_list: Vec<String> = Vec::new();

                println!("Add a todo to the {} list. Tape 0 to exit ", list_name);

                add_todo_in_list(todo_list, &mut todo_lists);//Adding to_do elements inside a list

            },
            Ok(Todo::Rename) => {

                println!("Choose the list number you want to rename :");

                let op_list_element : i32 = match require_option() { //take the user option
                    Some(num) => num,
                    None => {
                        println!("Enter a number !");
                        continue
                    }
                };

                println!("Enter the name you want to rename : ");
                let mut list_name : String = Default::default();
                io::stdin().read_line(&mut list_name).expect("Enter a value");//Handle the case when there is no value

                if op_list_element <= 0 || op_list_element >= (lists.len() +1) as i32 { println!("List doesn't exist!"); continue; };

                lists[(op_list_element - 1) as usize] = String::from(list_name);//Rename the list name


            },
            Ok(Todo::Remove) =>{

                println!("-------Enter the number label of the list that you wan to delete---------");

                let op_list_element : i32 = match require_option() { //take the user option
                    Some(num) => num,
                    None => {
                        println!("Enter a number !");
                        continue
                    }
                };

                if op_list_element > 0 && op_list_element <= lists.len() as i32  {
                    lists.remove((op_list_element - 1) as usize);
                    todo_lists.remove((op_list_element - 1) as usize);

                    println!("********List removed successfully**********\n\n");

                }else {
                    println!("********The list you want to delete doesn't exist*********\n\n");
                };

            },
            Ok(Todo::Exit) =>{

                println!("TodoList exited successfully !");
                break;

            }
            Err(_) =>{
                continue
            }
        }

    }

}
