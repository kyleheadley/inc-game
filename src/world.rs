use std::fmt;

#[derive(Clone)]
pub struct Filler{
	fill: f64,
	rate: f64,
	max: f64,
}

impl Filler{
	pub fn new(fill: f64, rate: f64, max: f64) -> Filler {
		Filler{fill,rate,max}
	}
	pub fn is_empty(&self) -> bool {
		self.fill == 0.0
	}
	pub fn amount(&self) -> f64 {
		self.fill
	}
	pub fn max(&self) -> f64 {
		self.max
	}
	pub fn over_max(&self) -> f64 {
		let over = self.fill - self.max;
		if over > 0.0 {over} else {0.0}
	}
	pub fn fill(&self) -> Filler {
		if (self.fill >= self.max && self.rate > 0.0)
		|| (self.fill <= 0.0 && self.rate < 0.0) {
			self.clone()
		} else {
			let try_fill = self.fill + self.rate;
			let fill = if try_fill > self.max && self.rate > 0.0 {
				self.max
			} else if try_fill < 0.0 && self.rate < 0.0 {
				0.0
			} else { try_fill };
			Filler{ fill: fill, ..self.clone() }	
		}
	}
	pub fn set(&self, amt: f64) -> Filler {
		Filler{ fill: amt, ..self.clone() }
	}
	pub fn set_rate(&self, amt: f64) -> Filler {
		Filler{ rate: amt, ..self.clone() }
	}
	pub fn set_max(&self, amt: f64) -> Filler {
		Filler{ max: amt, ..self.clone() }
	}
	pub fn force_add(&self, amt: f64) -> Filler {
		Filler{ fill: self.fill + amt, ..self.clone() }
	}
	pub fn add(&self, amt: f64) -> Result<Filler,f64> {
		let fits = self.max - self.fill;
		if amt <= fits { 
			Ok(Filler{ fill: self.fill + amt, ..self.clone() })
		} else { 
			Err(fits)
		}
	}
	pub fn add_max(&self, amt:f64) -> Filler {
		Filler{max: self.max + amt, ..self.clone()}
	}
	pub fn force_take(&self, amt:f64) -> Filler {
		Filler{ fill: self.fill - amt, ..self.clone() }
	}
	pub fn take(&self, amt: f64) -> Result<Filler,f64> {
		if self.fill >= amt {
			Ok(Filler{ fill: self.fill - amt, ..self.clone()})
		} else {
			Err(self.fill)
		}
	}
}

impl fmt::Display for Filler{
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}/{:.2}", self.fill, self.max)
    }
}

#[derive(Clone)]
pub struct World{
	people: Filler,
	food: Filler,
	land: Filler,
	wild: Filler,
	hermit: Filler,
}

impl World {
	pub fn new() -> World {
		World{
			people: Filler::new(0.0,-0.001,10.0),
			food: Filler::new(10.0,0.0,10.0),
			land: Filler::new(10.0,0.0,10.0),
			wild: Filler::new(0.0,0.01,0.0),
			hermit: Filler::new(0.0,0.001,0.0),
		}
	}
	pub fn update(&self) -> World {
		let unused = self.land.amount() - self.food.amount();
		let clearing = if unused > 5.0 { 1.0/unused } else{ 0.2 };
		let overcrowding = self.people.over_max();
		let into_woods = !self.wild.is_empty() && overcrowding > 0.0;
		let people = self.people.set_max(self.land.amount());
		let food = Filler::new(
			self.food.amount(),
			self.people.amount()*0.001 - overcrowding*0.004,
			self.land.amount()
		);
		let land = self.land.set_rate(clearing * 0.1);
		let wild = self.wild.set_max(land.max() - land.amount());
		let hermit = self.hermit.set_max(wild.amount());
		World{
			people: if overcrowding > 0.0 {people.fill()} else {people},
			food: food.fill(),
			land: land.fill(),
			wild: wild.fill(),
			hermit: if into_woods {hermit.fill()} else {hermit},
		}
	}
	pub fn title(&self, id: usize) -> String {
		match id {
			1 => {format!("food")}
			2 => {format!("birth")}
			3 => {format!("war")}
			_ => {format!("unused")}
		}
	}
	pub fn click(&self, id: usize) -> World {
		match id {
			1 => {
				let food = match self.food.add(1.0) {
					Ok(f) => f,
					Err(fit) => self.food.force_add(fit),
				};
				World{food: food, ..self.clone()}
			},
			2 => {
				let (people,food) = if let Ok(f) = self.food.take(10.0) {
					(self.people.force_add(1.0),f)
				} else { (self.people.clone(), self.food.clone())};
				World{
					people: people,
					food: food,
					..self.clone()
				}
			},
			3 => {
				let deaths = (self.people.amount()/2.0).floor();
				let people = self.people.force_take(deaths);
				let land = self.land.add_max(deaths);
				World{
					people: people,
					land: land,
					..self.clone()
				}

			}
			_ => self.clone(),
		}
	}
	pub fn text(&self) -> String {
		format!("\
			food: {}\n\
			people: {}\n\
			land: {}\n\
			wild_growth: {}\n\
			wild_people: {}\n\
		",self.food,self.people,self.land,self.wild,self.hermit)	
	}

}