
const NAMES_C : &'static str = "names";

fn main() {
    use std::env;

    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        eprintln!("Usage:  pod <command> <args>");
        return;
    }

    if args[1] == NAMES_C {

    }
}

//fn names_command()
// TODO : don't follow sym links, but label them in the results from 'names'

// TODO : intermediate mode
//        * need a way to store off multiple possible match results and refer to them later
//        * might need a way to start a match session in the middle of an existing session with a new pattern
//          on the results


// TODO : init (.pod)

// TODO : list all data
// TODO : list all patterns
// TODO : show named data
// TODO : show named pattern
// TODO : delete data
// TODO : delete pattern
// TODO : move data/pattern ?
// TODO : match
//        * outside of immediate mode probably has to show each possible set of match results
//        * inside immediate mode there needs to be the option of saving a match result set with a namespace
// TODO : save data     
// TODO : save pattern
// TODO : "all" file names (which can be saved into data) // just names?
// TODO : "all" known files (can also be saved into data)
// TODO : help

// TODO : following symbolic links needs to be out of scope for now; is there a way to detect when that's about to happen?

// TODO : for saving clone off the input string and then save that in the file if it turns out to be good

// TODO : .pod directory format
//       pattern directory (one pattern per file; just make the name the file name)
//       data directory (not sure what this needs to be because there needs to be entire series of results and reusing names 
//                       (especially in sub searches) is going to be useful)


// TODO : $data | $namespace.data // ?
// TODO : %pattern // ?
// TODO : restrict pattern, data, and namespaces to normal symbol characters

// TODO : pod files; pod files save $data; pod $data save $data ?
// TODO : pod 'pattern' save %pattern 
// TODO : pod 'data' save $data
// TODO : pod match $data %pattern // because they're prefixed, they can be in either order
// TODO : pod [i] match $data %pattern [i]
// TODO : pod [i] match $data 'pattern' [i]
// TODO : pod [i] match $data 'pattern' [i]
// TODO : pod [i] match %pattern 'data' [i]
// TODO : pod [i] match 'data 'pattern' [i] // if they're both literals then it's always data then pattern
// TODO : pod i // interactive mode

fn _read_line() -> std::io::Result<Box<str>> {
    use std::io::*;
    let mut s = String::new();
    stdin().read_line(&mut s)?;
    Ok(s.trim_end().into())
}
