use chrono::naive::*;

pub fn generate_date_strings(year: i32, month: u32) -> Vec<String> {
	let mut lastdate;
	if month == 12 {
		lastdate = NaiveDate::from_ymd_opt(year+1, 1, 1).unwrap();
	} else {
		lastdate = NaiveDate::from_ymd_opt(year, month+1, 1).unwrap();
	}
	NaiveDate::from_ymd_opt(year, month, 1).unwrap()
		.iter_days()
		.take_while(|x| x != &lastdate)
		.map(|x| x.format("%a, %b %-d").to_string())
		.collect::<Vec<String>>()
}

#[test]
fn test_date_generation() {
	let gen_dates = generate_date_strings(2024, 3);
	let target = vec![
		"Fri, Mar 1", "Sat, Mar 2", "Sun, Mar 3","Mon, Mar 4","Tue, Mar 5","Wed, Mar 6",
		"Thu, Mar 7","Fri, Mar 8","Sat, Mar 9","Sun, Mar 10","Mon, Mar 11","Tue, Mar 12",
		"Wed, Mar 13","Thu, Mar 14","Fri, Mar 15","Sat, Mar 16","Sun, Mar 17","Mon, Mar 18",
		"Tue, Mar 19","Wed, Mar 20","Thu, Mar 21","Fri, Mar 22","Sat, Mar 23","Sun, Mar 24",
		"Mon, Mar 25","Tue, Mar 26","Wed, Mar 27","Thu, Mar 28","Fri, Mar 29","Sat, Mar 30",
		"Sun, Mar 31"
	];
	assert_eq!(gen_dates, target);
	
	let gen_dates = generate_date_strings(2024, 12);
	let target = vec![
		"Sun, Dec 1","Mon, Dec 2","Tue, Dec 3","Wed, Dec 4","Thu, Dec 5","Fri, Dec 6",
		"Sat, Dec 7","Sun, Dec 8","Mon, Dec 9","Tue, Dec 10","Wed, Dec 11","Thu, Dec 12",
		"Fri, Dec 13","Sat, Dec 14","Sun, Dec 15","Mon, Dec 16","Tue, Dec 17","Wed, Dec 18",
		"Thu, Dec 19","Fri, Dec 20","Sat, Dec 21","Sun, Dec 22","Mon, Dec 23","Tue, Dec 24",
		"Wed, Dec 25","Thu, Dec 26","Fri, Dec 27","Sat, Dec 28","Sun, Dec 29","Mon, Dec 30",
		"Tue, Dec 31"
	];
	assert_eq!(gen_dates, target);
}