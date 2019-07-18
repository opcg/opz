extern crate clap;

use clap::App;
use clap::SubCommand;
use pyembed::{default_python_config, MainPythonInterpreter, PythonRunMode};

fn main() {

    let matches = App::new("OpenPipelineCLI")
                            .version("0.0.1")
                            .author("Christopher Heckey <chrisheckey@gmail.com>")
                            .about("CLI for building, testing, and releasing OpenPipelineCG packages")
                            .args_from_usage("-i, --info 'print current opcg package info'")
                            .subcommand(SubCommand::with_name("python")
                                                    .about("Start an interactive python interpreter")
                                                    )
                            .subcommand(SubCommand::with_name("test")
                                                    .about("Hello World Call")
                                                    )
                            .get_matches();


    if let Some(c) = matches.value_of("info") {
        println!("Value for info: {}", c);
    }

    if let Some(matches) = matches.subcommand_matches("test") {

        let code = {
            let mut config = default_python_config();
            config.run = PythonRunMode::Eval {
                code: String::from("import opz.hello;opz.hello.test()") 
            };

            match MainPythonInterpreter::new(config) {
                    Ok(mut interp) => {
                        interp.run_as_main();

                        // Return Success
                        0
                    }
                    Err(msg) => {
                        eprintln!("{}", msg);

                        // Return Error
                        1
                    }
            }
            
        };

        std::process::exit(code);

    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(matches) = matches.subcommand_matches("python") {

        // The following code is in a block so the MainPythonInterpreter is destroyed in an
        // orderly manner, before process exit.
        let code = {
            // Load the default Python configuration as derived by the PyOxidizer config
            // file used at build time.
            let mut config = default_python_config();
            config.run = PythonRunMode::Repl;

            // Construct a new Python interpreter using that config, handling any errors
            // from construction.
            match MainPythonInterpreter::new(config) {
                Ok(mut interp) => {
                    // And run it using the default run configuration as specified by the
                    // configuration. If an uncaught Python exception is raised, handle it.
                    // This includes the special SystemExit, which is a request to terminate the
                    // process.
                    interp.run_as_main()
                }
                Err(msg) => {
                    eprintln!("{}", msg);
                    1
                }
            }
        };

        // And exit the process according to code execution results.
        std::process::exit(code);
    }
}
