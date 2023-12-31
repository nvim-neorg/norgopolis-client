use futures::FutureExt;
use std::{future::Future, process::Command};

pub use norgopolis_protos::client_communication::MessagePack;
use norgopolis_protos::client_communication::{forwarder_client::ForwarderClient, Invocation};

use serde::de::DeserializeOwned;
use tonic::{transport::Channel, Request, Response, Status, Streaming};

pub struct ConnectionHandle(ForwarderClient<Channel>);

impl ConnectionHandle {
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

pub async fn connect(ip: &String, port: &String) -> anyhow::Result<ConnectionHandle> {
    Ok(ConnectionHandle(
        match ForwarderClient::connect("http://".to_string() + ip + ":" + port).await {
            Ok(connection) => connection,
            Err(err) => {
                if cfg!(feature = "autostart-server") {
                    Command::new("norgopolis-server").spawn()?;
                    return Ok(ConnectionHandle(
                        ForwarderClient::connect("http://".to_string() + ip + ":" + port).await?,
                    ));
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
                "test-module".to_string(),
                "func-name".to_string(),
                None,
                |response: (String,)| println!("{}", response.0),
            )
            .await
            .unwrap();
    }
}
