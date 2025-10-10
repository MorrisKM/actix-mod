use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder};
use actix_ws::Message;
use futures_util::StreamExt;
use tokio::task;


// pub async fn ws(req: HttpRequest, body: web::Payload) -> actix_web::Result<impl Responder> {
//   let (resp, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
//   actix_web::rt::spawn(async move {
//     while let Some(Ok(msg)) = msg_stream.next().await {
//       match msg {
//         Message::Ping(bytes) => {
//           if session.pong(&bytes).await.is_err() {
//             return
//           }
//         },
//         Message::Text(msg) => println!("we have received the message {msg}"),
//         _ => break
//       }
//     }

//     let _ = session.close(None).await;
//   });
//   Ok(resp)
// }


// #[get("/v1")]
// async fn chat_ws(req: HttpRequest, stream: web::Payload, chat_server: web::Data<ChatServerHandle>) -> Result<HttpResponse, Error>{
//   let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
//   //spawn websocket handler(and don't await it) so that the response is returned immediately
//   task::spawn_local(handler::chat_ws(
//     (**chat_server).clone(),
//     session,
//     msg_stream,
//   ));
//   Ok(res)
// }

#[get("/v2")]
pub async fn ws(req: HttpRequest, body: web::Payload) -> actix_web::Result<impl Responder> {
  let (resp, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
  actix_web::rt::spawn(async move {
    while let Some(Ok(msg)) = msg_stream.next().await {
      match msg{
        Message::Ping(bytes) => {
          if session.pong(&bytes).await.is_err() {
            return
          }
        },
        Message::Text(msg) => println!("we have recieved the message {msg}"),
        _ => break
      }
    }

    let _ = session.close(None).await;
  });

  Ok(resp)
}