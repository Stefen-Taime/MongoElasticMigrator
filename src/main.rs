use mongodb::{Client, bson::doc};
use elasticsearch::{Elasticsearch, http::transport::Transport, IndexParts};
use serde::{Deserialize};
use serde_json::Value;
use tokio;
use std::fs;
use futures::stream::StreamExt;

#[derive(Deserialize)]
struct Config {
    mongodb: MongoDBConfig,
    elasticsearch: ElasticsearchConfig,
}

#[derive(Deserialize)]
struct MongoDBConfig {
    uri: String,
    database_name: String,
    collection_name: String,
    filter: Option<String>,
}

#[derive(Deserialize)]
struct ElasticsearchConfig {
    uri: String,
    index_name: String, 
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading configuration...");
    let config_contents = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_contents)?;

    println!("Connecting to MongoDB...");
    let mongo_client = Client::with_uri_str(&config.mongodb.uri).await?;
    println!("Connecting to Elasticsearch...");
    let es_transport = Transport::single_node(&config.elasticsearch.uri)?;
    let es_client = Elasticsearch::new(es_transport);

    let start_time = std::time::Instant::now();
    let result = read_from_mongodb_and_write_to_es(&mongo_client, &es_client, &config.mongodb, &config.elasticsearch).await;
    let duration = start_time.elapsed();
    println!("Data migration completed in {:?}", duration);

    result
}

async fn read_from_mongodb_and_write_to_es(
    mongo_client: &Client,
    es_client: &Elasticsearch,
    mongodb_config: &MongoDBConfig,
    es_config: &ElasticsearchConfig
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Accessing MongoDB database and collection...");
    let db = mongo_client.database(&mongodb_config.database_name);
    let collection = db.collection(&mongodb_config.collection_name);

    let filter_doc = mongodb_config.filter.as_ref()
        .map(|f| serde_json::from_str(f).unwrap_or_else(|_| doc! {}))
        .unwrap_or_else(|| doc! {});
    println!("MongoDB Query Filter: {:?}", filter_doc);
    
    let doc_count = collection.count_documents(filter_doc.clone(), None).await?;
    println!("Number of documents in MongoDB to be migrated: {}", doc_count);

    let mut indexed_count = 0;
    println!("Querying MongoDB...");
    let mut cursor = collection.find(filter_doc, None).await?;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                println!("Document retrieved from MongoDB: {:?}", document);
                let transformed_document = transform_document(document)?;
                println!("Transformed document: {:?}", transformed_document);
                if write_to_elasticsearch(&es_client, &es_config, transformed_document).await.is_ok() {
                    indexed_count += 1;
                }
            }
            Err(e) => println!("Error reading from MongoDB: {}", e),
        }
    }

    println!("Total documents indexed to Elasticsearch: {}", indexed_count);
    Ok(())
}

fn transform_document(document: Value) -> Result<Value, Box<dyn std::error::Error>> {
    let mut doc = document.as_object().ok_or("Document is not an object")?.clone();
    doc.remove("_id");
    Ok(Value::Object(doc))
}

async fn write_to_elasticsearch(es_client: &Elasticsearch, es_config: &ElasticsearchConfig, document: Value) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending document to Elasticsearch: {:?}", document);
    let response = es_client.index(IndexParts::Index(&es_config.index_name)).body(document).send().await?;
    println!("Elasticsearch response: {:?}", response);

    if response.status_code().is_success() {
        println!("Document successfully indexed");
        Ok(())
    } else {
        println!("Error indexing document: {}", response.text().await?);
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to index document")))
    }
}
