use log::info;
use trillium::{Conn, Handler};

pub struct Echo {}

#[trillium::async_trait]
impl Handler for Echo {
    async fn run(&self, conn: Conn) -> Conn {
        info!("coucou");
        conn.ok("hello async-std\n")
    }
}
