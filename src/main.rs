// subscribe to the newsletter
// send email to subscribers
// unsubscribe
// manage multiple newsletters
// segment subscribers in multiple audiences
// track opening and click rates

#[tokio::main]
async fn main() {
    newsletter::run().await;
}
