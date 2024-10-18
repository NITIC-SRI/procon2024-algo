use rs::client::get;

#[tokio::main]
async fn main() {
    let test_num = 0;
    let url = format!("http://127.0.0.1:3000/{}/", test_num);
    let data = get(url).await;
    println!("{:?}", data.general.patterns[0]);
}
