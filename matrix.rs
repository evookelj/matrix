struct Matrix {
	data: Vec<Vec<f32>>,
}

impl Matrix {
	fn new(r: usize) -> Matrix {
		let mut ret = Vec::new();
		for _ in 0..r { ret.push(Vec::new()); }
		Matrix { data: ret }
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

	fn mult(&self, o: &Matrix) -> Matrix {
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

	fn add_edge(&mut self,x0:i32,y0:i32,x1:i32,y1:i32) {
		self.add_val(0,x0 as f32);
		self.add_val(0,x1 as f32);
		self.add_val(1,y0 as f32);
		self.add_val(1,y1 as f32);
		self.add_val(2,0.0);
		self.add_val(2,0.0);
		self.add_val(3,1.0);
		self.add_val(3,1.0);
	}
}

fn main() {

	let mut a = Matrix::new(4);
	for i in 0..a.rlen() {
		for j in 0..3 {
			a.add_val(i,(i+j) as f32);
		}
	}
	a.print();

	let mut b = Matrix::new(3);
	for i in 0..b.rlen() {
		for j in 0..2 {
			b.add_val(i,(j as isize-i as isize) as f32);
		}
	}

	b.print();

	//let mut b = Matrix::new(3);
	let r = a.mult(&b);
	r.print();
}