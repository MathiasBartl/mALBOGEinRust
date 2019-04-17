fn main() {
    println!("Hello, world!");
}




struct MOutput;

struct MRegister;

struct MProgram;


trait MListener {
    fn call();
}

trait MInterpreter {
    fn click();

    fn intialise(MProgram);



    fn get_output() -> mOutput;



    fn add_finished_listener();


    fn add_output_listener();


    fn add_end_program_listener();



    fn destroy();



}