# Norgopolis Client

This crate provides functionality to easily connect to a `Norgopolis` server and interact with
its modules.

To establish a connection, use the `connect` function. By default Norgopolis
runs on port `62020`:
```rs
use norgopolis_client;

#[tokio::main]
async fn main() {
    let connection = norgopolis_client::connect(&"127.0.0.1".into(), &"62020".into())
        .await
        .expect("Unable to connect to server!");

    // Invokes a specific module's function without any parameters.
    // The closure will be executed for every return value provided. Return values are streamed back
    // over time, hence the `await`.
    connection.invoke("module-name", "function-name", None, |response: YourExpectedResponse| println!("{:#?}", response))
        .await
        .unwrap();
}
```
