use std::mem;
//pattern_matching
mod pm;
//constants/global variables cannot be mutable
const MF:u8 = 2;
//only mutable in unsafe workspaces
static mut GR:u8 = 3;
//testing

//owr crates
extern crate Coucrate;
use Coucrate::greetIt::Esp;
fn ourCrate() {
    println!("{}",Coucrate::greetIt::Eng::hey());
    println!("{}",Esp::dew());
}
//crates
extern crate rand;
use rand::Rng;
fn randomeame() {
    let mut randal = rand::thread_rng();
    let r:u8 = randal.gen();
    println!("ranrand|{:?}",r);
}
//atomic reference counting & mutex
use std::thread;//RC class cannot use with threads
use std::sync::{Arc,Mutex};//we need to use Arc
struct Peo {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}
impl Peo {
    fn new(_name:Arc<String>, _state: Arc<Mutex<String>>) -> Peo {
        Peo{name: _name, state: _state}
    }
    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();//all threads try to mutate at the same time
        state.push_str("excited");
        println!("HEY, {}|{}!", self.name, state);
    }
}
fn cuentaHilos() {
    let name = Arc::new("Elon".to_string());
    let state = Arc::new(Mutex::new("meh".to_string()));
    let pdos = Peo::new(name.clone(), state.clone());
    let t = thread::spawn(move || {
        pdos.greet();
    });
    println!("Hello, {}|{}",name, state.lock().unwrap().as_str());
    t.join().unwrap();
}
//reference counting Rc
use std::rc::Rc; //crate dat do it for ours
struct people {
    name: Rc<String>
}
impl people {
    fn new(_name:Rc<String>) -> people {
        people{name: _name}
    }
    fn greet(&self) {
        println!("HEY, {}!", self.name);
    }
}
fn cuentaCuentos() {
    let name = Rc::new("Elon".to_string());
    println!("{}|{}",name, Rc::strong_count(&name));//1 here
    {//scope of person
        let person = people::new(name.clone());
        person.greet();
        println!("{}|{}",name, Rc::strong_count(&name));//2 here
    }
    println!("{}",name);
    println!("{}|{}",name, Rc::strong_count(&name));//1
}
//lifetimes
struct Person {
    name:String
}
impl Person {
    fn getName(&self) ->&String {//lifetime Illusion
        &self.name
    }
}
struct Company<'z> { //company has the z lifetime
    name:String,
    CEO:&'z Person
}
fn lifetiming() {
    let boss = Person{name:String::from("Elon")};
    let tesla = Company{name:String::from("Tesla"), CEO:&boss};

    let mut z: &String;
    {
        let p = Person{name:String::from("kim")};
        //z= p.getName(); 
    }//ERROR: P dropped here when still was borrowed (z);
    println!("{} has {}'s lifetime!", tesla.name, boss.name);
}
//borrowing
fn borra() {
    let print_vec= |x:&Vec<i32>| {
        println!("{:?}",x);
    };
    let ve = vec![1,2,3,5];
    print_vec(&ve);
    println!("{:?}",ve);
    let mut a = 90;
    {//workspace in where b owns a's referene
        let b = &mut a; //b borrows a reference
        *b +=2;
        println!("{}",b);
    }
    println!("{}",a);
    let z = vec!['p','q','a'];
    for i in &z {
        println!("{}",i);
    }
}
//Ownership
fn jefe() {
    //for non-primitive typos
    let v = vec![1,2,3,4];//owns this memory
    let v2 = &v;//copying the pointer not the memory
    println!("{:?}",v);
    //primitive typos
    let u:i32 = 1;
    let u2:i32 = u;
    println!("{}",u);
}
//dynamic dispatch
fn print_it_to(z:&printable) {
    /*looks at the type of the arguments and decide
     what function of printable should call.*/
    println!("{}", z.format());
}
//static dispatch
trait printable {
    fn format(&self) -> String;
}
impl printable for i32 {
    fn format(&self) -> String {
        format!("{}", *self)
    }
}
fn print_it<T: printable> (z:T) {
    //monomorphisation
    println!("{}",z.format());
}
fn dispatching() {
    let a = 123;
    println!("{}", a.format());
    print_it(a);
    print_it_to(&a);
}
//overloading
#[derive(Debug)]
struct Pointi {
    x: f64, 
    y: f64
}
use std::ops::Add;
impl Add for Pointi {
    type Output = Pointi;
    fn add(self, other:Pointi) -> Pointi {
        Pointi{x: self.x + other.x, y: self.y + other.y}
    }
}
fn overIt() {
    let p = Pointi {x: 1.0, y:2.0};
    let p1 = Pointi {x: 1.0, y:2.0};
    let p2 = p+p1;
    println!("{:?}", p2);
}
//traits
trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("no talk");
    }
    fn creation(name: &'static str) -> Self;
}
struct Human {
    name:&'static str
}
struct Cat {
    name:&'static str
}
impl Animal for Human {
    fn name(&self)->&'static str {
        self.name
    }
    fn talk(&self) {
        println!("{}|Talking",self.name);
    }
    fn creation (_name:&'static str) ->Human {
        Human{name:_name}
    }
}
impl Animal for Cat {
    fn name(&self) ->&'static str {
        self.name
    }
    fn talk(&self) {
        println!("{}|Meowing",self.name);
    }
    fn creation(_name:&'static str) -> Cat {
        Cat{name:_name}
    }
}
trait Sumame<T> {
    fn sum(&self) -> T;
}
impl Sumame<i32> for Vec<i32> {
    fn sum(&self) ->i32 {
        let mut result:i32 = 0;
        for x in self {
            result += *x;
        }
        result
    }
}
fn tratos() {
    let h = Human{name:"John"};
    h.talk();
    let c = Cat{name:"Cobra"};
    c.talk();
    let hm = Human::creation("Sascar");//1st way
    let ct:Cat = Animal::creation("Dk");//2nd way
    ct.talk();
    hm.talk();
    
    let vector = vec![1,2,3];
    println!("{}",vector.sum());
}
//high order Functions 
fn sum() ->i32 {
    let x = 6;
    x
}
fn HoF() {
    if sum() < 5 {
        println!("{}",sum());
    }
    let suma = (0..).map(|x| x*x)
    .take_while(|&x| x<=sum())
    .filter(|x| 5<sum())
    .fold(0, |_,x| 4+x);
    println!("|{}",suma);
}
//Closures: storing functions on variables(functions into functions) WOW
fn closures() {
    let sh = tups;
    sh();
    let ss = 2;
    {
        let plus = |s:i32| -> i32 {
            s+ss
        };
        println!("{:?}",plus(1));
    }//ss can be use after this workspace
    println!("{}",ss);
}
//functions -> LOL
//generics
struct Point<T> {//<T,V>
    x: T,
    y: T
}
fn genoma() {
    let a = Point{x:0, y:0};//i32
    let b:Point<f32> = Point{x:0.0, y:1.0};//f32

}
//tuples
fn adding(x:i32, y:i32) -> (i32,i32){
    (x+y, x*y)
}
fn tups() {
    let x = 9;
    let y = 8;
    let adde = adding(x,y);
    println!("{}|{}", adde.0, adde.1);
    //destructuring
    let (a,b) = adde;
    println!("{}|{}", a, b);
    let adi = adding(3,4);
    let comb = (adde,adi);
    println!("{:?}",comb);
    println!("{}", (comb.1).1);
}
//strings, &str
fn strings() {
    let s:&'static str = "wtf";//&str= string slice
    //utf-8
    for i in s.chars() {
        println!("{}",i);
    }
    if let Some(first) = s.chars().nth(0) {
        println!("{}", first);
    }
    let mut letters = String::new();//allocated in heap
    letters.push('a');
    //println!("{:?}", letters);
    //let mut a = 'a' as u8
    //a as char
    //let u:&str = &letters;//conversion
    let z = letters + "b";
    //let d = letters + &z;

}
//slices: part of an array with unknown size
fn slices(slice: &mut[i32]) {
    slice[0] = 666;
    println!("{:?}", slice);
}
//vectors
fn vectors() {
    println!(" ");
    let mut ve = Vec::new();
    ve.push(1);
    ve.push(2);
    println!("{:?}",ve);
    let index:usize = 0;
    ve[index] = 666;
    println!("{}", ve[index]);
//u never know what size has the vector
    match ve.get(3) {
        Some(o) => println!("{}", o),
        None => println!("Nothing here.")
    }
    for it in &ve {
        print!("{}|", it);
    }
    ve.push(666);
    println!("{:?}", ve);
    match ve.pop() {
        Some(o) => println!("{:?}", ve),
        None => println!("Nothing to do here.")
    }
    while let Some(op) = ve.pop() {
        println!("{}", op);
    }
}
//arrays
fn arrays() {
//arrays cannot be resiezed!!
//single-dimensional
    let mut arr:[i32;5] = [0,1,2,3,4];
    //println!("{}", arr.len());
    arr[0] = 1;
    for i in arr.iter() {//printing all elemntes of the array.
        print!("{}|", i);
    }
    //we cant compare arrays of diferent size
    println!(" ");
    let bu = [0u8; 10];
    for i in bu.iter() {
        print!("{}|", i);
    }
    println!("{}", mem::size_of_val(&bu));
//multi-dimensional
    let mtx:[[u8;3];2] = [
    //2rows,3columns
    [1,2,3],[4,5,6]
    ];
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            print!("{:?}|", mtx[i][j]);
        }
    }
}
//option<T>
fn options() {
    let x = 3.0;
    let y = 2.0;
    let result:Option<f64> = 
        if y != 0.0 {
           Some(x/y)
        } else {
            None
        };
    println!("{:?}",result); //debug value
    match result {
        Some(z) => println!("{}",z),
        None => println!("None")
    }
    //if let/ while let
    if let  Some(z) = result {println!("{}", z);}
}
//unions
union intOrFloat {
    i:i32,
    f:f32
}
fn onions() {
    let mut iof = intOrFloat {i: 234};
    unsafe { //modification of an unions is an unsafe operation
        //any operation with unions is unsafe
        iof.i = 45;
        println!("{}", iof.i);
        match iof {
            intOrFloat {i:45} => println!("iof"),
            _ => print!("Other")
        }
    }
    
}
//enumerations
enum Color {
    R,
    G,
    B,
    rgb(u8,u8,u8),//tuple
    rgba {//using struct
        red:u8,
        green:u8,
        blue:u8,
        alpha:u8
    }
}
fn enumerations() {
    let c:Color = Color::rgb(0,0,0);
    match c {
        Color::R => println!("R"),
        Color::G => println!("G"),
        Color::rgb(0,0,0) => println!("Black"),
        Color::rgba{red:_, green:_, 
        blue:_, alpha:_} => println!("rgba"),
        _ => print!("other")
    }
}
//structs
struct point {
    x:f32,
    y:f32
}
struct Line {
    startPoint:point,
    endPoint:point
}
fn estructures() {
    let p = point {x:3.5, y:5.4};
    let k = point {x:7.0, y:5.5};
    let line = Line {startPoint:p, endPoint:k };
}
fn control_flow() {
    //ifes
    if MF > 0 {
        println!("Greater than.");
    } else {
        println!("less than.");
    }
    let comp = if MF > 0 {"great"} else {"less"};
    println!("{}", comp);
    println!("{}", if MF < 0 {"great"} else {"less"} );
    //Loops
    {
        let mut i = 0;
        while(i < 10) {
            print!("_");
            i +=1;
        }
        loop {
            i +=1;
            if i == 20 {
                break;
            }
        }
        for z in 0..10 {
            if z == 2 || z == 5 {
                continue;
            }
            print!("{}, ", z);
        }
        for (x,y) in (30..40).enumerate() {
            println!("{}-{} ",x,y);
        }
    }
    //match/switch-case
    let op = 10;
    match op {
        0 => print!("zero"),
        3 => print!("three"),
        4...10 => println!("+3"),
        _ => print!("NaN")
    };
}
fn heap_stack() {
    //stack storage
    let x = 16;
    //heap allocation
    let y = Box::new(5);
     println!("{}",x);
     println!("{}",mem::size_of_val(&y));
     //stack allocation, again
     let q = *y;
     println!("{}", q);
}

fn operators() {
    //arithmetics
    let mut a = 2+3*4; 
    a +=1; // *= -= -= /= %=
    let a3 = i32::pow(a, 3); //a^3
    let cubed = f32::powi(2.5, 3);
    println!("{}", cubed);
    println!("{}", std::f64::consts::PI);
    //bitwise ops
    //|, &, ^,!
    let z = 1 | 2;
    println!("{}",z);
    let two_to_10 = 1 << 10;
    println!("{}", two_to_10);
    //logicals
    let fact = 3.0 < 4.0;
    println!("{}",fact);
}
fn scopes() {
    let a = 10;
    {
        let mut a = 9;
        a = 45;
        println!("{}", a);
    }
    println!("{}", a);
}
fn main() {
    //unsigned: 0 or >0
    //signed: 0 > x < 0
    //immutable
    let a:u8 = 125;
    //mutable
    //i8,u8,i16,u16,i32,u32,i64,u64 bits 
    let mut a:i32 = -6;
    //isize/usize size of the pointer
    let z:isize = 123;
    //chars
    let d:char = 'z';
    //double-precission
    let e = 2.5;
    let e :f32 = 2.5;
    //boolean 1byte
    let g = false;
    let g = -1>0;
    let mut try_ = [1,2,3,4,5];
    println!("{}", mem::size_of_val(&a));
    println!("{}", mem::size_of_val(&z));
    println!("{}", mem::size_of_val(&e));
    println!("{}", mem::size_of_val(&g));
    operators();
    scopes();
    unsafe { //unsafe workspace
        GR = 6;
        println!("{}", GR);
    }
    heap_stack();
    control_flow();
    estructures();
    enumerations();
    onions();
    options();
    arrays();
    vectors();
    slices(&mut try_[0..3]);
    strings();
    tups();
    pm::patterns_matching();
    genoma();
    closures();
    HoF();
    tratos();
    overIt();
    dispatching();
    jefe();
    borra();
    lifetiming();
    cuentaCuentos();
    cuentaHilos();
    randomeame();
    ourCrate();
    println!("-DONE-");
}