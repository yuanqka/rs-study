use std::thread::{self, sleep};
use std::time::Duration;

pub fn main() {
    let f = || {
        for i in 1..=3 {
            println!("子线程0: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
        1
    };
    thread::spawn(f); //该线程依赖主线程的结束, 如果主线程结束, 该线程也结束
    //thread::sleep(Duration::from_secs(3));
    for i in 1..=2 {
        println!("主线程: {}", i);
        thread::sleep(Duration::from_secs(1));
    }
    let f_1 = || {
        for i in 1..=3 {
            println!("子线程1: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    };
    let _handle = thread::spawn(f_1); //新的子线程仍然在此处启动, 但是因为后面调用了thread::spawn函数的返回值, 会阻塞主线程, 使其等待子线程结束, 以获得返回值
    println!("新的子线程已启动");
    thread::sleep(Duration::from_secs(1));
    //handle.join().unwrap();

    println!("子函数线程");
    thread_of_another_fn();
    println!("子函数线程结束?");
    let _ = _handle.join().unwrap(); //主线程在这一步阻塞, 等待子线程结束
    closures_with_thread();
    send_msg_between_thread();
}

fn thread_of_another_fn() {
    let f = || {
        for i in 1..=3 {
            println!("子函数子线程0: {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    };
    thread::spawn(f); //.join().unwrap();
    for i in 1..=2 {
        println!("子函数主线程: {}", i);
        thread::sleep(Duration::from_secs(1));
    }
}

fn closures_with_thread() {
    let vector: Vec<i32> = Vec::from([1, 2, 3]);
    let f = move /*注意move是必须的, 因为这个让我们拥有了外界环境数据的所有权, 而不是引用, 从而避免了生命周期的问题 */ || {
        println!("通过闭包使用另一线程的数据, 例如主线程的数组: {:?}", vector);
    };
    let vector: Vec<i32> = Vec::from([1, 2]);
    thread::spawn(f).join().unwrap();
    println!("何意味: {:?}", vector);
}

fn send_msg_between_thread() {
    use std::sync::mpsc;
    //为了安全, 可以通过通讯实现线程间的沟通
    //思想来源于go的“不要通过共享内存来通讯；而是通过通讯来共享内存。”
    //rust中可以通过通道实现消息传递并发. 类似于管道, 或者就是rust中的管道?
    let (tx, rx) = mpsc::channel();
    //通道返回一个元组, 前者是transmitter, 后者是receiver.
    let tx1 = tx.clone();
    thread::spawn(move || {
        //注意move, 这里是为了传送发送端的工具tx
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs_f32(0.5));
        }
        thread::sleep(Duration::from_secs(1));
        tx.send("done!".to_string()).unwrap();
    });
    sleep(Duration::from_micros(10));
    // 警告：如果取消注释下面两行，会导致死锁。
    // 原因：rx.iter() 会等待所有发送端(tx, tx1)销毁才结束。此时 tx1 仍被主线程持有，
    // 主线程阻塞在此处无法继续执行去销毁 tx1，导致通道永远不会关闭。
    //let msg: Vec<String> = rx.iter().collect();
    //println!("Got: {:?}", msg);

    // drop(tx1);//注意这里如果没有在子线程中拿走tx1所有权, 就也要手动销毁tx1
    //或者像下面一样, 消耗掉tx1
    thread::spawn(move || {
        let vals = vec![
            String::from("_hi"),
            String::from("_from"),
            String::from("_another"),
            String::from("_thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs_f32(0.3));
        }
        thread::sleep(Duration::from_secs(1));
        tx1.send("done!".to_string()).unwrap();
    });
    println!("偷一个消息: {}", rx.recv().unwrap());
    match rx.try_recv() {
        Err(_) => {
            println!("悲, try_recv方法没偷到消息");
            println!("岂能空军? 睡一秒再偷一次");
            sleep(Duration::from_secs(1));
            match rx.try_recv() {
                Err(_) => println!("还是没偷到消息"),
                Ok(msg) => println!("偷到消息! : {}", msg),
            }
        }
        Ok(msg) => println!("偷到消息: {}", msg),
    };
    let msg: Vec<String> = rx.iter().collect();
    println!("Got: {:?}", msg);
    match rx.recv() {
        Err(_) => println!("不能在用recv方法偷消息了, 因为通道已经被关闭了"),
        Ok(..) => println!("还能偷?"),
    };
}
