//! This crate provides functionality to easily connect to a `norgopolis` instance and interact with
//! its modules.
//!
//! This Rust crate provides a simple and lightweight layer for communicating with norgopolis.
//! To establish a connection, use the `connect` function. By default Norgopolis runs on port `62020`:
//!
//! ```rs
//! use norgopolis_client;
//!
//! #[tokio::main]
//! async fn main() {
//!     let connection = norgopolis_client::connect(&"localhost".into(), &"62020".into())
//!         .await
//!         .expect("Unable to connect to server!");
//!
//!     // Invokes a specific module's function without any parameters.
//!     // The closure will be executed for every return value provided. Return values are streamed back
//!     // over time, hence the `await`.
//!     connection.invoke("module-name", "function-name", None, |response: YourExpectedResponse| println!("{:#?}", response))
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! If the `autostart-server` feature flag is enabled, this client will look for a binary called `norgopolis-server`
//! on the host system and will auto-execute it if a connection could not be initially established.
//!
//! The server will be forked into a separate system process and will automatically shut down after 5 minutes
//! of inactivity.

use futures::FutureExt;
use std::{
    future::Future,
    io::{BufReader, Read},
    net::ToSocketAddrs,
    process::{Command, Stdio},
};

pub use norgopolis_protos::client_communication::MessagePack;
use norgopolis_protos::client_communication::{forwarder_client::ForwarderClient, Invocation};

use serde::de::DeserializeOwned;
use tonic::{transport::Channel, Request, Response, Status, Streaming};

/// Defines a connection to a Norgopolis instance.
pub struct ConnectionHandle(ForwarderClient<Channel>);

impl ConnectionHandle {
    /// Invokes a function of a given module running under Norgopolis.
    /// Returns a future to the response stream.
    ///
    /// It's recommended to use the non-raw functions if you do not need greater control
    /// over the data being sent.
    pub fn invoke_raw(
        &mut self,
        module: String,
        function_name: String,
        args: Option<MessagePack>,
    ) -> impl Future<Output = Result<Response<Streaming<MessagePack>>, Status>> + '_ {
        self.0.forward(Request::new(Invocation {
            module,
            function_name,
            args,
        }))
    }

    /// Invokes a function of a given module running under Norgopolis.
    ///
    /// On every received message a callback will be executed with the raw
    /// MessagePack return data.
    ///
    /// It's recommended to use the non-raw functions if you do not need greater control
    /// over the data being sent.
    pub async fn invoke_raw_callback<F>(
        &mut self,
        module: String,
        function_name: String,
        args: Option<MessagePack>,
        callback: F,
    ) -> anyhow::Result<()>
    where
        F: Fn(MessagePack),
    {
        self.invoke_raw(module, function_name, args)
            .then(|response| async move {
                let mut response = response?.into_inner();

                while let Some(data) = response.message().await? {
                    callback(data);
                }

                Ok::<(), anyhow::Error>(())
            })
            .await?;

        Ok(())
    }

    /// High-level function to invoke a given module's function.
    ///
    /// Will execute a callback on every received return message, but
    /// will also automatically deserialize the received MessagePack into
    /// a struct of your choice.
    ///
    /// Example:
    /// ```rs
    ///  // Automatically deserialize the MessagePack into a String.
    ///  connection.invoke("module-name".to_string(), "function-name".to_string(), None, |response: String| println!("{}", response))
    ///      .await
    ///      .unwrap();
    ///  ```
    pub async fn invoke<TargetStruct, F>(
        &mut self,
        module: String,
        function_name: String,
        args: Option<MessagePack>,
        callback: F,
    ) -> anyhow::Result<()>
    where
        F: Fn(TargetStruct),
        TargetStruct: DeserializeOwned,
    {
        self.invoke_raw(module, function_name, args)
            .then(|response| async move {
                let mut response = response?.into_inner();

                while let Some(data) = response.message().await? {
                    callback(rmp_serde::from_slice::<TargetStruct>(data.data.as_slice()).unwrap());
                }

                Ok::<(), anyhow::Error>(())
            })
            .await?;

        Ok(())
    }

    /// Invokes a function of a given module running under norgopolis.
    ///
    /// Instead of streaming return values back over time, this function waits until all possible
    /// return values have been received and then returns a vector of outputs.
    pub async fn invoke_collect<TargetStruct>(
        &mut self,
        module: String,
        function_name: String,
        args: Option<MessagePack>,
    ) -> anyhow::Result<Vec<TargetStruct>>
    where
        TargetStruct: DeserializeOwned,
    {
        let response = self.invoke_raw(module, function_name, args).await;

        let mut response = response?.into_inner();
        let mut result: Vec<TargetStruct> = Vec::new();

        while let Some(data) = response.message().await? {
            result.push(rmp_serde::from_slice::<TargetStruct>(data.data.as_slice()).unwrap());
        }

        Ok(result)
    }
}

/// Establish a connection with a running Norgopolis instance.
///
/// If the `autostart-server` feature flag is enabled, will attempt to also spawn a Norgopolis
/// instance if one is not already running.
pub async fn connect(ip: &String, port: &String) -> anyhow::Result<ConnectionHandle> {
    let address = format!("{}:{}", ip, port);

    Ok(ConnectionHandle(
        match ForwarderClient::connect("http://".to_string() + &address).await {
            Ok(connection) => connection,
            Err(err) => {
                if cfg!(feature = "autostart-server")
                    && address
                        .to_socket_addrs()?
                        .all(|socket| socket.ip().is_loopback())
                {
                    let command = Command::new("norgopolis-server")
                        .stdout(Stdio::piped())
                        .spawn()?;

                    if let Some(stdout) = command.stdout {
                        let mut buffer = BufReader::new(stdout);
                        let mut str = [b' '; 5];

                        buffer.read_exact(&mut str)?;

                        if str.starts_with(b"ready") {
                            return Ok(ConnectionHandle(
                                ForwarderClient::connect("http://".to_string() + ip + ":" + port)
                                    .await?,
                            ));
                        }
                    }
                }

                return Err(err.into());
            }
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn establish_connection() {
        connect(&"127.0.0.1".into(), &"62020".into())
            .await
            .unwrap()
            .invoke(
                "hello-world".to_string(),
                "echo".to_string(),
                Some(MessagePack::encode("hello".to_string()).unwrap()),
                |response: String| println!("{}", response),
            )
            .await
            .unwrap();
    }
}
