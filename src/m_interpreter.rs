macro_rules! digits {
() => (10)
}
// todo REPLACY MACROS WITH TYPE aliases
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

macro_rules! m_program {//TODO should be bytestring or asccistring
() => (String)
/////(str) =>  str.
}
//debug code
macro_rules! to3 {
($trits:expr) => {
unpack_debug!(u16::from_str_radix(&String::from($trits), base!())) // fixme unpack result
}
}

macro_rules! unpack_debug {
($result:expr) => {
match $result {
Ok(v) => v ,
Err(_) => panic!("Some debug code error."),
}
}
}

macro_rules! assert_range {
($var_name:ident) => {
assert!( 59049 > $var_name);
assert!( 0 <=  $var_name);
};
}



use core::str::Chars;
//use core::ascii;
//use std::io::{Write, Read};
//use num_traits::pow;


// todo, use this instead of writer
pub trait OutputWriter {
    fn write(&mut self, character : char);
}

pub fn prepare_program (input : Chars) -> m_program!(){//TODO check // output byte or asccistring
    //Iterator <Item=u8>
    let mut output : m_program!() = String::new();
    for c in input {
        if c.is_whitespace() || c == '\n'{
            //ignore
        } else if c.is_ascii() {
            // convert and put into next memory space
output.push(c);
        } else {
            panic!("Illegal program character");
        }
    }
    return output;
}

 pub fn initialise_memory(program : m_program!()) -> MmemImpl {//TODO check
    let  m : [m_value!(); mem_size!()] = [0; mem_size!()];
    let mut memory = MmemImpl{mem : m};
    let mut mem_counter : m_value!() = 0;
    for c in program.bytes() { //TODO bytes
        memory.write(mem_counter,c as m_value!());//conversion char to number
mem_counter = mem_counter +1;
    }
    assert!(mem_counter > 1); //programs need to have at least 2 valid characters
    for x in mem_counter..max_address!() {

        let v_old = memory.read(x-2);
        let old = memory.read(x-1);
        memory.write(x, crazy(v_old,old));
        //question, program less than two chars
    }
    return memory;
}

fn rot_r3(inp: m_value!()) -> m_value!() { //TODO assert range
assert_range!(inp);
let base : m_value!() = base!();
    let di : m_value!() = inp % base!();
    let out : m_value!() = inp / base!();
    let out : m_value!()= out + base.pow(digits!() as u32 - 1) * di;
    assert_range!(out);
    return out;
}


//TODO assert range
fn crazy(mut in1: m_value!(),mut in2: m_value!()) -> m_value!() {
    assert_range!(in1);//debug code
    assert_range!(in2);
    let mut out : m_value!() = 0;
    let mut power = 1;
    //let mut i1 = in1;
    //let mut i2 = in2;
    for _x in 0..(digits!() ) {//FIXME, is that actuall correct
        let d1 = in1 % base!();
        in1 = in1 / base!();
        let d2 = in2 % base!();
        in2 = in2 / base!();


        let dout: m_value!() = match d1 {
            0 => match d2 {
                0 => 1,
                1 => 0,
                2 => 0,
                _ => panic!("Crazy Function error")
            },
            1 => match d2 {
                0 => 1,
                1 => 0,
                2 => 2,
                _ => panic!("Crazy Function error")
            },
            2 => match d2 {
                0 => 2,
                1 => 2,
                2 => 1,
                _ => panic!("Crazy Function error")
            },
            _ => panic!("Crazy Function error")
        };
        out = out  + dout * power;
        power = power * base!();
    }
    assert_range!(out);//debug code
    return out;
}


trait Mmemory {
    fn write(&mut self,address: m_value!(), value: m_value!());
    fn read(&self, address: m_value!()) -> m_value!();
    //debug code
    fn dump(&self) -> [m_value!(); mem_size!()];
}


pub struct MmemImpl {
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
    //debug code
    fn dump(&self) -> [m_value!(); mem_size!()] {
    return self.mem;
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
pub struct Interpreter<'a>{
    //debug code
    opcode_log : Vec<m_value!()>,
    memory : Box<dyn Mmemory>,
    a : m_value!(),
    c : m_value!(),
    d : m_value!(),
    io_iterator : & 'a mut dyn Iterator<Item = char>,
    io_callback : &'a mut dyn OutputWriter,

}

pub fn initialise_interpreter<'a>(source  : String,  inputhandle : &'a mut dyn Iterator<Item = char>, outputhandle : &'a mut dyn  OutputWriter) -> Interpreter<'a> {
    let program = prepare_program(source.chars());
    let mem = initialise_memory(program);
    return Interpreter {opcode_log : Vec::new() ,memory : Box::new(mem), a : 0, c : 0, d : 0, io_iterator :inputhandle, io_callback  : outputhandle }
}

impl Interpreter<'_> {
    //debug function
    fn dump_memory(&self) -> Vec<m_value!()> {
        self.memory.dump().to_vec().clone()
    }
    fn inc_c(&mut self) {
        if self.c == max_address!(){self.c = 0} else
        {self.c = self.c +1};
    }

    fn inc_d(&mut self) {
        if self.d == max_address!() {self.d = 0} else
        {self.d = self.d + 1};
    }

    fn get_a(&mut self) -> m_value!(){
        return self.a;
    }

    fn set_a (&mut self,val : m_value!()){
        self.a = val;
    }

    fn get_c(&mut self) -> m_value!(){
        return self.c;
    }

    fn set_c (&mut self,val : m_value!()){
        self.c = val;
    }

    fn get_d(&mut self) -> m_value!(){
        return self.d;
    }

    fn set_d (&mut self,val : m_value!()){
        self.d = val;
    }

    fn read_mem(&mut self, address : m_value!()) -> m_value!(){return self.memory.read(address);}
    fn write_mem(&mut self,address : m_value!(), output : m_value!()){
        self.memory.write(address,output);
    }

    fn nop(&mut self){}

    fn stop(&mut self){
// cause an end to the execution loop
    }

    fn input(&mut self) -> m_value!(){
        let i = self.io_iterator.next();
        match i {
            Some(x) => return x as m_value!(),
            None => panic!("Program input error!"),
        }
    }

    fn output(&mut self, character : m_value!()) {
        self.io_callback.write((character as u8) as char);


    }

    fn op_code_4(&mut self) {
        let d = self.get_d();
        let old = self.read_mem(d);

        self.set_c(old);
    }

    fn op_code_5(&mut self) {
        let a = self.get_a();
        self.output(a);
    }

//fixme NEW LINES, LINE FEEDS, EOF
    fn op_code_23(&mut self) {
        let inp = self.input();
        self.set_a(inp % 256);//QUESTION MODULUS

    }


    fn op_code_39(&mut self) {
        let d = self.get_d();
        let x = rot_r3(self.read_mem(d));
        self.write_mem(d, x);
        self.set_a(x);
    }

    fn op_code_40(&mut self) {
        let d =self.get_d();
        let x = self.read_mem(d);
        self.set_d(x);
    }

    fn op_code_62(&mut self) {
        let a =self.get_a();
        let d = self.get_d();
        let val = self.read_mem(d);
        let x = crazy(val,a );
        self.write_mem(d, x);
        self.set_a(x);
    }

    fn op_code_68(&mut self) {
        self.nop();
    }

    fn op_code_81(&mut self) { self.stop();}

    fn op_code_else(&mut self) {
        self.nop();
    }

/*
    fn create_op_code(&mut self) -> m_value!(){//FIXME this is not used, but where should it be used
        let c = self.get_c();
        let x = self.read_mem(c);
        let x = x + c;
        return x % 94;
    }
*/
    fn read_command(&mut self) -> m_value!() {
        let address = self.get_c();
        return self.read_mem(address);
    }

    fn decode_command(&mut self, command: m_value!()) -> m_value!() {
        //return (command + self.get_c()) % 94;
        return (command % 94 + self.get_c() % 94) % 94;//FIXME
    }

    fn modify(&mut self, command_pointer : m_value!(),command: m_value!()) {
        let new_command = ENCODING_TABLE[usize::from(command % 94)];//TODO 
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

    pub fn tick(&mut self) -> bool {
        let command = self.read_command();
        //let command_pointer = self.get_c();
        let op_code = self.decode_command(command);
        self.opcode_log.push(op_code);
        self.execute(op_code);
        let command_pointer = self.get_c();
        let command = self.read_command();
        self.modify(command_pointer, command);//FIXME, after Jump the new instruction should be encrypted
        self.inc_c();
        self.inc_d();
        if op_code == 81 {
            return true;
        } else {return false; }
    }
}

//todo, write explicite routines for converting value to char  and likewise


/*impl std::fmt::Display for Vec<m_value!()>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(),std::fmt::Error> {
        let iter = self.iter();
        for val in iter {
            write!(f,"{},", val);
    }
        Ok(());
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_crazy_lower_trit() {
        assert_eq!(crazy(0,0) % 3, 1);
        assert_eq!(crazy(0,1) % 3, 0);
        assert_eq!(crazy(0,2) % 3, 0);
        assert_eq!(crazy(1,0) % 3, 1);
        assert_eq!(crazy(1,1) % 3, 0);
        assert_eq!(crazy(1,2) % 3, 2);
        assert_eq!(crazy(2,0) % 3, 2);
        assert_eq!(crazy(2,1) % 3, 2);
        assert_eq!(crazy(2,2) % 3, 1);
    }
    // 0,19683,
    #[test]
    fn test_crazy_upper_trit() {
        let zero = 0;
        let one = 19683;
        let two = 39366;
        let shift = 19683;
        assert_eq!(crazy(zero, zero) /shift, 1);
        assert_eq!(crazy(zero,one)/shift, 0);
        assert_eq!(crazy(zero,two) /shift, 0);
        assert_eq!(crazy(one,zero) /shift, 1);
        assert_eq!(crazy(one,one) /shift, 0);
        assert_eq!(crazy(one,two) /shift, 2);
        assert_eq!(crazy(two,zero) /shift, 2);
        assert_eq!(crazy(two,one) /shift, 2);
        assert_eq!(crazy(two,two) /shift, 1);

    }

    #[test]
    fn test_crazy_all_trit() {
        let zero = 0;
        let one = 1 + 1*3 + 1*3*3 + 1*3*3*3 + 1*3*3*3*3 + 1*3*3*3*3*3 + 1*3*3*3*3*3*3 + 1*3*3*3*3*3*3*3 + 1*3*3*3*3*3*3*3*3 + 1*3*3*3*3*3*3*3*3*3;
        let two = 2 + 2*3 + 2*3*3 + 2*3*3*3 + 2*3*3*3*3 + 2*3*3*3*3*3 + 2*3*3*3*3*3*3 + 2*3*3*3*3*3*3*3 + 2*3*3*3*3*3*3*3*3 + 2*3*3*3*3*3*3*3*3*3;
        assert_eq!(crazy(zero, zero) , one);
        assert_eq!(crazy(zero,one), zero);
        assert_eq!(crazy(zero,two) , zero);
        assert_eq!(crazy(one,zero) , one);
        assert_eq!(crazy(one,one) , zero);
        assert_eq!(crazy(one,two) , two);
        assert_eq!(crazy(two,zero) , two);
        assert_eq!(crazy(two,one) , two);
        assert_eq!(crazy(two,two) , one);

    }
    #[test]
    fn test_rotr(){
    assert_eq!(rot_r3(to3!("2000200001")),to3!("1200020000"));
    }


    struct DummyReader;

    impl Iterator for DummyReader {
        type Item = char;
        fn next (&mut self) -> Option<char> {
            assert!(false);
            None
        }
    }



    struct DummyWriter;

    impl OutputWriter for DummyWriter {
        fn write(&mut self, character: char) {
            assert!(false);
        }
    }

    fn find_end_of_program(dump : & Vec<m_value!()>) -> usize{
        let mut counter = dump.len()-1;
        while counter >= 2 && crazy(dump[counter -2], dump[counter -1 ]) == dump[counter]{
            counter = counter -1;
        }
        counter
    }

    #[test]
    fn test_read_source_file(){
        let bottles = r#"b'`;$9!=IlXFiVwwvtPO0)pon%IHGFDV|dd@Q=+^:('&Y$#m!1S|.QOO=v('98$65aCB}0i.Tw+QPU'7qK#I20jiDVgG
S(bt<%@#!7~|4{y1xv.us+rp(om%lj"ig}fd"cx``uz]rwvYnslkTonPfOjiKgJeG]\EC_X]@[Z<R;VU7S6QP2N1LK-I
,GF(D'BA#?>7~;:9y16w43s10)p-,l*#(i&%e#d!~``{tyxZpuXsrTTongOkdMhg`Hd]ba`_^W@[ZYXW9UNSRQPOHMLK
J-++FE''<A$?>=<;:387xw43s10/(-&m*)('&}${d!~}|^zyxwvutmVqpiRQlkjiKafedc\E`_^@\[ZYX;V9NMRQ42NG
LK.IH*F?DCBA$#>7~;{{8xx5uu2rr/oo,ll)ii&f|e"!aw`{z\r[vXnmVTpongPkNihgJ_dcFa`B^]\UZ=RWV8TSLQ4O
N0LE.IHA)E>'BA:?!7~5|38y6/v321q).-&m*)i'&%|{d!~}_{zs\wvutsUqTonPlOjiKgJedFbE`_A]@[Z<X;VU7S6Q
P22GL/JIB+FEDC%;@?>7~;:987w5v32r0)p-,+k)('~g$#"b~w|uz]xwvutsrqTinQlOjLhgfeH]bE`CB]\>ZSXWVUTS
RQPON1LE.I,+*((&&$$""~~||zzxxv4u210/(-n+l)(i&g$ddy~}`u^]\ZvutVlUjSQQOOdMKgfeG]F[DBB@@>><<:VU
T6L5JO200EJ-HG*E>'B%$9>=<|4{2y05v321r).o,mlj(igg|#d!~}`uz]x[ZotWUUjoRmlkNibKJIGGEEZ_B]\?Z=XW
PU876442NM/KD-B+))''%%##!!}}{{yyw5v32s0q.-&+l)j'hff{"caav{^yxwZutslUpSnQOOdiLgfHHcba`Y^A\?Z=
;;PU8SRQ4ONMLEJ-,+))''%%##!=<;{3z1xvvttrrppnnll#j!&g$#d!b}|{zyr[vYtsrTjShQfkNihgJedcba`Y^A\?
Z=;WV9TSRQPOHM0K.-++)ED&B;$9"~<;:z2y0wuussqqoom+ljj!&%$dzcx}`{zy\wvutsrqjSnQPNNLhgIedG\EZCA]
\[=S<Q:886644220L/JIHA*)(&&$@?!=6}4{yywwuus10/o'n%lj('&f|ezcaa__]][wvuWmVkTRnQlkNLLaJIHFbE`_
B]@U>Y<;P9775533H1/KJ,HA*?(&&$$">=<|4{2ywwu321q)p'nl*k('gg${"c~a`^z]xwvYtmrUpSRPlOMMbK`IGGEE
Z_^]?U>S<::8866442200.JIH*@)>C&A@?"=<5|{8y65vtt10/(-n+lk"'&%e{dyb``^^\\ZvutVlUjSQmlkMcLaJHHF
bECCX]\[=S<Q:886R5PON1LKJCH+F)(=BA@"8!6}{{2y0543s+r)pnnlljjhhffddbb`|_zyx[vutslUTSQQOOMihgI_
H]FDDBB@@>><XWV8N7L5331MLK-C,A*(D'BA$""=<;:927xwvt2s0/p-n+*)('~%f#dcaa__]y\ZZotsrTjShQOkjiKa
J_HFFDDBB@@>><X;99NS6QPO2MLKJIHA*E('%%:#8=~;:9z7654321*/p-,m*k(hh}$#dyb}`{zy[qZoXVVTTRnmlNdM
bKIIGGEECCAA?[>YXWP9T76K42200.JI+G@)>'%A@?!7~5|zzx654t,s*qo-n+*jj!h%fec!b}|{^s\[vYWWlqTonQlO
jchKfIHFFDDB^]\>T=R;P9NS6QPO2MLE.-,*FED&<%:#!!}}{{yyw543s+r)pnnl*kii~%f#"caa|{zsx[ZutVrkTinQ
lkNiLgfe^cFEDYBW\[=YR;P977553O200EJIH*@)>C&$$9>!<;|9z7654-tsrppnnll#('&f|ezca}|{]s\qZXtsrTjS
hQOOMihgI_H]FDDB^A\[><<WVUTSLQ43NM/KD-BG*ED'B%@?>=<5:{zy0wuussqqoomm$ki'hff{"c~}`{t]\wvuWmVk
pSnmPNNcLKfIGG\aD_^A\?T=<;99775QPO1G0E.,HG)E>'<%#?"~~5:98x0w.ussq/pnn%*k('hff#z!ba|{z\r[puXs
rUpSnglONihgI_H]FDDYBW\[Z<R;P977553311//--++))'CBA#9"7<}:9z7x54-t1rq(ommkkiiggeecca}|{]s\qZX
tsrTjShQOkjiKaJ_HFFDDB^A\[==XWVOT7R542N1LKJ-HGF?D'B%$""7~5|zzxxv43s1*q(ommk)jhh}$e"!~a|{zyr[
vYXVVTTRRPPNNLLJJHH]FD`_A]V?TY<WVU8SRQPOHM0K.-++))''%%#?"~~5:9y70w.us1r/.-n+*)('~%fedbbw`u^\
xwvXnWlUSSQQOOMMKKIIGGEa`_AW@UZ=XW:U8SRQPONG0/.C,*FED&<%:#!!}}{{yywwuussqqo-n+*k(!h%f#"!aw`u
^\\ZZoXVrqpRhQfOMMKgfeG]F[DBB@\[Z<R;P97S6QP22GL/J-,*F)DCB%:?"!~||zz1x/432r*q(ommkkiiggeeccaa
_{^yx[vYtmVqTSQQOOMMKKIeHFF[`_A]V?T=;;9977553O2MLK.IHAF)('%%##!=~||3876v.u,sq/pnn%*)(h~g|ecc
aa__]][[YuXsrUSSnmleNMhgfH^G\aD_^A\?ZYXQ:98M644220LK-IB+@)''%%#?>=}5|3zxxv4u21r/p-,+*#(i&g$#
c!~av_t][[YutsUkTiRPPNNLLJJHHFFDDB^A\[Z=XWVUTM6Q43HM0..CH+FE''BA@?>=6;|9z765u-t+0q.-,m*)('&%
|#dcb``uzy[wpYnWUUSonmOeNcLJJHHFFDDBB@\?ZY<W:UTSRQPOH1L/.,,**(D'%%:?>=}5|3zxxvvttrrppn,m*)ii
&%$#"!~}v{^y\wvuWmVkpSnmlOjihgfedc\aD_BAV[Z<XQ:OT7RQPI2M0/--++)EDC%;$9"~~||zz1xvv-trrppn,m*)
(!hg$#c!xav_]yxwYoXmVTTRRPPNjihJ`I^GEECCAA??=Y<::OT7RQ4O2G0K.-BGFE'=&;$9"7<}:98y6/4ut10/o'n%
*kii~%f#"c~a|{t]x[ZXXVrqpRhQfOMMKKIIGGEECCA]@[Z<<WVUNS6Q431M0KJI,GFE>C&%$""7~|:9y70w.us10/o'
n%ljjhhffddb~a|{^\\wvutmVUTRnQlkNiLgfed]FE`_A]V?TY<WV977RQPONMF/.I,**?D'BA$?"=<;:981x5vussq/
.-m%l#jh&%$dzcxa_{zy[qZoXVrqpRhQfOMMKgJHH]ba`BXAV?=Y<WV88SRQPONMFK.I,+)E(CBA$?>=<;:927x5vuss
qqo-,+k#j!hffddbb``^^s\qZXXVVTpSQQfkNihg`IHcbaCYBW\?==RW:UT755J321FK.IH+F)>'B%$9"~~||zzxxvvt
210p(o&mkki'&%e{dyb``^z]xwvYtmrUTSQmlkMcLaJHdcbDZCXA?[><<QVUT6L5J31MLK-C,A*((&&$$""~<}:9zxx5
4-tsrp.o,+l)j'&}f#d!~}_u^s\ZZXXVrqpRhQfOMihJf_H]FDDB^]\>T=R;99775Q4ON00KJIBG*E('%A$?>=~;:927
xwvttr0/.n,m$)('g}f{"c~}`{^yxwvoXsVUSShQOkjLhaJ_HFba`BXAV?=YX:VO8M644220L/JI++FEDCB;@#>!<;:z
2y05v321r/.-,+$)j'hgeeccaa__]][wZXXmVkTiRPPNjMhgfIdcba`_XA@?==;;9977L5J31MLK-C,A*((&BA@"8!6}
{{y765u-t+rppn,m*)j'h%$#"!~}v{^y\[YutsUkTiRPPNNLhgfH^G\ECCAA??==;;9UTS5K4IN1LKJC,G*)''%%#?>=
}5|3zxxvvttrrppnnl*k('gg|e"c~a__ty\wvuXmVUTinmlNdMbKIedFb[DYB@\?==RWVU7M6K42N1LK.,,G@E('&$@#
>=~;|927xw4uss*/.n,%l#(i&%f#d!~w`{^][[YYWWlUjonmOeNcLJJHHFFDDB^A\[==XWVOT7R542N1LKJ-HGF?D'&%
:?"=<}:3z7xwuussqqo-,+k#j!h}fddb~}_{t]r[YYWWUqpoQgPeNLLJJHHFFDDBB@@>Z=XWV9N7R54I2GLKJ,B+@)''
%%#?>=}5|3zx654t,s*qoommkkiig%f#"bb}v{^y\wvuWmVkpSnmlOjchKJIGGEaD_^A\?ZYR;V986RQP2H1F/--++))
''%%##!=~;:zz765.3t1rq(-,+k#j!&g$#"c~}|uz]x[ZXtsrTjShQOOMMKKIIGGEECCAA?[><<QV9TSR5PONMF/.-+G
FE'=&;$""~~||zzxxvvttrrp.o,+ljj'&%$#z!ba|_]]rwZutWrUponmlejMLKIIGGEECCX]\[=S<Q:8TSR4J3H1F/DI
,GF)D'BA@?>=6}|9zxx/432r*q(o&mk)j'&g$e"!~}|{zsx[vYXVVTponPfOdMbKIIGG\ECCAA??=YXW9O8MR5PONG0K
.I,**?D'BA@9"=~}4{yywwuussqqoommkki'h%$ddyb}`_ty\ZZotsrTjShmPkjiLaJIHFba`BXAV?==;;9977553311
//-IHG)?(=B%@?"=~;49z7x543s+r)pnnl*)(h~g|ec!b``uzyxZpYnWUqpoQgPeNLLJJHHFFD`C^]??ZYR;V986R5PO
N1LKD-,+))''%%##!=<;{3z1xvvttrrppn,mkk"'&f${dy~a|{^\\wvunsVUpSQQfkNihKfIdcb[`C^A\[Z<R;PUT6RQ
4I2GL/JIH+FEDC<%@#"~~|:98x0w.ussqqoommkkiigge#"!aw`uz]xwYYtsrqpinQlOjMKK`eHcbaD_^]\[TY<;:O86
R5PO2MF/J-,A*((&BA@"8!6}{{yywwu3trr).-,l$k"igge#dbbw|{z\r[pYWsVqpRRgPkNMKgJedcF[D_B]\>ZS<QV9
TS55POH1L/J-++@E(CBA$?>7~;|{y76v4-t+rp.-,l$k"iggeecca}`{z\\wvunsVqTonPleNchKfedGba`Y^A@?==;W
VU7M6K42N1//DIHG)?(=&$$""~~||zzxxv4u21rpp-,+*#ji&geez!b}|_z]xwvunWrUpSQQfkjiKaJ_HFbE`_AA\[ZY
XQV9T764P3NML/JIHGF?D'&%##!=<;{3z1x/v-2s0/p-n+*)('&}f#d!b``uz][[pYnsrqSiRgPNNLLJfedF\EZCA]\[
=S<Q:886644220L/JI++FEDCBA@9>!<}:{yy05v321r/.-,+*)"'hg$e"!b``{zyxwvutmVqToRPlkNihgfedcb[DCBW
@>><<::8TSR4J3H1//--++))''%A@?!7~5:{87x5.u2s0/o-&m$)j'&ff{"c~a|_]]rwZutsVkpSnQPeNchgfH^G\ECC
AA??==;;997S6QPO2MF/J-,A*((&&$$""~~||zzxx/4u210q.-&+l)jig%$d"ybw|_zyx[vunsVUTRnmlNdMbKIIGGEa
`B^W@U><<::88664PON0F/DI,GF)D'BA@9"=~}4{yy0wuus10/o'n%ljjh&%$dzcxa__]][wvXtmVkTRRPPNjMhgIIdc
baZ_B]@[ZY;Q:OT7RQP3NMLKDI,+FED&<%:#!=<|:3z1xvvttrrppnnlljjh&g$#d!b}|{zyr[vYXVrqpRhQfOMMKKII
GGEECCAA??=Y<WV88SRQPONGL/J-HGF(>'<A$?>=~;:9876/4utsq/pnn%*)i'~g|ec!b}|_z]xwvutsrkToRQOkjiKa
J_HFFDDBB@\?==RWV8TM6K42NML.D-B+))'C&A@""=<;:9876/4u2s0/o-&m$)j'&%f#"!~}|{zsx[vuXsVqSShmlOdM
hKJ_HFFDDBB@\[Z<R;P977553311//--++)E(CBA:#"!6}{{yyww.us10/o'n%ljjhhf$#"bxav_]yxwYoXmVTTRRPPN
NLhKfeHFF[DC^]\>T=RW:UT7R5J32M0KJ-++F?D'&A@">7~5:{87xvv32+rq.omm$)j'&g$e"!xa|_^\xwYunWlUSonm
OeNcLJJHHFFDDBB@@>Z=XW99TSRKP3N10.J-HGF)DCB;@#"!6}{{y765u-t+rppn,+*j"i~geeccaa__]][wZutWUUpo
nmfONihgI_H]bE`_B]@[ZYXQ:9T755JO200E.CHGF(>'<A$?>!<}:9876/4utsq/.-m%l#jhhf$#c!xav_]][[YuXsrU
pSnmlkjchKJIGGEa`_AW@U><<::8TS5QJ3H1/KJI+A*?(&&$@#>=~||9876543,1rq.-m+$k"'h%$e"c~}|{zyxqvYXs
VTTinmlNdMbgJedGbE`_^]\[ZYR;:9775QPO1M0EJ-HG*E>'B%$9"~~||zzxxvvttrrppn,m*)ii~%f#d!b``uz]xwvY
nsVqTSQQOkjiKaJ_HFFDDBB@@>><<::886R5PON1LE.-H+))>'<A@?!7~5|z8y65v3t10).onmkkiiggeec!~}_u^s\Z
ZXXVrqSohQfOMMKKIeHcbE`C^]V[>Y<;P977553311//--++)EDC%;$9"7~5:{876w432+rqpnn%lj('g%|ezcaa__]y
\wvYtWrqpohmPkNMKKIIGGEECCAA??==;W:88MR5PON1LKJIHA*)(=&$$">!<;|zz765432+0qpo&+*)i!h}fd"!~`v_
t][[YutsUkTiRPPNNLLJJHdGbaD_B]\[ZYXQV9T76KP3NM//JIHGFED=&A$#8=~||3876v.u,1r/.-n+*)('&%|edcx}
|^zs\qZXXVrqSohQfkNihKf_HGF[`_^@V?TY<::OT7RQ4OH10K.IH+F)>'B%@#!=<}4{z765u-t+r).o,+l)j'~%fedb
~a__ty\wvYtWrkpSRQfkNLLafIdcFaD_^W@?Z=;;PU8SR533NMLEJ-,G*((=B%@?"=~;:927xw43s10q(-,+k#j!hffd
db~}_{t]r[YYWWUUSSQQOkNihKfIdcbaZCBA?[><<QVU7SL5J311//--++))''%%#?"=<}{{87654-2srqo-n+*k(i&%
$#"y~a|_^sxwYunWlUSSQQOkjiKaJ_HFFDDBB@@>><<:V9TSR5PONMLKD-,+@)>C&$$9>!<;|92yx543s+r)p'n%lj(i
&%f#zc~a`u^\\ZZXXVVTTRRPPNNLLafIdcbEZ_B]@?=Y<::OT7RQP3HM0K.-BG*((=B%@?>!<5|{z1xvvt2sqq(-,+k#
j!hffddbb``^^\\ZvuWslUjoRmlOMMhg`eHGbECCX]@[Z=X;VUNS6Q4O200EJ-++@E(CBA$?>=6}:{8yww.3t10pp-,+
*#(i&g$eccx}`{zy\wvutmrUpSnQOOdiLJJ_dGba`C^]\[ZS<W:U866KP3NM//JIHGFE>C&A$?"~~5:{876w43210/(-
n+l)jhh}$eccx}`{zy\wvutsrqjSnQlOMMbgJedFFa`_^]\[ZSX;V9T755JO2MLK.IHGFEDCB;@#"!6}{{y765u3t+0q
.-n+$k(i&geez!b``uz]xwvYnWrUpSQQfkNihJJe^cFaD_B@@UZ=XWV9TMR5P3200..,,**((&&$$""~<;:z2y0w.3t1
0/p-,%l)jiggeecca}|{]s\qZXXVVTponPfOdMKgfeG]F[DBB@@>><<:V9TS55PONGL/J-H+))>C&A@?"=<;49zy6w43
tr*qp-n+*kii~g$e"ca}|_t]x[vuWslUjoRmlNNibgJeHcFDDY^A\[Z=XQV9T76K4INML.D-B+))''%%##!!}}{9z765
v32+rqp'nllj('&f|ezca}|{]s\qZXtsrTjShmPkjMKKfed]bEDCA]@[Z=X;VUTMR5P320LKJ,B+@)''%%##!!}}{{yy
wwu321q)p',m*)(i&%$#zc~a|{]yr[pYWsVqpohQlOjMKK`I^cbD`YBW\?ZYX;PU8S653311//--+GFE'=&;$""~~|:9
y70w.ussqqo-n+*)j!&g$e"!~`v{zyx[voXWrUpoRmPkdMhKJHdGEEZ_^]?U>S<:VUT6L5J311//--++))''%A$?>~~;
:38y6wvt2s0/.o,+$)j'hgeeccaa_{zy[qZoXmVTTRRPPeNLLJJHdGba`C^]\U>=X;VU86L5P32GLKJ,B+@)''%%##!=
~||3876v.u,sqqo-,+k#j!&g$#ccx}`{^y\ZZotWrqpShmPkNMKK`edFb[DYB@@>><<::88664PON0F/D-BG*EDC&A:#
"!}}{{yyw543s+r)p'n%*k('h%f#"y~a|_^\\ZZXXmrqpRhQfOMiLJJ_dcEaZCXA?[ZY;Q:O8664PO1MF/DI,GFE(CBA
:#"=~||38y65v3,srq(-n+*k(i~%f#dcaav_t][wvuWmVkTRRPPNNLLJJHHFFD`CAAV[>YXW:UN7R542N1//D-BG*((=
B%@?>!<;49z7x5vtt+r).-,l$k"'h%$#d!~w|_z]\ZZoXVrqSohQfOMMKgfeG]F[DBB@@>ZYX:P9N75QPO1G0E.,,**(
(&B%@?!!<;:3z7xwu3t10/p-,+$kj'h%$ecc~}|{ty\wZutVrkTinQlkMMhgfed]FaD_B@@UZ=XWV9TSRQPI21LKJ,B+
@)''%%##!!}}{{yyw5v32sqq.-,+*)"'hg$#"bxav{^yx[vYtsrqpohmPONchKII^cFa`C^W@?Z=;;PU8SR5PI2M0/D-
++)EDC%;$9"~~||z876v.u,sqqoommk)j'&ff{d!ba_{^yxwZoXWVkTinmlNdMbgJedGbE`Y^A@?=YXW9O8M6442NM/K
D-B+))''%A$?>!<}:38y6wv-21q/(o&mk)(h&}f{dbb``^^\\ZZXXVVTpSnmOOjihafIdGFD`C^]\?ZYXQV9T7R533HM
LK-C,AF)DCB%@?>=6}|{2ywwuuss*q(om+*)i!h}fddbb``^^\\ZvYtsVTTonmlkdiLKJHdGbaD_B]\[ZYRW:U87L5J3
H1/KJI+A*?D'BA@9"!<}{{2765u-t+0q.-n+l#jih}f{dy~a|{^y\qZuXWlUjonmOeNcLJfIdcbE`Y^A\?><<::88664
PON0F/D-++))'CB$@9"7~||zzx6w432s0).onmk)(h&}f{db~}|^t]r[YYWWUUSSQQOOMMKgJHH]bE`_B]@[ZS<W:977
L53ON0LE.C,**((&B%@?>!<;:38y6w432r*/.-,m*)('~gf#d!~a|_zyxwpYXWUqpoQgPeNLLJJHHFFDDBB@@UZ=XW:U
8SRQPOHM0/.,HG)E>'<%#?"~~5:98x0w.ussq/p-,m*k('&%${"c~a`u^\\qZotWrqSSnmlkjibKfIdcbDZCX]@[ZY<W
VUTSRK4O21//--++)E(&&;@?!=6}4{yywwuussq/p-,+$kjiggeec!~}_u^s\ZZXtsUqjShQOkNihKII^cFEDY^]\>T=
RW:UT7R5JO21L/JI,G*E>'&A$?>!<}:3z7xwu321q)p'nlljjhhffddbb``^zyxZpYnsVqpRRmlejMhKfIGG\aD_^]@[
ZSX;:977553311//--++)E(&&;@?>~6}49z76w4u210)po,mkk"'h%$ecc~}|{ty\[vuWslUjoRmlOjMhgfe^cFaDCXA
?[><<Q:O8MRQP2H1F/--++))''%%##!!};|987x54321*qp-n+*ki!hgfd"!~`|_ty\wvYtmVqTSQmPkjiLafIHG\ECC
AA??==RWVU7M6K42NML.D-B+))'CBA#9"7~||zzxxvvttr0q.-nll)"ih%fddy~a|{^y\wpYXWUUSonmOeNcLJJHHFba
`BXAV?==;;997S6QP311LKDI,+*(D'BA$?"=<5:{z7x54uss0/.'n+l)jh&%f#"!xa`{z\xqZotWrqTRRmlkjchKJeHF
F[`C^]@[>YXWVOT7R542200..,HGF(>'<%:#8!}}{987w/v-2sqq(-n+*)j'&%$#zcb}`{z]x[vutsrqjoRQlOjiLgJe
dcba`Y^A@?=YXW9O8M6K4I20L/--BGFE'=&;@#>=~;|9876543,s0qpnnlljjhhffddbb`|_]]rwZutVVqponmlkjchK
fIdcbDZCX]@[ZY<WVUTSRQPIN1L/.,HGF(>'<%:#!!}}{{2ywwuussq/pnn%*k('&}f#d!b``uz]xwvoXsVUjonPlkNc
LafedF\EZCAA??==;;997SRQ3I2G0..,H+FED'<%$#!!};:9y1x/vttrrppnnlljjh&g$#dbb}v{^]\ZvYtsVqTohmPO
NchgIe^G\EC_^]?U>SX;VU8S6QPI210..,,*FED&<%:#!!}}{{yywwu3t10qoo,+*#(ihge#d!~a|_zyxqvYtWVTTinm
OkdMbKIedFbaDY^]\>T=R;997SR4PI2G0..,HG)E>'<A$?>~~;:981x5vus1r/.-n+*)(!hg$e"!b``{zyxwpuXsVqTR
nmPkjihg`eHcFa`B^W@UZ=XW99TSRQPOH1L/J-++@E(CBA$?>=<;:3zy654t,s*/p-,m*#jih}$#"b~av{^yx[voXWVT
TRRPPNNLhgfH^G\ECCA]\[=S<QV9TS644IN10K.,,AF)DC&A$9>!~;|98y6w4-ts0q.-n+l)"i&gfd"!~`v_t]r[pYWs
VqpoRmlejMLKIIGGEECCAA??==;W:88MRQP2H1FK.IH+F)DCB;$?"!6;|zz16w43ss0/.-&+l)j'hff{"c~}|_zyxwpu
XWVTpSQQfOdMbgfeG]F[DBB@@>><<::88664P311FKJI+A*?D'BA$?"=<;:92yx54t2+r).o,+l)"i&gfd"!~`|_ty\w
vuXmrUTShQOOMMKKIIGGEECCAA?[ZY;Q:OT7RQ4O2MF/.-B+@EDC%;$9>!<;|9z76/4uts*qoommkki'&f${dyb`|{]y
r[pYWsVqpSnQlkdiLKJ_dcbDZCXA??==;;99775QPO1G0E.,,*F)''<A@?!7~5|z8y65vtt10/(onmk)j'&g$e"!~w`_
zyxZpYnWUUSonmOeNcLJfedF\EZCXA??=Y<WV977RQPOHM0/J-++@E(CB%@#>=<;49zyxvvttrrp.-,l$k"iggeeccaa
__]][wvuWmVkpSnmPkNihgfe^GbEDYBW\[Z<R;P97S6QPO2MLKJIHAF)D'&$$""~~||z876v.u,sqqoom+*j(!h}fddb
b`|_zyx[vutsrqjoRQlOMMbgfeG]F[`C^]@[T=<WV8TS6K4IN1LK.IB+*)'CBA#9"7~||zzx654t,s*qoommkkiigge#
d!~a__t]\wvXtmVkpSnmPkNcLKJHHFFD`_^@V?T=;WVU7M6K42200..,,**(D'BA$""=6;|{zx6w43t1r/(-nmljjhhf
fd"caav{zy[qZoXVrqpRhQfOMMKgJHH]ba`BXAV?=Y<WV977RQJ321/K.IH+F)DC<%@#"7~||zzxxvvtt+0qoo&+l)('
h%$#z!b}`_]][[YYWWUqTRRglkMibK`IGcFa`_B]\[TY<W:9N7L533HMLK-C,A*((&&$$""~~||zzx6wuu,1r/.nn+*)
(!h%f#"!aw`uz]xwvYtsrqjSRQOkjiKaJ_H]F[DBB@@>Z=XW:U8SRQPOHM0/.,,**((&&$@?>~6}4{yyw5vtt+0/.n&m
$kiig%$#cybw|_]]rwZutWrUponmlkdMhgJeHcEEZ_^AV?Z=XW9UN7LQ422GL/JIHA*E('%%##!!}}{987w/v-trrppn
nlljjh&g$#ccx}`{^yxZvoXmrUponQfkNMLaJHHFba`BXAV?==;WVU7M6K4220L/--BGFE'=&;@#>=~;|92y6wvttrrp
.-,l*k"'h%$#d!~w|_^yxZvoXmrUSShmPkjMhKfed]FaD_^]?U>S<::8866442200..,H+FE''BA@?8=~;|{y7x543t1
0/.',m*kjh&geez!b}|{t]\wZXXmrqpRhQfOdMKgfeG]F[DBB@\[Z<R;P977553311//-I,**?D'BA$""7~}:98x0w.3
t10q.o&m*kjhhf$#"bxav_]][[YYWWUUSSQmPkjLLg`eHcFaDBBW\?ZYX;VOT7R54I2GLKJ,B+@)''%A$""7<;:z2y0w
uus1r/.-n+*#jihf$e"!b``{zyrwZYXmrUSShmlkMcLaJHHFFDDBB@\?==RW:UT7R5PONGL/J-,**(DCB$:#8!};:9y1
x/vttrrppnnlljjh&geez!b}|^^yxwvoXsVqpoQgPejMhgfIdcbaZCB]@[Z=;;VUTSRKP3N1LK-IB+@E(CB$$?>=<;:3
z7x5vtt+0q.-,m*)('&%|e"!b}`{]]rwvYnWVUSSQQOOMihJf_H]bE`_B]V?Z=<Q:8T755JON0LE.C,*F)DC%%:?"=~;
:9y1x/4u210q(-nml#jh&g$#dbb}|uz]\wvuWmVkpSnmPkNihafIdGFDDBB@\?==R;PUTS5K4I200.JIH*@)>'%%##!=
<|:3z1xvvttr0q.-mm*)(!h%f#"!aw`uz]xwvYtsrkTSRPPNNLhgfH^G\ECCAA??==;;997S6QP311LKJIBG*)DCB$:#
8=~;:{8y6543,1rqp'nl*)i'~g|ec!~}_u^s\ZZXXVVTTRRPPNNLhKfeHcFa`_^]V?>Y<WV97M6542NML.J-BG*ED'B;
$?"!}}{{y76v43t+0/o-&m$kiig%fddy~}_{t]rwZutVVkpSnQPNjMhgfI^cFE`CAAV[ZY;Q:OT755JO2ML/J-HA*)D'
BA$""=<5:{8y654t,10/.o,+$)jihffddbb``^^\\ZZXXVrqpRhQfkNLLafIdcFaD_^]V?>YXW9O8M6442200.JIH*@)
>'%A$?>!}}:98705vutr0q.-n+l)('&}$e"cbw`^zyxZpYnWUUSonmOeNcLJJHdcEaZCXA??==;W:UT66QPONMF/J-HG
F(>'<A$?>=~;:9870w4ut+0/o-&m$kiiggeeccaa__]yxwYoXmrUponQlkjihg`eHcFECCX]@>>SX;VUT7RQPONMFK.I
,+@E(&&;@?>~6}49zxx/4u210)p-n+ljj!&g$#"yb}`_t][wvuWmVkTRRPPNNLLJJHHFFDDB^A\[Z=R;:97SRQ3I2G0.
.,,**((&&;$">=};4{2yw5v32s0q.',m*k('g%|ezcaa__]][[YYWWUUSSQmPNNchKfeGGbaZC^A\[Z<R;PU8SRQ4ONG
0/JIH*@)>'%%:?>=}5|3zx654t,s*qo-n+*k(i&%${"cb}|{]s\qZXXmrqSohQfkNihKfIdcb[`CBAV?=YXW9O8MR5PO
200KJIHA*)(=B%@?"=~;:981xwvt210p(o&+l)(i&}f#dcaa__]][[pYWWUqpRngPeNLLJfedF\EZCAA??==R;9U8SRQ
4IN10/--++)E(CB%##>7~}|3876v.u,sqqoommkkiig%$#cybw|_zy\wZunWVUSoRPPejiKg`I^GEECCAA??=YXW9O8M
64P3NM0..IHAF)('%A$?>!<}:927x5vussqqoom+*j(!h}fddb~}|^t]r[pYnWUqTonPPkjibKfIdcbDZCX]@[ZY<WVU
N7653ONM/K.CH+FE(C&A@?>7<}|{yywwu321q)p'nlljjhhf$#"bxav_]][wZXXmrqpRhQfOMMKgJedGbE`_^]\U>=X;
VU86L5P32GL/JIHA*E('<%:#8=~;:9z16wvus10p.-n%*)i'~g|ecca}`^^sxwYunWlUSoRmlOjMhaJIHFba`BXAV?==
;WV8TM6K4220LKJ,B+@)''%%##!!};|98yww43,1rq.omm$)j'&g$e"!x}`{^yxwYoXmVTTRRPlkjLbK`IGcbaCYBW@>
><<::88664P3NM//JIHA*E(C&$$9>!<;:{876/vutrrppn,m*)jhh%$#"y~a`_tyxwYoXmVTTRnmlNdMbKIIGGEECCA]
@[Z=X;VUTSLQ4O21//-IHG)?(=&$@?>~6}4{yywwuussqqo-n+*jj'&%$#zc~a`^z]xwvYtsrqpiRmPkjiKaJ_HFF[DY
B@\[=YR;PU8SRQJ321/K.,,AFED&<%:#!!}}{{yywwuussq/p-,mkk"'hgfd"c~}`{^sx[ZYWsrqSiRgPNNLLJJHHFFD
`_^@V?T=;;9U8SR5P3NMLKJC,+*((&&$$""~~5:9y70w.3t10q.o,+*)('~%fe"caav_ty\ZZotWrqToRmlkjihafIdG
F[DBB@@U><XWV8N7L53311//--++))'C&$$9>!<;:{8765432+rqpnnl*)(h~g|eccaa__]][[YutsUkTinQlkNibKJe
HFF[`C^]@[T=X;:88M64PON0F/D-++))''%%##!!};|98xx/4u2s0qoo&+l)('h}$edcx}|^zy\qvYtsVqTohQlOjMKK
`eHcbDD_^W\?Z=X;99NS6QPO2MLEJ-,G*((=BA#?8!6;|98y6w432+r/p-nll#(i&%ee"!~}v{^y\wZXXmrUponQlkji
bgJIdGbaD_B]\[ZYR;VU8S6Q332MLKJIB+F)D'%%:?"=<||987654-2s0q.omm$)j'&%f#"!~}|uz]x[vYWWlqpoQgPe
NLLJJHHFFDDBB@@>><X;VUTM65PON0F/D-+GFE'=&;$">=};4{2ywwuussqqoommk)('g}f{"c~}`{^s\wZYWsrTpiRg
lOjiKKf_dGbE`CAAV[>YXW:UNS65P311FKJ,HA*?D'BA$?"=<5|9z7xvv-2s0/oo,+*#(i&g$eccx}`{zy\wvunsVUpS
QQfkjiKaJ_dGbaD_B]\[ZS<W:U866KP3NM//JIHGF?D'B%@#!!6;|987x54321*/po,+*j"i~ge#"b~w`u^\\ZZXXVVT
TRRPPNNLhKfeHcFa`_^]\U>Y<W:88MR5PO11LKJIHGF?D'B%@#!!6;|987x543210/(-nm*kii~%fddybw|{z\r[puXs
rUpiRmPOMMbgfHd]F[DBB@\[Z<R;P977553311//--++)E(CBA$9"!~5|zzxxvv-21q/(o&mkkiiggeeccaa_{^yx[YY
tmrUTonmOeNchKfeHcFaZ_B]@[><<QVUT6L5JO2MLK.IHA*)(&BA@"8!6}{{yywwuussqqoommk)j'&gee"!~w|_^y\Z
ZotWrqToRmlkdiLgJIGGEa`_AW@U><<::8866442200.JIH*@)>C&A@?"=<;:3z7xwuussqqoommkk"iggeec!b``uzy
xZpYnsVqpoRmlkjibgJIH]FDDY^]\>T=R;9977553311//--++)EDC%;$9>!<;|9z765432+rq.o,+l)j'&%$#"!x}`_
^s\ZZXXVVkTRRPlkMibK`IGGEECCAA??=Y<::OT7RQ4O2MLKJIHGF?(CB%@#>~~5:9z1x5vussqqoommk)('g}f{dbb`
`^^\\ZvYWWlqTonmfONiLgfIdG\aDC^A\[>Y<QV9875QPO1G0E.,,**((&&$$""~~||z876v.u,1r/.o,m*#j'h%fddy
~a__t]r[puXsrqTonglONiLJJ_dGEEZ_B]\?Z=XWVO8S6Q422GL/--BG*EDC&A@?>7<}|{yywwuussqqoom+ljj!h}$e
"!b}`{zyxwpYtWrUSShmPNNchKfedGba`_^]V[>=<:VUT6L5J311//-IHG)?(=&$$""~~||z8yww.321q)p',m*)j'h%
$#"!~}v_z]x[YYnsVTTinQlkjMhgfedcbaZ_BA@U>S<QVUT6L5JO2ML/JC,G*)>'%%:?>=}5|3zxxv4uss*/.n,%l#jh
hffddbb``^^\x[YYnsVqpoRgPONLLJfedF\EZCAA?[Z<XQ:O86644220L/JI,**E>C&%@#!!6;|98y6w4-2sr/.-m%l#
jhhffddbb``^^\\ZZXXmrUSShmlkMcLafIdcFaD_^W@?><<::88664422G0..,,*F)''<A@?!7~5:{87x5v321*/p-n+
ljj!&%$dzcxav_ty\wvuXsrqpiRQlOMMbgJHH]bE`_B]@[ZYXWPU8S65331MLK-C,A*(DCB$:#8!}}{{yywwuussq/p-
,+l)('&%${dc~a__ty\ZZotWrqToRmlkjihg`eHcFECCXAV[ZY;Q:OT755JO2MLK.IHGFEDCB;$#>!}}49z76w4-ts0q
oo&+l)(i&g|#d!ba__]][[YYWWUUSSQQOkNLLafedF\EZ_B]\[>YR;:U8SR5P3NMFK.I,+@)''%%:?>=}5|3zxxvvttr
rppnnllj('&f|ez!b}|{^yxwpYtWVkTinQlkjcLKfIGG\aDBBW\?ZY<W:OT7R542200EJIH*@)>'%%##!!}}{{yywwu3
21q)p',m*)(i&}fe"caav{^\\qvYtsVqTonglOjMLJJHHFFDDB^]?[T=R;997755331MLK-C,A*(D'BA@#>=<5|{8yww
.3trr).o,+l)j'&%${"cba__]yxwYoXmVTponPfOdMKKIIGGEECCA]@[Z=;;VUTSRQJO210.J-HG*E(CBA@?>7<}:{zx
xv43s1*q(om+*)i!h}fddbb``^^\\ZZXXVrUponQlkjihgf_HGFD`_A]V?T=;;9UTS5K4I200..,,**((&&$@#>=~;4{
z7xvv-2s0/p-&m*kjhhffddbb``^^s\ZZXXVrUSShmlkMcLafIdcbEZ_BA\[Z<R;P977553311//--++))'C&$$9"7<}
:9z7x5.u2s0qoo&+ljj!&g$#"c~}v{^]xwvXnWlUSonmOeNcLJJHHFFDDBB@@>><<:V9TS6Q4ONMF/J-H+))>C&$$9>!
<;:{8765.3tsr)pnn%*)(h~g|#dbbw|_zy\wZutsrqjSnQlOMMbgJHH]bE`_^A\[ZYXWPU87644220LKJ,B+@)''<%##
!!}}{{yy05v32s0)p-nmkki'&f${dyb`|{]yx[putVrkTiRPlkMibK`IGGEECCAA??=Y<WVU8M6Q4311//--B+)ED&B;
$9"~~||zzxxv432r*q(-n+*)j'~%fedbb``uzyxZpYnWUUSSQQOOMMKKIIGcbaCYBW\?ZY<W:UTM65P3NM0K.IHG@E('
&$@#!!6;:9y1x/vttrrppnnlljjhhffd"c~}`{^yxwvoXsVqTRRglOMMbgJedcFa`_^]V[>=XWV8N7L53ONM/K.CHGF(
>'<A$?>!<}:98765.u2s0qoo&+ljj!&g$#"c~}|{zyxqvYtWrqpRhQfkNihg`IdGF[DBBW\[=YR;P977553311//--+G
FE'=&;@#>=<}4{8yxvvttrrppn,+*j"i~ge#"!aw`u^s\qZXtWrqpSnglONMKKIIGGEECCAA??==;WV8TM6KP311FK.I
H+F)DC<%$?"=<}:{876/4ut10/o'n%ljjhhffddbb``^^sx[vuXsVqpongPkNiLJJ_dGEEZ_B]\[>YXWVUNS65P311FK
.,,AF)DC&A$?>=<;:3z7x5vtt+0qoo&+l)('h%$#"!~}v{^y\[YYnWUUSonPleNcLJJHdGba`YB]@[ZY;Q:O866K4IN1
LKJC,+F)''<A$""7~5:98x0w.3t10qoo&+lk(igg|#d!~a|_ty\[ZXXVVkponPfOdMKKIIGGEECCAA??==;WVU7M6KP3
NM0K.IB+F)(&&$@?!=6}4{yywwuussqqoom+*)i!h}$e"!~a|{ty\[ZoXVVTTinQlkNiLgfe^GbE`CAAV[><<QV9TSR5
PONMFK.I,+))'CBA#9"7~|:98x0w.ussqqoommkki'h%$dd!~}|{zsx[vYXVrUponQlkjihg`eHcFECCAA??TYXW9O8M
6442200..,,**(DCB$:#8!};|987x543210/(o,ml#jh&%e#zcxa__]][[YYWWUUSSQmlkMcLaJ_dGba`C^]\[ZYXWPU
8S6QP2NG0E.,H+FED=&%@#!!6;|zz16w43t1r)p-nmkkiiggeeccaa__]][wvuWmVkpSnmlOjchKJeHFF[`CAAV[>YX;
V9TSL5P3N1//DI,**?D'BA@#>=<5:{8yxvv-trrp.-m+$k"igge#d!~``{zyxwpuXsVqpoQgPejMhgfIdcba`Y^A@[ZY
;Q:O86R533HML.JC,A*((&&$$""~~||zzx654t,s*/p-,m*k('&%$#zc~a`^^\\ZvuWslUjSQQOOMMKKIIGGEaDBBW\?
ZYX;VUTSRQPIN10/D-++))>C&A@#>!<;:98765.u2srp.-,l$k"iggeeccaa__]][[YutsUkTinQlkjcLgJI^cbD`YBW
@>Z=XWV9NS6Q4311F/--+GF(D=&;$">=<|4{2ywwuussqqo-n+*)j'~g$e"!a}v_t][[YutsUkTiRPPNNLLJJHH]FDDB
B@\?ZYX;VUNS6Q4311FKJI+A*?(&&$$""~~||zzxxvvt2sqq(-n+*)j'&%|ed!b}|_z]xwvunsVqTSQQOOMMKgJHH]ba
C_^AV[Z<XQ:O866442200..,H+FED'BA@?>7~}:{87x5v3210/.',ml)('g}f{dbb``^^\\ZZXXVVTponPfOdiLgfIdG
ba`_^]\U>Y<W:88MR533H1FK.IHG*EDCBA@?>7<}:9z7x5uu,10q(onm$ki'&%e{dyb``^^\\ZvutVlUjSQQOOMMKKIe
HFF[`C^]@[T=X;VUT6L5J31M0..C,AFED&<%:?"~~5:{876w.utsqq(ommkkii~%$#cybw`^^\\ZZXXVVTpSnmPkNibg
JeHGEECCAA?[Z<XQ:O866442200..,HGF(>'<A$?>=~;:3zy6wuu,1rpp',m*)j'h%$#z!b}`{zy[qZoXVrqpRhQfOMM
KKIIGGEECCAA??=Y<WVU8SRQPI21L/--BG*((=B%@?"=~;:98705v3tsqqo-,+k#j!hf$#"bxav_]][[YYWWUUSSQmPk
jiLgfedcb[DC^A??TY<::OT7RQ4O2MLKJIHG@E(C&A@?!7~5|z876v4u,10/o'n%*k('&g$#"!~}|{t]\[pYWsrqSoRg
lOjiLg`IdGFDDBB@@>><<::O866442N1//DIHG)?(=B%@?>!6;|9zyww.3trr).-,l$k"ig%$#cybw`^^\\ZZXXVVTTR
RPlOMMbgJedcFaZCBAV?TY<WV9T7RQJO2M0/--++)ED&B;$9"~<;:z2y0wuussqqoo&mkk"'h%$#d!~}v_^y\wvYWmVU
pSQQfkNLLafIdcFaDY^A@?==;;997SRQ3I2G0..,,**(D'%%:#8=~||3876v.u,1r/.o,m*#jihffddbb``^zyxZpYnW
UqpoQgPeNcLaJHdGbaD_B]\UZ=X;VUT6L5J31MLK-C,A*((&&$$""~~||zzxxv4u210q.-,%lk(igg|#dbbw|_zy\wZu
tsrkpSnQPNjihJ`I^GEaDBBW\[=YR;P9775QPO1G0E.,,**((&&$$">!<;:{87654-ts0qoo&+ljj!&g$#d!b}|{zyxq
vYtWVkTRRPPNNcLafedF\EZCAA??==;;997755331M0KJIB+*EDC%;$9"~<}{{2y0543s+r).omm$)j'&g$ezc~a`^^\
\ZZXtsrTjShQOOMMKKIIGGEaD_^]@[TY<;:8866442NM/KD-B+))''%%##!!};:z81x/4u21r/p-,%l)j'hff{"caav{
^yxwZutslqToRQOOdihgI_H]FDDBB@@>><<::88664P311FK.IHG*EDCB;$#>!<;|9z76543,1rq.o,+lj"i&g$#"b~a
v{^yxwpYXWUqpoQgPeNLLJJHHFFDDBB@@>><X;99NS6QP3N1F/.-++))''<%#?>~<5|3zxxvvttrrp.-,l$k"'h%$e"c
~w|_^][[putsUkTiRPlkjLbK`IGcbD`YBW@>><<::8866442N1//DI,GF)D'BA:#"!}}{{yy0wu32r0)p'nlljjhhffd
"!~`v_ty\wvYtWrqpinQPkjiKaJ_HFbECCXAV[ZY;Q:OT755JO2ML/J-HGFE>'&%##!!}}{{yyww.ussqqo-nll#('&f
|ez!b}|_z]xwvutmrUTSQQOkjLhaJ_HFbaC_^AV[Z<XQ:O86RQP2H1F/--++))''%A$?>!<}:98765.ut10p.'n%*kii
~g|#d!~a|_zyxwvutmrUpoRmPkMMbgfI^GFEC_^]?U>S<::8866442200..,H+FE(C<%@#"7~5:98x0w.us10/o'n%lj
jh&%e#zcxa__]][[YYWWUqpoQgPejMhgfI^GFEC_B@@UZ=XW:U8SLQ4O21F/--BG*((=BA@"8!6;|987x54-ts0qoo&+
ljj!&g$#d!b}|{ty\wZYWWUUSSQQOOMMKKIIGcbD`YBW\?==RW:UTS6QPONG0/J-HG*E(CBA@?8=~;|987w5v-2s0/.'
nmljj!&%$dzcxa_{zy[qZoXVrqSohQfOMMKKIIGGEECCA]\[=S<QV9TS6Q4I2M0KJI+A*?(&&$@?>~6}4{yyw543s+r)
pnnlljjhhffd"c~}|_zsx[ZuXVVkpSQQfOdMbgJedGbE`_XA\?Z=;;PU866KP3NML/JIHAF)D'&$$">=<|4{2ywwu321
q)p'nlljjhhff{"c~}|_zyxwpYtWVTTRRPPNNLLJJ_dcbDZCXA??==;;99775Q4ONM0KJIHG@E('BA@"8!6}{987w/v-
trrppnnlljjhhffddb~a|{^y\wvutsrkToRQOkNLLaJ_H]ba`BXAV?==;;9977553311//-I,GFE>'B%$""~~||z876v
.u,sqqoommkkiig%f#"!bw|_z]\ZZoXVrqpRhQfOMMKKIIGGEECCAA?[><<QV9TSR5PI2M0/-IH*F?(=&$$""~~||zzx
6wuu,s*qoommk)j'&%f#"y~a`_]yxwYoXmVTpSQQfkjLhaJ_HFFD`_^@V?T=;;997755331M0KJ-H+FED=&A$?"~~5:{
yy05v321r/.-,%*kji~geeccx}`{z]x[vutsrkToRmPNNchKII^cFa`_B]\[ZYXQV9TS6Q4O11FKJ-B+F)(&&$$">=<|
4{2yw543s+r)pnnlljjhhf$e"!~w`_z][[puXVVkTinQlkNLLafIHcbD`YBW\?ZY<W:OT7R5P311FKJ,HG*?D'BA@#>7
~;|{yywwuussqq(-,+k#j!hffddbb``^^\x[vutWrqjoRmPOMihgI_H]FDDB^]\>T=R;9977553311//--BG*EDC&A@?
8!~}4{2ywwuus10p.'n%ljjhhffddbb``^zyxZpYnsVqpSnQlkjibgJIdcbDZCXA?[><<Q:OTSR4J3HM0..CH+FE(C&A
@?>=6}|{yywwuussqqoo&mkkiig%fddy~}|^t]rwZutWrUponmlkdiLgJedFb[DYB@\[Z<R;P977553311//--++))'C
&A@?"=<;:9870wv32r0)p'nl*kii~%f#"c~w`_^\\ZZXXVVTTRRPPNNLhKII^cbaCYBW\?ZY<W:OT7R54220LK-IB+@)
''%A@?!7~5|zzxxvvttr0q.-,m*#ji&geez!b``uz]xwZuXsrkpSnQPNjihJ`I^GEECCAA??==;;99775Q422GL/JIH+
FED=&A$#8!6;|9870wv3trr).omm$)j'&g$ez!b}`_]]rwvuWmVkTRRPPNjihJ`I^GEECCAA??=Y<::OTSR4J3HM0KJI
,G@)(C&$$9>!}}49z76w4u21*/ponlljjh&geez!~}_u^s\ZvutVlUjSQQOOMMKKIeHcbECC^]\[TY<;:8T7RQ4O2MLK
JCH+FE(C&A##8=<}4{8y65u3,10/.'nml#jhhff{"!~`v_t][[YutsUkTiRPPNNLLJJHHFFD`C^]@[>S<;V977LQ4ON1
//JCHGF)D&&;@#>=~;|92765vussq/.-m%l#jhhf$#"bxav_]y\ZZoXW2qjiR.-e=)KgJ%^]F!~C}W@[ZY;WPbTSqK#m
2k}ih,gTF)bPO%:"K7I54zW7gvv-sr*N.'JI[6FE&fUeAR>P+u:9[[pYW3lkS/.QyON*bKJ%dcF!m_^W@>-<;W:sN6%4
]n[MjEWz,GFd'&s`#L]~6;|WW7UBeuc1qNpLJIk6FEgD1{zyQ=|*:([775WVrUSoAQ,Od*KJJ%HFF!!}}|?.Z=;QPtTq
%4o31kj/WIyfSRbC<`MLo\<|k{2V0fv-Qb=q.o&JH#G4~V$Bdy>P_;](x8vH5"3UpSh.fe=ib(J%7cF!`2B{i.Z<wuPt
'qLQn"2~YK-hBG)ccC<NM]K7}|Y{i1U/Ad2sO/LoJIkZFEhf$TA!~>+{]]88Y6XslT0B.zl,=<;(J%d]F!`}BW@yyY+d
tO8Mq5PINkjih-BTecQCa`qp>J~5XzW165eR,bO/L^m8[6j'D%UBdc>}`N^9x&vonF2qCSRmf>M*;J&8^]\n~}}@?[xY
+:Pt8S6o]3l~Y..,,*@RQ"#;
        let mut reader = DummyReader {};
        let mut writer = DummyWriter {};
let interpreter = initialise_interpreter(String::from(bottles), & mut reader, & mut writer);
        //TODO

        //check registers
        //Readout memory and compare
        let dump = interpreter.dump_memory();
        //check output size
        assert_eq!(59049, dump.len());

        //check memory wellformed
        //check if program has loaded
        let program_dump = dump.clone().resize(bottles.len(),0); //note bottles is ascii

        //TODO
        //check if crazy operator has been applied
        //go to the one before the lst of the dump, adn then try the crazy operator
        assert_eq!(
            bottles.len(),
            find_end_of_program(&dump)
        );
        return;
        for x in bottles.len()-2..59046 {
            assert_eq!(dump[x+2], crazy(dump[x],dump[x+1]));//fails here
        }



    }

}