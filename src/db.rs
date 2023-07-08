use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "estacionamento";
const COLL: &str = "carros";

const ID: &str = "_id";
const PLACA: &str = "name";
const HORAS: &str = "author";
const PRECO_HORA: &str = "num_pages";
const ADDED_AT: &str = "added_at";

//Classe DB
//Atributos da classe:
#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}
//Funções da classe:
impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("estacionamento".to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }
    pub async fn fetch_carros(&self) -> Result<Vec<Carro>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Carro> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_carro(&doc?)?);
        }
        Ok(result)
    }
    fn get_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL)
    }
    fn doc_to_carro(&self, doc: &Document) -> Result<Carro> {
        let id = doc.get_object_id(ID)?;
        let placa = doc.get_str(PLACA)?;
        let horas = doc.get_i32(HORAS)?;
        let preco_hora = doc.get_f32(PRECO_HORA)?;
        let added_at = doc.get_datetime(ADDED_AT)?;

        let carro = Carro {
            id: id.to_hex(),
            placa: placa.to_owned(),
            horas: horas as i32,
            preco_hora: preco_hora as f32,
            added_at: *added_at,
        };
        Ok(carro)
    }
}
