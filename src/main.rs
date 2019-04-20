fn main() {
    println!("Hello, world!");
}


const DIGITS : int = 10;

struct MOutput;

struct MRegister;

// should contain a reader
struct MProgram;



macro_rules! m_value {
    () => (u16)
}

trait Mmemory {
    fn write(address : m_value!(), value : m_value!());
    fn read(address : m_value!()) -> m_value!();
}


trait MListener {
    fn call();
}




trait MInterpreter {
    const DIGITS : int;


    fn click();

    fn initialise(program : &MProgram);

    fn add_input_reader(reader : input);// use the reader from std

    fn get_output() -> mOutput;



    fn add_finished_listener(listener : &MListener);


    fn add_output_listener(listener : &MListener);


    fn add_end_program_listener(listener : &MListener);



    fn destroy();

    fn rot_r (inp : m_value!()) -> m_value!()   {
    let di = inp % 3;
    let out = inp /3;
    let out = out + 3^(DIGITS -1)* di;
    return out;

    }

    fn crazy(in1 : m_value!(), in2 : m_value!()) -> m_value!() {
        for x in 0..(DIGITS -1){
            let d1 = in1 % 3;
            let in1 = in1 / 3;
            let d2 = in2 % 3;
            let in2 = in2 / 3;

            let out : m_value!() = 0;

            let dout : m_value!() = match d1 {
                0 => match d2 {
                    0 => 1,
                    1 => 0,
                    2 => 0
                },
                1 => match d2 {
                    0 => 1,
                    1 => 0,
                    2 => 2
                },
                2 => match d2 {
                    0 => 2,
                        1 => 2,
                    2 => 1
                }
            };
            let out = (out *3) + dout;



        }
         return out;
    }

}

