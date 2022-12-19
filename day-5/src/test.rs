use colored::*;

fn dbg_print(severity: String, messege: String, from: String) {
    if severity == "warn" {
        println!("{}", format!("{messege} called by {from}").red());
    } else if severity == "debug" {
        println!("{}", format!("{messege} called by {from}").yellow());
    } else if severity == "result"{
        println!("{}", format!("{messege} called by {from}").green());
    } else {
        println!("{}", format!("malformed debug: severity: {severity}, messege: {messege}, from: {from}").purple());
    }
}

pub fn main () {
    let test_sub = "33";
    dbg_print(format!("debug"), format!("test_sub = {test_sub}"), format!("testingenv"));
    dbg_print(format!("warn"), format!("warning test"), format!("testingenv"));
    dbg_print(format!("result"), format!("reult = 17"), format!("testingenv"));
    dbg_print("warn".to_string(), "result =5".to_string(), "testingenv");
}
