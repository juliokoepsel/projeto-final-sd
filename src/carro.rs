//Model
//Atributos da classe:
#[derive(Debug)]
pub struct Carro {
    pub id: i32,
    pub placa: String,
    pub horas: i32,
    pub preco_hora: f32,
    pub added_at: DateTime<Utc>,
}
//Funções da classe:
impl Carro {
    //Construtor da classe
    pub fn new(id: i32, placa: String, horas: i32, preco_hora: f32) -> Carro {
        Carro {
            id: id,
            placa: placa,
            horas: horas,
            preco_hora: preco_hora,
        }
    }
    //Calcula o preço total de acordo com a quantidade de horas e o preço da hora
    pub fn calcular_preco(&self) -> f32 {
        return self.horas as f32 * &self.preco_hora;
    }
    //Formata todos os atributos da classe em uma string
    pub fn to_string(&self) -> String {
        return format!(
            "ID: {}, Placa: {}, Horas: {}, Preço da Hora: {}, Valor Total: {};",
            &self.id,
            &self.placa,
            &self.horas,
            &self.preco_hora,
            &self.calcular_preco()
        );
    }
    //Formata os atributos da classe em uma string para serem salvos em um arquivo
    pub fn to_file(&self) -> String {
        return format!("{}\n{}\n{}", &self.placa, &self.horas, &self.preco_hora);
    }
}