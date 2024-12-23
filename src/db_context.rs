use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Result, Surreal};

pub async fn get_database() -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: "root",
        password: "s85g1d3GA5ZH1rJ",
    })
    .await?;

    db.use_ns("namespace").use_db("database").await?;

    Ok(db)
}
// pub async fn get_database() -> Result<Surreal<Client>>{
//   let db: Surreal<Client> = Surreal::init();
//   let _ = db.connect::<Ws>("localhost:8000").await?;

//   let _ = db.signin(Root {
//     username: "root",
//     password: "root",
// })
// .await?;
// let _ = db.use_ns("products")
// .use_db("products")
// .await?;

// Ok(db)

// }
