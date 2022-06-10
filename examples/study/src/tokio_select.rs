use tokio::sync::oneshot;

#[tokio::main]
#[test]
async fn test_tokio_select() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("one");
    });

    tokio::spawn(async {
        let _ = tx2.send("two");
    });

    /*
        select! 允许同时等待多个计算操作，然后当其中一个操作完成时就退出等待:
    */
    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }

        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    };
}