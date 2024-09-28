use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("STHG WRONG");

    // создание вектора из чисел
    let mut a :Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    
    let mut step = a.len()/2;
    
    
    while step != 0{
        for i in 0..a.len(){
            for j in (0+i..a.len()-step).step_by(step).rev(){
                if a[j] > a[j+step]{
                    a.swap(j, j+step);
                }
            }
            
        }
        step /= 2;
    }   
    //можно ли сделать вывод без цикла как в python *array?
    for num in a{
        print!("{} ",num);
    }
}
