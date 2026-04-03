//and_then返回的就是闭包的输出, 而map则是Option<闭包的输出>

#[allow(dead_code)]
#[allow(unused_variables)]
#[derive(Debug)]
enum Food {
    Apple,
    Banana,
    Orange,
    Date,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
#[allow(dead_code)]
struct Cooked(Food);

fn eat_food(food: Option<Cooked>) {
    match food {
        Some(Cooked(Food::Date)) => println!("就拿这个考验干部?"),
        Some(x) => println!("这 {:?} 中!", x),
        None => println!("啥也木有吃到"),
    }
}

fn peel(food: Option<Food>) -> Option<Peeled> {
    Some(Peeled(food?)) //使用?解包
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn all_in_one(food: Option<Food>) -> Option<Cooked> {
    food.map(|x| peel(Some(x)))
        .and_then(|x| chop(x))
        .and_then(|x| cook(Some(x)))
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

pub fn main() {
    let apple = Some(Food::Apple)
        .map(|food| Peeled(food))
        .map(|Peeled(food)| Chopped(food))
        .map(|Chopped(food)| Cooked(food));
    let banana = None.or/*_else*/(all_in_one(Some(Food::Banana))); //or_else惰性求值, 而or立即求值其参数(all_in_one(Some(Food::Banana))变量所有权被移走), 但两者都不改变空值调用者, 也就是None还是None
    let orange = all_in_one(None);
    let mut none = None;
    let temp = Food::Date;
    let _date = none.get_or_insert_with(|| temp); //惰性求值, 不改变参数所有权, 改变none的值
    //println!("{:?}", temp);如果是get_or_insert会panic, value moved
    //注意, or, or_else, get_or_insert, get_or_insert_with说的改变或不变值是指调用方法的变量, 所有权是指方法调用的参数
    eat_food(apple);
    eat_food(banana);
    eat_food(orange);
    eat_food(all_in_one(none));
}
