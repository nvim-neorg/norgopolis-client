# Norgopolis Client

For information about Norgopolis, consult https://github.com/nvim-neorg/norgopolis.

This crate provides functionality to easily connect to a `norgopolis` instance and interact with
its modules.

This Rust crate provides a simple and lightweight layer for communicating with norgopolis.
To establish a connection, use the `connect` function. By default Norgopolis runs on port `62020`:

```rs
use norgopolis_client;

#[tokio::main]
async fn main() {
    let connection = norgopolis_client::connect(&"localhost".into(), &"62020".into())
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

If the `autostart-server` feature flag is enabled, this client will look for a binary called `norgopolis-server`
on the host system and will auto-execute it if a connection could not be initially established.

The server will be forked into a separate system process and will automatically shut down after 5 minutes
of inactivity.
