use std::fs::DirEntry;
use std::io::{Write, BufRead, stdout, stdin, Stdout, Stdin, Error};
use std::{fs, env};
use std::process::{Command, Child};

fn main() {
    let mut handout:Stdout = stdout();
    let handin: Stdin = stdin();
    let mut index: u8 = 0;
    let mut entries: Vec<Entry> = Vec::new();
    cwrite("Found these programs:\r\n", &mut handout);
    for exec in fs::read_dir(env::current_dir().unwrap().as_path()).unwrap(){
        let execc: DirEntry = exec.unwrap();
        let path: String = execc.path().to_str().unwrap().to_string().to_owned();
        let execname: String = execc.file_name().to_str().unwrap().to_owned();
        if execname.ends_with(".exe"){
            cwrite(format!("{index} > {execname}\r\n").as_str(), &mut handout);
            entries.push(Entry{index: index, path: path});
            index += 1;
        }
    }
    loop{
        cwrite("Enter the executable index to start: ", &mut handout);
        let curow = index + 2;
        let mut inpt: u8 = 0;
        let mut is_err: bool = true;
        let reads: String = cread(&handin).replace("\r\n", "");
        match reads.parse::<u8>(){
            Ok(v) => {
                inpt = v;
                is_err = false;
            }
            Err(e) => {
                cwrite(format!("Parsing \"{reads}\" failed: {e}\r\n").as_str(), &mut handout);
                cread(&handin);
            }
        }
        if entries.contains(&Entry{index: inpt, path: String::from("-")}) && !is_err{
            match Command::new(&entries[usize::try_from(inpt).unwrap()].path).spawn(){
                Ok(mut v) => {
                    match v.wait(){
                        Ok(v) => {
                            cwrite(format!("Child process exited with status: {v}\r\n").as_str(), &mut handout);
                        }
                        Err(e) => {
                            cwrite(format!("Starting child process failed: {e}\r\n").as_str(), &mut handout);
                        }
                    }
                }
                Err(e) => {
                    cwrite(format!("Creating child process failed: {e}\r\n").as_str(), &mut handout);
                }
            }
        }
        cwrite(format!("Executable with the index {inpt}\r\n").as_str(), &mut handout);
        //cwrite(format!("\x1B[{curow};1H").as_str(), &mut handout);
    }
}

fn cwrite(text: &str, handle: &mut Stdout){
    handle.lock().write_all(text.as_bytes()).unwrap();
    handle.flush().unwrap();
}

fn cread(handle: &Stdin) -> String{
    let mut buf: String = String::from("");
    handle.lock().read_line(&mut buf).unwrap();
    return buf;
}

struct Entry{
    index: u8,
    path: String
}

impl PartialEq for Entry{
    fn eq(&self, other: &Self) -> bool {
        return self.index == other.index;
    }
}