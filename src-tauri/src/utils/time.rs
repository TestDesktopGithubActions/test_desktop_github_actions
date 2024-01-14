use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

pub static TICKER: once_cell::sync::Lazy<
    std::sync::Arc<tokio::sync::Mutex<Option<tokio::sync::mpsc::UnboundedSender<Event>>>>,
> = once_cell::sync::Lazy::new(|| std::sync::Arc::new(tokio::sync::Mutex::new(None)));

#[derive(Debug)]
pub enum Event {
    Close,
}

const DATE_FORMATTER: &str = "%Y/%m/%d";
const TIME_FORMATTER: &str = "%H:%M:%S";
const DATETIME_FORMATTER: &str = "%Y/%m/%d %H:%M:%S";

pub fn now() -> i64 {
    Utc::now().timestamp()
}

fn from_timestamp(ts: i64) -> Option<DateTime<Utc>> {
    let time = NaiveDateTime::from_timestamp_millis(ts);
    time.map(|time| DateTime::<Utc>::from_naive_utc_and_offset(time, Utc))
}

fn format(ts: i64, fmt: &str) -> String {
    let time = from_timestamp(ts);
    time.map_or(String::new(), |time| time.format(fmt).to_string())
}

/**
 * time to date
 */
pub fn date_str(ts: i64) -> String {
    format(ts, DATE_FORMATTER)
}
/**
 * time_str
 */
pub fn time_str(ts: i64) -> String {
    format(ts, TIME_FORMATTER)
}
/**
 * datetime_str
 */
pub fn datetime_str(ts: i64) -> String {
    format(ts, DATETIME_FORMATTER)
}

/**
 * date to time
 */
pub fn date_utc_to_time(date: &str) -> i64 {
    let dt: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(date).unwrap();
    dt.timestamp()
}

pub fn now_time() -> i64 {
    Utc::now().timestamp()
}

pub(crate) async fn tick(mut rx: tokio_stream::wrappers::UnboundedReceiverStream<Event>, exp: i64) {
    tracing::info!("start tick, exp: {exp}");
    let timer_duration = tokio::time::Duration::from_secs(exp as u64);
    // let timer_duration = tokio::time::Duration::from_secs(10);
    let sleep = tokio::time::sleep(timer_duration);
    tokio::pin!(sleep);
    use tokio_stream::StreamExt as _;

    loop {
        tokio::select! {
            Some(cmd) = rx.next() => {
                match cmd{
                    Event::Close => {
                        tracing::warn!("[tick] close");
                        break
                    }
                }
            }
            _ = &mut sleep => {
                tracing::warn!("[tick] tick done");
                let pub_conn = crate::database::db::PUBLIC_SQLITE_POOL
                    .get().unwrap();
                let info = crate::database::latest_login::LatestLogin::get_one(pub_conn.get_pool().unwrap())
                    .await
                    .unwrap();
                let res = crate::service::api::event::api::account_update_token(info.token.clone()).await;
                tracing::info!("[update_token_once_expired] res: {res:?}");
                if let Some(result) = res.result{
                    let user: crate::service::api::User =
                        serde_json::from_value(result).map_err(|_|
                            crate::Error::BadRequest(
                                crate::AccountUpdateTokenError::Parse(crate::ParseError::JsonDeserialize).into(),
                            )
                        ).unwrap();
                        let command = crate::service::node::command::command_tx_generator();
                    let _ = command.send(crate::service::node::command::Event::UpdateToken(
                        user.token,
                    ));
                }

                let s = tokio::time::sleep(timer_duration);

                sleep.as_mut().set(s)
            }
        }
    }
}
