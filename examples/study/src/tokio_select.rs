use tokio::sync::oneshot;

#[tokio::main]
#[test]
async fn test_tokio_select() {
    let (send1, rec1) = oneshot::channel();
    let (send2, rec2) = oneshot::channel();

    tokio::spawn(async {
        let _ = send1.send("one");
    });

    tokio::spawn(async {
        let _ = send2.send("two");
    });

    /*
        select! 允许同时等待多个计算操作，然后当其中一个操作完成时就退出等待:
    */
    tokio::select! {
        val = rec1 => {
            println!("rx1 completed first with {:?}", val);
        }

        val = rec2 => {
            println!("rx2 completed first with {:?}", val);
        }
    };
}