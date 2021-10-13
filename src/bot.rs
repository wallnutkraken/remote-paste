use std::{error::Error, marker::PhantomData};

use futures::{
    channel::mpsc::{self, Receiver},
    Sink, SinkExt, StreamExt,
};
use telegram_bot::{
    Api, MessageEntity, MessageEntityKind, MessageKind, SendMessage, ToChatRef, Update, UpdateKind,
};

pub struct Bot<S: Sink<String, Error = Box<dyn Error>> + Send + Unpin + 'static> {
    client: Api,
    listen_user: String,
    output_type: PhantomData<S>,
}

impl<S: Sink<String, Error = Box<dyn Error>> + Send + Unpin + 'static> Bot<S> {
    pub fn new(api: Api, listen_user: String) -> Self {
        Self {
            client: api,
            listen_user: listen_user,
            output_type: PhantomData,
        }
    }

    pub async fn listen_and_paste(&mut self, output_sink: S) -> Result<(), Box<dyn Error>> {
        let (mut sender, receiver) = mpsc::channel(5);
        let client = self.client.clone();
        let mut stream = self.client.stream();
        let listen_to = self.listen_user.clone();
        self.listen_for_send(output_sink, receiver);
        while let Some(Ok(update)) = stream.next().await {
            if let Update {
                kind: UpdateKind::Message(msg),
                ..
            } = update
            {
                match msg.from.username {
                    Some(username) => {
                        if username != listen_to {
                            // Ignore people whomst are not
                            continue;
                        }
                    }
                    None => {
                        if let Err(err) = client
                            .send(SendMessage::new(
                                msg.chat.to_chat_ref(),
                                "No username, get a username then set it to the listen username",
                            ))
                            .await
                        {
                            println!("no_username message error: {}", err);
                        }
                    }
                };
                if let MessageKind::Text { data, entities } = msg.kind {
                    if let Some(url_text) = get_url(data, entities) {
                        sender.send(url_text).await?;
                    }
                }
            }
        }
        Ok(())
    }

    fn listen_for_send(&mut self, sink: S, receiver: Receiver<String>) {
        tokio::spawn(async {
            if let Err(err) = sink
                .sink_map_err(|e| e)
                .send_all(&mut receiver.then(|item| Box::pin(async move { item })).map(Ok))
                .await
            {
                println!("failed sending to paster: {}", err);
            };
        });
    }
}

fn get_url(text: String, entities: Vec<MessageEntity>) -> Option<String> {
    match entities
        .into_iter()
        .filter(|ent| ent.kind == MessageEntityKind::Url)
        .take(1)
        .collect::<Vec<MessageEntity>>()
        .first()
    {
        Some(url_def) => Some(
            text.chars()
                .skip(url_def.offset as usize)
                .take(url_def.length as usize)
                .collect(),
        ),
        None => None,
    }
}
