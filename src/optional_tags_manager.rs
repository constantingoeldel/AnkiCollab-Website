use crate::database;

pub async fn get_tags(deck: i64) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let query = "SELECT tag_group from optional_tags WHERE deck = $1";
    let client = database::TOKIO_POSTGRES_POOL.get().unwrap().get().await.unwrap();
    let tags = client.query(query, &[&deck])
        .await?
        .into_iter()
        .map(|row| row.get::<_, String>("tag_group"))
        .collect::<Vec<String>>();

    Ok(tags)
}

pub async fn add_tag(deck: i64, tag_group: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = database::TOKIO_POSTGRES_POOL.get().unwrap().get().await.unwrap();
    match client.query_one("SELECT id FROM optional_tags WHERE deck = $1 AND tag_group = $2", &[&deck, &tag_group]).await {
        Ok(_no) => return Err("Tag already exists".into()),
        Err(e) => e,
    };

    client.execute("INSERT INTO optional_tags (deck, tag_group) VALUES ($1, $2)", &[&deck, &tag_group]).await?;
    Ok(tag_group)
}

pub async fn remove_tag(deck: i64, tag_group: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = database::TOKIO_POSTGRES_POOL.get().unwrap().get().await.unwrap();
    client.execute("DELETE FROM optional_tags WHERE deck = $1 AND tag_group = $2", &[&deck, &tag_group]).await?;
    Ok(tag_group)
}