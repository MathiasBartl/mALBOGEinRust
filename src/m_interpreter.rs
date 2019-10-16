macro_rules! digits {
() => (10)
}

macro_rules! m_value {
    () => (u16)
}

macro_rules! mem_size {
() => (59049 )
}

macro_rules! max_address {
() => (59048)
}

macro_rules! base {
() => (3)
}

macro_rules! m_program {
() => (String)
/////(str) =>  str.
}

fn initialise_memory(program : m_program!()) -> MmemImpl {
    let m : [m_value!(); mem_size!()] = [0; mem_size!()];
    let memory = MmemImpl{mem : m};
    return memory;
}

fn rot_r3(inp: m_value!()) -> m_value!() {
    let di = inp % base!();
    let out = inp / base!();
    let out = out + base!() ^ (digits!() - 1) * di;
    return out;
}


fn crazy(in1: m_value!(), in2: m_value!()) -> m_value!() {
    let out : m_value!() = 0;
    for x in 0..(digits!() - 1) {
        let d1 = in1 % base!();
        let in1 = in1 / base!();
        let d2 = in2 % base!();
        let in2 = in2 / base!();

        let out= 0;

        let dout: m_value!() = match d1 {
            0 => match d2 {
                0 => 1,
                1 => 0,
                2 => 0,
                _ => panic!("Crazy Functon error")
            },
            1 => match d2 {
                0 => 1,
                1 => 0,
                2 => 2,
                _ => panic!("Crazy Functon error")
            },
            2 => match d2 {
                0 => 2,
                1 => 2,
                2 => 1,
                _ => panic!("Crazy Functon error")
            },
            _ => panic!("Crazy Functon error")
        };
        let out = (out * base!()) + dout;
    }
    return out;
}


trait Mmemory {
    fn write(&mut self,address: m_value!(), value: m_value!());
    fn read(&self, address: m_value!()) -> m_value!();
}


struct MmemImpl {
    mem: [m_value!(); mem_size!()]
    // here we could be able to modify the memory size dependent on
    //the trinary word length
}

//needs mem to be mutable
impl Mmemory for MmemImpl {
    fn write(&mut self, address: m_value!(), value: m_value!()) {
        self.mem[usize::from(address)] = value;
    }
    fn read(&self, address: m_value!()) -> m_value!() {
        return self.mem[usize::from(address)];
    }
}











const ENCODING_TABLE: [m_value!(); 94] = [57, 109, 60, 46, 84, 86, 97, 99, 96, 117, 89, 42, 77, 75, 39, 88, 126, 120, 8,
    108, 125, 82, 69, 111, 107, 78, 58, 35, 63, 71, 34, 105, 64, 53, 122, 93, 38, 103,
    113, 116, 121, 102, 114, 36, 40, 119, 101, 52, 123, 87, 80, 41, 72, 45, 90, 110, 44,
    91, 37, 92, 51, 100, 76, 43, 81, 59, 62, 85, 33, 112, 74, 83, 55, 50, 70, 104,
    79, 65, 49, 67, 66, 54, 118, 94, 61, 73, 95, 48, 47, 56, 124, 106, 115, 98];















//thread_local!(static mut a : m_value!() = 0);
//thread_local!(static mut c : m_value!() = 0);
//thread_local!(static mut d : m_value!() = 0) ;
//thread_local!(static mut memory : MmemImpl);
pub struct Interpreter{
    a : m_value!(),
    c : m_value!(),
    d : m_value!(),
}



impl Interpreter {
    fn inc_c(&mut self) {
        self.c = self.c +1;
    }

    fn inc_d(&mut self) {
        self.d = self.d + 1;
    }

    fn get_a(&self) -> m_value!(){
        return self.a;
    }

    fn set_a (&mut self,val : m_value!()){
        self.a = val;
    }

    fn get_c(&self) -> m_value!(){
        return self.c;
    }

    fn set_c (&mut self,val : m_value!()){
        self.c = val;
    }

    fn get_d(&self) -> m_value!(){
        return self.d;
    }

    fn set_d (&mut self,val : m_value!()){
        self.d = val;
    }

    fn read_mem(&self, address : m_value!()) -> m_value!(){return 0;}
    fn write_mem(&mut self,address : m_value!(), output : m_value!()){}

    fn nop(&self){}

    fn stop(&mut self){

    }

    fn input(&mut self) -> m_value!(){
        return 0;//TODO

    }

    fn output(&self, character : m_value!()) {

    }

    fn op_code_4(&mut self) {
        self.set_c(self.read_mem(self.get_d()));
    }

    fn op_code_5(&self) {
        self.output(self.get_a());
    }


    fn op_code_23(&mut self) {
        let inp = self.input();
        self.set_a(inp % 256);

    }


    fn op_code_39(&mut self) {
        let x = rot_r3(self.read_mem(self.get_d()));
        self.write_mem(self.get_d(), x);
        self.set_a(x);
    }

    fn op_code_40(&mut self) {
        let x = self.read_mem(self.get_d());
        self.set_d(x);
    }

    fn op_code_62(&mut self) {
        let x = crazy(self.read_mem(self.get_d()), self.get_a());
        self.write_mem(self.get_d(), x);
        self.set_a(x);
    }

    fn op_code_68(&self) {
        self.nop();
    }

    fn op_code_81(&mut self) { self.stop();}

    fn op_code_else(&self) {
        self.nop();
    }


    fn create_op_code(&self) -> m_value!(){
        let x = self.read_mem(self.get_c());
        let x = x + self.get_c();
        return x % 94;
    }

    fn read_command(&self) -> m_value!() {
        let address = self.get_c();
        return self.read_mem(address);
    }

    fn decode_command(&self, command: m_value!()) -> m_value!() {
        return (command + self.get_c()) % 94;
    }

    fn modify(&mut self, command_pointer : m_value!(),command: m_value!()) {
        let new_command = ENCODING_TABLE[usize::from(command % 94)];
        self.write_mem(command_pointer, new_command);
    }

    fn execute(&mut self, op_code: m_value!()) {
    match op_code {
    4 => self.op_code_4(),
    5 => self.op_code_5(),
    23 => self.op_code_23(),
    39 => self.op_code_39(),
    40 => self.op_code_40(),
    62 => self.op_code_62(),
    68 => self.op_code_68(),
    81 => self.op_code_81(),
    _ => self.op_code_else()
                   }
    }

    fn tick(&mut self) {
        let command = self.read_command();
        let command_pointer = self.get_c();
        let op_code = self.decode_command(command);
        self.execute(op_code);
        self.modify(command_pointer, command);
        self.inc_c();
        self.inc_d();
    }
}
