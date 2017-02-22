pub struct Matrix { data: Vec<Vec<f32>>, }

impl Matrix {
	pub fn new(r: usize) -> Matrix {
		let mut ret = Vec::new();
		for _ in 0..r { ret.push(Vec::new()); }
		Matrix { data: ret }
	}

	pub fn identity(&self) -> Matrix {
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

	pub fn get_val(&self, r: usize, c: usize) -> f32 {
		return self.data[r as usize][c as usize];
	}

	pub fn rlen(&self) -> usize {
		return self.data.len() as usize;
	}

	pub fn clen(&self) -> usize {
		return self.data[0].len() as usize ;
	}

	pub fn print(&self) {
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

	pub fn add_val(&mut self, r: usize, val: f32) -> bool {
		if r>=self.rlen() { return false; } 
		else {
			self.data[r].push(val);
			return true;
		}
	}

	pub fn m_mult(&self, o: &Matrix) -> Matrix {
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

	pub fn s_mult(&self, s: f32) -> Matrix {
		let mut ret = Matrix::new(self.rlen());
		for r in 0..self.rlen() {
			for c in 0..self.clen() {
				ret.add_val(r,self.get_val(r,c)*s);
			}
		}
		return ret;
	}
}

pub fn reg_test() {
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

pub struct Gmatrix { data: Matrix, }

impl Gmatrix {
	pub fn new() -> Gmatrix { Gmatrix { data: Matrix::new(4) } }

	pub fn get_val(&self, r: usize, c: usize) -> f32 { return self.data.get_val(r,c); }

	pub fn add_val(&mut self, r: usize, val: f32) -> bool { return self.data.add_val(r,val); }

	pub fn rlen(&self) -> usize { return self.data.rlen(); }

	pub fn clen(&self) -> usize { return self.data.clen(); }

	pub fn s_mult(&self, s: f32) -> Matrix { return self.data.s_mult(s); }

	pub fn m_mult(&self, o: Gmatrix) -> Matrix { return self.data.m_mult(&o.data); }

	pub fn add_pt(&mut self, x0: i32, y0: i32) {
		self.add_val(0, x0 as f32);
		self.add_val(1, y0 as f32);
		self.add_val(2, 0.0);
		self.add_val(3,1.0);
	}

	pub fn add_edge(&mut self,x0:i32,y0:i32,x1:i32,y1:i32) {
		self.add_pt(x0, y0);
		self.add_pt(x1, y1);
	}

	pub fn print(&self) { self.data.print(); }
}