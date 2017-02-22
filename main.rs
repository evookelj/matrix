use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Matrix { data: Vec<Vec<f32>>, }

impl Matrix {
	fn new(r: usize) -> Matrix {
		let mut ret = Vec::new();
		for _ in 0..r { ret.push(Vec::new()); }
		Matrix { data: ret }
	}

	fn identity(&self) -> Matrix {
		let mut ret = Matrix::new(self.rlen());
		for i in 0..self.rlen() {
			for j in 0..self.rlen() {
				if j==i {
					ret.add_val(i,1.0);
				} else {
					ret.add_val(i,0.0);
				}
			}
		}
		return ret;
	}

	fn get_val(&self, r: usize, c: usize) -> f32 {
		return self.data[r as usize][c as usize];
	}

	fn rlen(&self) -> usize {
		return self.data.len() as usize;
	}

	fn clen(&self) -> usize {
		return self.data[0].len() as usize ;
	}

	fn print(&self) {
		let mut fin = String::new();
		//let mat = &self.data;
		for i in 0..self.rlen() {
			for j in 0..self.clen() {
				fin.push_str(&(self.data[i][j].to_string() + "\t"));
			}
			fin.push_str(&"\n");
		}
		println!("{}", fin);
	}

	fn add_val(&mut self, r: usize, val: f32) -> bool {
		if r>=self.rlen() { return false; } 
		else {
			self.data[r].push(val);
			return true;
		}
	}

	fn m_mult(&self, o: &Matrix) -> Matrix {
		let mut ret = Matrix::new(self.rlen());

		if self.clen()!=o.rlen() { 
			println!("Dimensions don't fit mult qualifications");
			return ret;
		}

		let mut val: f32;
		for r in 0..self.rlen() {
			for c in 0..o.clen() {
				val = 0.0;
				for k in 0..self.clen() {
					//println!("self.get({},{}): {}",r,k,self.get_val(r,k));
					//println!("o.get({},{}): {}\n",k,c,self.get_val(k,c));
					val += self.get_val(r,k)*o.get_val(k,c);
				}
				ret.add_val(r,val);
			}
		}
		return ret;
	}

	fn s_mult(&self, s: f32) -> Matrix {
		let mut ret = Matrix::new(self.rlen());
		for r in 0..self.rlen() {
			for c in 0..self.clen() {
				ret.add_val(r,self.get_val(r,c)*s);
			}
		}
		return ret;
	}
}

fn reg_test() {
	let mut a = Matrix::new(4);
	for i in 0..a.rlen() {
		for j in 0..3 {
			a.add_val(i,(i+j) as f32);
		}
	}
	println!("MATRIX A:");
	a.print();

	let mut b = Matrix::new(3);
	for i in 0..b.rlen() {
		for j in 0..2 {
			b.add_val(i,(j as isize-i as isize) as f32);
		}
	}
	println!("MATRIX B:");
	b.print();

	let r = a.m_mult(&b);
	println!("MATRIX R: result of AxB:");
	r.print();

	let s = r.s_mult(0.5);
	println!("MATRIX S: result of Matrix R multiplied by .5:");
	s.print();

	let i = s.identity();
	println!("MATRIX I: identity matrix of S");
	i.print();
}

struct Gmatrix { data: Matrix, }

impl Gmatrix {
	fn new() -> Gmatrix { Gmatrix { data: Matrix::new(4) } }

	fn get_val(&self, r: usize, c: usize) -> f32 { return self.data.get_val(r,c); }

	fn add_val(&mut self, r: usize, val: f32) -> bool { return self.data.add_val(r,val); }

	fn rlen(&self) -> usize { return self.data.rlen(); }

	fn clen(&self) -> usize { return self.data.clen(); }

	fn s_mult(&self, s: f32) -> Matrix { return self.data.s_mult(s); }

	fn m_mult(&self, o: Gmatrix) -> Matrix { return self.data.m_mult(&o.data); }

	fn add_pt(&mut self, x0: i32, y0: i32) {
		self.add_val(0, x0 as f32);
		self.add_val(1, y0 as f32);
		self.add_val(2, 0.0);
		self.add_val(3,1.0);
	}

	fn add_edge(&mut self,x0:i32,y0:i32,x1:i32,y1:i32) {
		self.add_pt(x0, y0);
		self.add_pt(x1, y1);
	}

	fn print(&self) { self.data.print(); }
}

//================DONE W MATRIX================

fn plot(x: i32, y: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let y2 = (250+y) as usize;
	let yf = (499-y2 as i32).abs() as usize;
	let xf = (250+x) as usize;
	screen[yf][xf] = color;
}

fn line1(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line1(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*a+b;
	while x < x1 {
		plot(x,y, screen, color);
		if d>0 {
			y += 1;
			d += b;
		}
		x += 1;
		d += a;
	}
}

fn line2(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line2(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*b+a;
	while y < y1 {
		plot(x,y, screen,color);
		if d<0 {
			x += 1;
			d += a;
		}
		y += 1;
		d += b;
	}
}

fn line7(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0;
	let mut y = y0;
	if x0>x1 { return line2(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = a-(2*b);
	while y > y1 {
		plot(x,y, screen,color);
		if d>0 { //bc deltay = A = negative
			x += 1;
			d += a;
		}
		y -= 1;
		d -= b;
	}
}


fn line8(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let mut x = x0 as i32;
	let mut y = y0 as i32;
	if x0>x1 { return line8(x1,y1,x0,y0,screen,color); }
	let a = 2*(y1-y0) as i32;
	let b = -2*(x1-x0) as i32;
	let mut d: i32 = 2*a-b;
	while x < x1 {
		plot(x,y,screen,color);
		if d<0 {
			y -= 1;
			d -= b;
		}
		x += 1;
		d += a;
	}
}

fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, screen: &mut [[[u32; 3]; 500]; 500], color: [u32; 3]) {
	let dx: f64 = (x1 as f64)-(x0 as f64) as f64;
	let dy: f64 = (y1 as f64)-(y0 as f64) as f64;
	if dx<0.0 { draw_line(x1,y1,x0,y0,screen,color); }

	let m = dy/dx;

	if (dy==0.0) && (dx==0.0) { return ; }
	if (m >= 0.0) && (m < 1.0) { line1(x0,y0,x1,y1,screen,color); } 
	else if m>=1.0 { line2(x0,y0,x1,y1,screen,color); } 
	else if (m <= 0.0) && (m > -1.0) { line8(x0,y0,x1,y1,screen,color); } 
	else if m<=-1.0 { line7(x0,y0,x1,y1,screen,color); } 
	else { println!("Should never reach this"); }
}

fn img(gm: &mut Gmatrix) {
	let mut i:i32 = -250;
	let mut j:i32 = -250;
	while i<251 {
		while j<251 {
			gm.add_edge(i, (i-j)%250, j, (j-i)%250);
			gm.add_edge((i-j)%250,i*-1,(j-i)%250,j*-1);
			j += 50;
		}
		i += 50;
		j=-250;
	}
}

fn draw(gm: Gmatrix, screen: &mut [[[u32; 3]; 500]; 500]) {
	//gm.print();
	for i in 0..gm.clen()-1 {
		let r = (i%256) as u32;
		let g = (256-(i/2)) as u32;
		let b = ((r*g)%250) as u32;
		//let b = (i%256) as u32;
		draw_line(
			gm.get_val(0,i) as i32, //x0 
			gm.get_val(1,i) as i32, 
			gm.get_val(0,i+1) as i32, //y0 
			gm.get_val(1,i+1) as i32, //y0
			screen,
			[r,g,b]);
	}
}

fn main() {
	reg_test();

	static HEADER: &'static str = "P3\n500 500 255\n";
	let path = Path::new("img.ppm");
	let display = path.display();
	//create file
	let mut file = match File::create(&path) {
        Err(why) => panic!("Error creating {} because {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };
	//write header to file
	match file.write_all(HEADER.as_bytes()) {
		Err(why) => panic!("Error writing header because {}", why.description()),
		Ok(_) => (),
	};
	//inner array: [r,g,b] for each pixel
	let mut screen: [[[u32; 3]; 500]; 500] = [[[0; 3]; 500]; 500];
	let mut gm = Gmatrix::new();
	img(&mut gm);
	draw(gm, &mut screen);

	for i in 0..500 {
		for j in 0..500 {
			match file.write_all(format!("{} {} {}\n",screen[i][j][0],screen[i][j][1],screen[i][j][2]).as_bytes()) {
				Err(why) => panic!("Error writing pixel {} {} because {}", i, j, why.description()),
				Ok(_) => (),
			};
		}
	}
	println!("Finished writing to img.ppm");
}