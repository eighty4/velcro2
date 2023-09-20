use std::env;

use anyhow::{anyhow, Result};
use elasticsearch::auth::Credentials;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use elasticsearch::Elasticsearch;

pub fn es_client() -> Result<Elasticsearch> {
    let host = env::var("ELASTICSEARCH_HOST").unwrap_or("127.0.0.1".to_string());
    let user = env::var("ELASTICSEARCH_USER").unwrap_or("elastic".to_string());
    let password = env::var("ELASTICSEARCH_PASSWORD").unwrap_or("elastic".to_string());
    let port = env::var("ELASTICSEARCH_PORT").unwrap_or("9200".to_string());
    let url_string = format!("https://{host}:{port}");
    let url = match Url::parse(url_string.as_str()) {
        Err(err) => return Err(anyhow!("elasticsearch url {url_string} parse error: {err}")),
        Ok(url) => url,
    };
    println!("elasticsearch connections to {url_string} as user {user}");
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport_result = TransportBuilder::new(conn_pool)
        .auth(Credentials::Basic(user, password))
        .cert_validation(CertificateValidation::None)
        .disable_proxy()
        .build();
    match transport_result {
        Err(err) => Err(anyhow!("elasticsearch transport error: {err}")),
        Ok(transport) => Ok(Elasticsearch::new(transport)),
    }
}
