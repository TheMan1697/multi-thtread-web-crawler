use super::model::Pokemon;

const POKEAPI_URL: &str = "https://pokeapi.co/api/v2/pokemon/";

pub async fn fetch_pokemon(id: u32) -> Result<Pokemon, Box<dyn std::error::Error>> {
    let url = format!("{}{}", POKEAPI_URL, id);
    let response = reqwest::get(&url).await.map_err(Box::new)?;
    let pokemon = response.json::<Pokemon>().await.map_err(Box::new)?;

    Ok(pokemon)
}