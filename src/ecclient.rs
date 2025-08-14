use std::error;
use std::fmt;
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use std::time::SystemTime;

use aes::cipher::BlockDecryptMut;
use aes::cipher::KeyIvInit;
use aes::cipher::block_padding::Pkcs7;
use aes::cipher::block_padding::UnpadError;
use aes::cipher::generic_array::GenericArray;
use hex::FromHexError;
use http::HeaderValue;
use log::error;
use reqwest::Url;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde::Serialize;
use typenum::U16;
use typenum::U32;

use crate::types::Part;
use crate::types::PuzzleKey;

/// Implements a CookieStore for the sole purpose of transmitting the Everybody Codes
/// session cookie. Will not store any other cookies.
struct EcSessionCookieStore {
    // Needs to be Arc<RwLock<_>> because CookieStore must implement Send and Sync
    // https://docs.rs/reqwest/latest/reqwest/cookie/trait.CookieStore.html
    cookie: Arc<RwLock<String>>,

    // Predicate to determine whether the provided URL should be authenticated
    // with the cookie.
    should_send_cookie: Box<dyn Fn(&Url) -> bool + Sync + Send>,
}

impl EcSessionCookieStore {
    fn new(
        cookie: &str,
        should_send_cookie: Box<dyn Fn(&Url) -> bool + Sync + Send>,
    ) -> EcSessionCookieStore {
        EcSessionCookieStore {
            cookie: Arc::new(cookie.to_string().into()),
            should_send_cookie,
        }
    }
}

impl reqwest::cookie::CookieStore for EcSessionCookieStore {
    fn set_cookies(&self, _cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, _url: &Url) {}
    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        if !self.should_send_cookie.as_ref()(url) {
            return None;
        }
        match HeaderValue::from_str(
            format!("everybody-codes={}", self.cookie.read().unwrap()).as_str(),
        ) {
            Ok(hv) => Some(hv),
            Err(e) => {
                error!("failed to create HeaderValue from cookie string: {e}");
                None
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    HttpError(reqwest::Error),
    FromHexError(FromHexError),
    FromUtf8Error(FromUtf8Error),
    UnpadError(UnpadError),
    UrlParseError,
    KeyNotYetAvailable,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::HttpError(ref e) => write!(f, "an HTTP request to EC has failed: {}", e),
            Self::FromHexError(ref e) => write!(f, "failed to convert hex-encoded value: {}", e),
            Self::FromUtf8Error(ref e) => write!(f, "failed to decode UTF-8 encoded string: {}", e),
            Self::UnpadError(ref e) => write!(f, "failed to decrypt EC content: {}", e),
            Self::UrlParseError => write!(f, "failed to parse a URL"),
            Self::KeyNotYetAvailable => {
                write!(f, "puzzle for the provided key is not yet available")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::HttpError(ref e) => Some(e),
            Self::FromHexError(ref e) => Some(e),
            Self::FromUtf8Error(ref e) => Some(e),
            Self::UnpadError(_) => None,
            Self::UrlParseError => None,
            Self::KeyNotYetAvailable => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::HttpError(e)
    }
}

impl From<FromHexError> for Error {
    fn from(e: FromHexError) -> Error {
        Error::FromHexError(e)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Error {
        Error::FromUtf8Error(e)
    }
}

impl From<UnpadError> for Error {
    fn from(e: UnpadError) -> Error {
        Error::UnpadError(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(_e: url::ParseError) -> Error {
        Error::UrlParseError
    }
}

pub struct EcClient {
    base_url: String,
    base_cdn_url: String,
    client: reqwest::blocking::Client,
    seed: i64,
}

#[derive(Deserialize)]
struct UserInfoResponse {
    #[serde(rename = "penaltyUntil")]
    penalty_until_ms: i64,
    #[serde(rename = "serverTime")]
    server_time_ms: i64,
    seed: i64,
}

#[derive(Deserialize)]
struct KeyResponse {
    key1: Option<String>,
    key2: Option<String>,
    key3: Option<String>,
}

#[derive(Deserialize)]
struct PuzzleInputResponse {
    #[serde(rename = "1")]
    part_one_encrypted: String,
    #[serde(rename = "2")]
    part_two_encrypted: String,
    #[serde(rename = "3")]
    part_three_encrypted: String,
}

#[derive(Serialize)]
struct AnswerRequest {
    answer: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AnswerResponse {
    correct: bool,
    #[serde(rename = "lengthCorrect")]
    length_correct: bool,
    #[serde(rename = "firstCorrect")]
    first_correct: bool,
    #[serde(with = "serde_millis")]
    time: SystemTime,
    #[serde(rename = "localTime")]
    #[serde(with = "serde_millis")]
    local_time: Duration,
    #[serde(rename = "globalTime")]
    #[serde(with = "serde_millis")]
    global_time: Duration,
    #[serde(rename = "globalPlace")]
    global_place: i32,
    #[serde(rename = "globalScore")]
    global_score: i32,
}

fn get_me(base_url: &str, client: &Client) -> Result<UserInfoResponse, Error> {
    let url = format!("{}{}", base_url, "api/user/me");
    let response: UserInfoResponse = client.get(url).send()?.json()?;
    Ok(response)
}

impl EcClient {
    pub fn new_with_base(
        base_url: &str,
        base_cdn_url: &str,
        cookie: &str,
    ) -> Result<EcClient, Error> {
        let base_url_for_predicate = Url::parse(base_url)?;
        let should_send_cookie =
            Box::new(move |url: &Url| url.authority() == base_url_for_predicate.authority());
        // We need to use an Arc here because reqwest::ClientBuilder requires an
        // Arc<C> of CookieStore:
        // https://docs.rs/reqwest/latest/reqwest/blocking/struct.ClientBuilder.html
        let cookie_store = Arc::new(EcSessionCookieStore::new(cookie, should_send_cookie));
        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent("ec2024")
            .cookie_provider(cookie_store.clone())
            .build()?;
        let me = get_me(base_url, &client)?;
        Ok(EcClient {
            base_url: String::from(base_url),
            base_cdn_url: String::from(base_cdn_url),
            client,
            seed: me.seed,
        })
    }

    pub fn new(cookie: &str) -> Result<EcClient, Error> {
        Self::new_with_base(
            "https://everybody.codes/",
            "https://everybody-codes.b-cdn.net/",
            cookie,
        )
    }

    fn get_encryption_key(&self, key: &PuzzleKey) -> Result<KeyResponse, Error> {
        let url = format!(
            "{}api/event/{}/quest/{}",
            self.base_url, key.event, key.quest
        );
        let response: KeyResponse = self.client.get(url).send()?.json()?;
        Ok(response)
    }

    pub fn get_puzzle_input(&self, key: &PuzzleKey) -> Result<String, Error> {
        let keys = self.get_encryption_key(&key)?;
        let aes = match key.part {
            Part::One => keys.key1,
            Part::Two => keys.key2,
            Part::Three => keys.key3,
        }
        .ok_or(Error::KeyNotYetAvailable)?;
        let aes_key = GenericArray::<u8, U32>::clone_from_slice(aes.as_bytes());
        let aes_iv = GenericArray::<u8, U16>::clone_from_slice(&aes.as_bytes()[..16]);
        let cipher = cbc::Decryptor::<aes::Aes256>::new(&aes_key, &aes_iv);
        let url = format!(
            "{}assets/{}/{}/input/{}.json",
            self.base_cdn_url, key.event, key.quest, self.seed
        );
        let response: PuzzleInputResponse = self.client.get(url).send()?.json()?;
        let encrypted_text = match key.part {
            Part::One => &response.part_one_encrypted,
            Part::Two => &response.part_two_encrypted,
            Part::Three => &response.part_three_encrypted,
        };
        let mut buf = Vec::new();
        buf.resize(encrypted_text.len() / 2, 0);
        hex::decode_to_slice(&encrypted_text, buf.as_mut_slice())?;
        let result =
            String::from_utf8(cipher.decrypt_padded_vec_mut::<Pkcs7>(buf.as_mut_slice())?)?;
        Ok(result)
    }

    pub fn post_answer(&self, key: &PuzzleKey, answer: &str) -> Result<AnswerResponse, Error> {
        let url = format!(
            "{}api/event/{}/quest/{}/part/{}/answer",
            self.base_url,
            key.event,
            key.quest,
            key.part.as_u8()
        );
        let request = AnswerRequest {
            answer: answer.to_string(),
        };
        Ok(self.client.post(url).json(&request).send()?.json()?)
    }

    pub fn get_penalty_delay(&self) -> Result<Option<Duration>, Error> {
        let me = get_me(&self.base_url, &self.client)?;
        match me.penalty_until_ms - me.server_time_ms {
            ..=0 => Ok(None),
            delay_ms => Ok(Some(Duration::from_millis(delay_ms as u64))),
        }
    }
}

#[cfg(test)]
mod tests {
    use httptest::Expectation;
    use httptest::Server;
    use httptest::ServerPool;
    use httptest::matchers::all_of;
    use httptest::matchers::contains;
    use httptest::matchers::eq;
    use httptest::matchers::json_decoded;
    use httptest::matchers::matches;
    use httptest::matchers::not;
    use httptest::matchers::request;
    use httptest::responders::status_code;

    use super::*;

    static SERVER_POOL: ServerPool = ServerPool::new(2);

    fn server_url(server: &Server) -> String {
        let url = server.url("/");
        let scheme = url.scheme().unwrap();
        let authority = url.authority().unwrap();
        format!("{scheme}://{authority}/")
    }

    fn set_base_expect(server: &Server) {
        let m = all_of![
            request::method("GET"),
            request::path(matches("/api/user/me")),
            request::headers(contains(("cookie", "everybody-codes=deadbeef"))),
        ];
        server.expect(Expectation::matching(m).respond_with(status_code(200).body(r#"{"id":1337,"code":"DEADB33F","name":"johnny","country":"ua","url":"https://everybody.codes","level":15,"seed":7,"penaltyUntil":1755087853695,"badges":{"1":null,"2024":null,"2025":null},"ai":false,"streamer":false,"serverTime":1755113738573}"#)));
    }

    fn make_client(server: &Server) -> EcClient {
        let base_url = server_url(&server);
        let base_cdn_url = format!("{}{}", server_url(&server), "_cdn/");
        EcClient::new_with_base(base_url.as_str(), base_cdn_url.as_str(), "deadbeef")
            .expect("creating EC client")
    }

    #[test]
    fn test_get_client() {
        let server = SERVER_POOL.get_server();
        set_base_expect(&server);
        make_client(&server);
    }

    #[test]
    fn test_get_puzzle_input_key_not_available() {
        let server = SERVER_POOL.get_server();
        set_base_expect(&server);
        let m = all_of![
            request::method("GET"),
            request::path(matches("/api/event/2024/quest/5")),
            request::headers(contains(("cookie", "everybody-codes=deadbeef"))),
        ];
        server.expect(Expectation::matching(m).respond_with(
            status_code(200).body(r#"{"key1": "AwAwAwAwAwAwAwAwAwAwAwAwAwAwAwA="}"#),
        ));
        let client = make_client(&server);
        matches!(
            client.get_puzzle_input(&PuzzleKey {
                event: 2024,
                quest: 5,
                part: Part::Two
            }),
            Err(Error::KeyNotYetAvailable)
        );
    }

    #[test]
    fn test_get_puzzle_input_does_not_authenticate_to_cdn() {
        let server = SERVER_POOL.get_server();
        set_base_expect(&server);
        let m = all_of![
            request::method("GET"),
            request::path(matches("/api/event/2024/quest/5")),
            request::headers(contains(("cookie", "everybody-codes=deadbeef"))),
        ];
        server.expect(Expectation::matching(m).respond_with(
            status_code(200).body(r#"{"key2": "AwAwAwAwAwAwAwAwAwAwAwAwAwAwAwA="}"#),
        ));
        let cdn_server = SERVER_POOL.get_server();
        let m = all_of![
            request::method("GET"),
            request::path(matches("/assets/2024/5/input/7.json")),
            request::headers(not(contains(("cookie", "everybody-codes=deadbeef"))))
        ];
        cdn_server.expect(Expectation::matching(m).respond_with(status_code(200).body(
            r#"{
                "1": "2ae06416829972cd3a095a35961d7464ca637f4a671677c6176b39967ff10f38c107f7aa6cc03e6174792d9eea1ec792",
                "2": "2ae06416829972cd3a095a35961d7464868838a10267a6f4c53f55660f9db6d02989c4df830ce94c5cedab6476f44080",
                "3": "2ae06416829972cd3a095a35961d746471867b81e5652c50e90d0ebbdc01ad1b7b863757e385f2c6bb6c5ead02692d15"
        }"#)));
        let base_url = server_url(&server);
        let base_cdn_url = server_url(&cdn_server);
        let client = EcClient::new_with_base(base_url.as_str(), base_cdn_url.as_str(), "deadbeef")
            .expect("creating EC client");
        assert_eq!(
            "Hello, I'm your input too.

Wowzers.",
            client
                .get_puzzle_input(&PuzzleKey {
                    event: 2024,
                    quest: 5,
                    part: Part::Two
                })
                .unwrap()
        );
    }

    #[test]
    fn test_get_puzzle_input() {
        let server = SERVER_POOL.get_server();
        set_base_expect(&server);
        let m = all_of![
            request::method("GET"),
            request::path(matches("/api/event/2024/quest/5")),
            request::headers(contains(("cookie", "everybody-codes=deadbeef"))),
        ];
        server.expect(Expectation::matching(m).respond_with(
            status_code(200).body(r#"{"key2": "AwAwAwAwAwAwAwAwAwAwAwAwAwAwAwA="}"#),
        ));
        let m = all_of![
            request::method("GET"),
            request::path(matches("/_cdn/assets/2024/5/input/7.json")),
        ];
        server.expect(Expectation::matching(m).respond_with(status_code(200).body(
            r#"{
                "1": "2ae06416829972cd3a095a35961d7464ca637f4a671677c6176b39967ff10f38c107f7aa6cc03e6174792d9eea1ec792",
                "2": "2ae06416829972cd3a095a35961d7464868838a10267a6f4c53f55660f9db6d02989c4df830ce94c5cedab6476f44080",
                "3": "2ae06416829972cd3a095a35961d746471867b81e5652c50e90d0ebbdc01ad1b7b863757e385f2c6bb6c5ead02692d15"
        }"#)));
        let client = make_client(&server);
        assert_eq!(
            "Hello, I'm your input too.

Wowzers.",
            client
                .get_puzzle_input(&PuzzleKey {
                    event: 2024,
                    quest: 5,
                    part: Part::Two
                })
                .unwrap()
        );
    }

    #[test]
    fn test_post_answer() {
        let server = SERVER_POOL.get_server();
        set_base_expect(&server);
        let m = all_of![
            request::method("POST"),
            request::path(matches("/api/event/2024/quest/6/part/1/answer")),
            request::headers(contains(("cookie", "everybody-codes=deadbeef"))),
            request::body(json_decoded(eq(serde_json::json!({
                "answer": "forty_two",
            })))),
        ];
        server.expect(Expectation::matching(m).respond_with(
            status_code(200).body(r#"{"correct":true,"lengthCorrect":true,"firstCorrect":false,"time":1755169141515,"localTime":79507010,"globalTime":23803141515,"globalPlace":797,"globalScore":0}"#)
        ));

        let client = make_client(&server);
        let response = client
            .post_answer(
                &PuzzleKey {
                    event: 2024,
                    quest: 6,
                    part: Part::One,
                },
                "forty_two",
            )
            .unwrap();
        assert_eq!(
            AnswerResponse {
                correct: true,
                length_correct: true,
                first_correct: false,
                time: UNIX_EPOCH + Duration::from_millis(1755169141515),
                local_time: Duration::from_millis(79507010),
                global_time: Duration::from_millis(23803141515),
                global_place: 797,
                global_score: 0,
            },
            response
        );
    }
}
