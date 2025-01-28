//use zero2prod::run;

#[tokio::test]
async fn health_check_works(){
    dummy_test();
}

fn dummy_test() {
    let server = zero2prod::run("127.0.0.1:0").expect("failed to spawn server");
    let _ = tokio::spawn(server);
}
