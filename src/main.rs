mod cache;
mod db;
mod feed;

use std::time::Instant;

use feed::*;
use sqlx::{
    postgres::{PgPoolOptions, PgRow, PgTypeInfo},
    Postgres, Row,
};

#[derive(Debug)]
struct Text {
    content: String,
    content_type: String,
    src: Option<String>,
}

impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Text {
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let content = decoder.try_decode::<String>()?;
        let content_type = decoder.try_decode::<String>()?;
        let src = decoder.try_decode::<Option<String>>()?;
        ::std::result::Result::Ok(Text {
            content,
            content_type,
            src,
        })
    }
}

impl ::sqlx::Type<::sqlx::Postgres> for Text {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("Text")
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/librepod")
        .await?;

    let result = db::get_channel("600259e547cf0cfd8e04c1144fd196bd".into(), &pool).await;
    if let Ok(result) = result {
        println!("{:#?}", result.get_unchecked::<Text, usize>(5));
    } else {
        println!("{:#?}", result.err());
    }

    Ok(())
}
