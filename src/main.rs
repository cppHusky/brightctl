use std::io::{BufRead,Write};
const GAMMA:f64=2.2;
const STEPS:i32=100;
#[derive(Debug)]
enum Op{
	Inc(i32),
	Dec(i32),
}
fn main()->Result<(),Box<dyn std::error::Error>>{
	let op=std::env::args().nth(1).expect("The first argument is empty!");
	let delta:i32=std::env::args().nth(2).expect("The second argument is empty!").parse().expect("The second argument must be unsigned integer!");
	let op=match op.as_str(){
		"inc"=>{Op::Inc(delta)},
		"dec"=>{Op::Dec(delta)},
		_=>{panic!("The first argument should be either `inc` or `dec`.")},
	};
	let max_brightness_path=get_path("max_brightness")?;
	let brightness_path=get_path("brightness")?;
	let max_brightness=read_val(&max_brightness_path)?;
	let previous_brightness=read_val(&brightness_path)?;
	eprintln!("Previous brightness is {previous_brightness}/{max_brightness}");
	let mut level=((STEPS as f64)*((previous_brightness as f64)/(max_brightness as f64)).powf(1./GAMMA)).round() as i32;
	eprintln!("Previous brightness level is {level}.");
	match op{
		Op::Inc(i)=>{
			level+=i;
			if level>STEPS{
				eprintln!("Warning: Current brightness level reaches maximum.");
				level=STEPS;
			}
		}
		Op::Dec(d)=>{
			level-=d;
			if level<0{
				eprintln!("Warning: Current brightness level reaches minimum.");
				level=0;
			}
		}
	};
	eprintln!("Current brightness level is {level}.");
	let mut new_brightness=((max_brightness as f64)*((level as f64)/(STEPS as f64)).powf(GAMMA)).round() as i32;
	if new_brightness==previous_brightness{
		match op{
			Op::Inc(_)=>{new_brightness=std::cmp::min(previous_brightness+1,max_brightness);}
			Op::Dec(_)=>{new_brightness=std::cmp::max(previous_brightness-1,0);}
		}
	}
	eprintln!("Current brightness is {new_brightness}/{max_brightness}");
	write_val(&brightness_path,new_brightness)?;
	Ok(())
}
fn get_path(tail:&str)->Result<std::path::PathBuf,Box<dyn std::error::Error>>{
	let mut entry=std::fs::read_dir("/sys/class/backlight")?.nth(0).unwrap().unwrap().path();
	entry.push(tail);
	Ok(entry)
}
fn read_val(path:&std::path::PathBuf)->Result<i32,std::io::Error>{
	let f=std::fs::File::open(path).expect(format!("Cannot open {}",path.display()).as_str());
	let reader=std::io::BufReader::new(f);
	Ok(reader.lines().next().unwrap()?.parse::<i32>().unwrap())
}
fn write_val(path:&std::path::PathBuf,val:i32)->Result<(),std::io::Error>{
	let f=std::fs::OpenOptions::new().write(true).open(path).expect(format!("Cannot open {}",path.display()).as_str());
	let mut writer=std::io::BufWriter::new(f);
	writer.write(val.to_string().as_bytes())?;
	Ok(())
}
