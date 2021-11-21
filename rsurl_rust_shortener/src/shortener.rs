use nanoid::nanoid;
use mongodb::{Client, Collection, options::ClientOptions};
use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;
use mongodb::{bson::{doc, Document}, options::FindOptions};


#[derive(Debug, Serialize, Deserialize)]
pub struct Surl {
    short_url: String,
    long_url: String,
    userid: Option<String>
}


pub struct SurlCollection {
    collection: Collection<Surl>
}

impl SurlCollection {
    pub async fn new() -> SurlCollection {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

        client_options.app_name = Some("RSurl".to_string());

        let client = Client::with_options(client_options).unwrap();

        let db = client.database("rsurl");

        let collection = db.collection::<Surl>("surls");

        SurlCollection {
            collection
        }
    }

    pub async fn store(&mut self, url: &str) -> String {
        //let id = self.shortener.next_id();
        //let id = base_62::encode(url.as_ref());
        let id = nanoid!(7, &nanoid::alphabet::SAFE);
        let surl = Surl {
            short_url: id.clone(),
            long_url: url.to_string(),
            userid: None
        };
        self.collection.insert_one(surl, None).await;
        println!("{} was generated", id);
        format!("http://localhost:8000/{}", id)
        /*use url::{Url, Host, Position};
        let issue_list_url = Url::parse(url);
        match issue_list_url {
            Ok(s) => println!("{}", "OK"),
            Err(err) => println!("{}", err)
        };
        String::from("1")*/
    }

    pub async fn lookup(&self, id: &str) -> Option<String> {
        //self.collection.get(id)

        match self.collection.find_one(doc!{ "short_url": id }, None).await {
            Ok(optsurl) => match optsurl {
                Some(surl) => {
                    //self.collection.update_one(doc!{ "short_url": id, "count": 1 }, )
                    Some(surl.long_url)
                },
                None => None
            },
            Err(_) => None
        }
    }
}

pub async fn test() {

}
