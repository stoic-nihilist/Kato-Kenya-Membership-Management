use axum:: {
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use jsonwebtoken::{encode, decode, EncodingKey, Header, Validation};
use bcrypt:: { hash, verify, DEFAULT_COST };
use chrono::{Local, NaiveDate, Utc};
//use jsonwebtoken::header::Header;


const SECRET: &[u8] = b"secret";

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Member
{
    pub member_id: String,
    //pub membership_number: String,
    pub company_name: String,
    pub trading_name: String,
    pub company_type: String,
    pub registration_number: String,
    pub tax_pin: String,
    pub year_established: String,
    pub website: String,
    pub member_email: String,
    pub member_phone_primary: String,
    pub member_phone_secondary: String,
  //  pub whatsapp_number: String,
    pub physical_address: String,
    pub postal_address: String,
    pub city: String,
    pub county_state: String,
    pub country: String,
    //pub gps_coordinate: String,
    pub company_profile: String,
    //pub number_of_staff: i32,
    pub annual_turnover: String,
    pub status: String,
    pub membership_category_id: String,
    pub joining_date: String,
  //  pub expiration_date: String,
 //   pub renewal_date: String,
    pub approved_by: String,
  //  pub approval_date: String,
  //  pub created_at: String,
//pub updated_at: String,

}




#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Logs
{
    pub role_id: String,
    pub account_status: String,
    pub last_login: String,
    pub two_factor: bool,
    pub role_name: String,
    pub role_description: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct User
{
    pub user_member_id: String,
    pub user_id: String,
    pub user_first_name: String,
    pub user_last_name: String,
    pub user_email: String,
    pub user_phone: String,
    pub user_password: String,
    pub user_password_confirm : String,
    pub user_password_hash: String,
    pub role_id: String,

}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Contact
{
    pub contact_id:String,
    pub contact_member_id : String,
    pub contact_first_name : String,
    pub contact_last_name : String,
    pub contact_designation : String,
    pub contact_email : String,
    pub contact_phone : String,
    pub contact_national_id : String,

}



#[derive(Serialize, Deserialize, Default)]
pub struct LoginRequest
{
    pub user_id : String,
    pub password : String,
}


#[derive(Serialize, Deserialize, Default)]
pub struct LoginResponse
{
    pub token : String,
    pub valid : bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Claims
{
    pub sub : String,
    pub exp : usize,
}

pub struct AuthUser(pub Claims);

#[axum::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AuthUser
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection>
    {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "Missing token".into()))?;

        let token_data = decode::<Claims>
            (
                auth_header,
                &jsonwebtoken::DecodingKey::from_secret(SECRET),
                &Validation::default(),
            )
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".into()))?;

        Ok(AuthUser(token_data.claims))
    }
}




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{

    let db_url = "mysql://jeffreyy:password123@localhost/kato_testing";
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect(db_url)
        .await
        .expect("Failed to connect to database.");

    println!("Connected to database.");
    //pool.ping().await?;

    let app:Router<> = Router::new()
        .route("/login", post(login))
        .route("/members", get(get_members))
     //   .route("/members/:id", get(get_member))
      //  .route("/members", post(update_member))
        .route("/members", post(create_member))
        .route("/contact_persons", get(get_contact))
     //   .route("/contact_persons", post(create_contact))
        .route("/users/:id", get(get_user))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .with_state(pool);


        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server online. \nListening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    
    Ok(())
}

async fn update_member(
    AuthUser(_claims): AuthUser,
    State(pool) : State<MySqlPool>,
    Json(payload): Json<Member>
)

{

}


async fn login(
    State(pool) : State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<serde_json::Value>)>

{

    //fetch hashed password from db
    let row = sqlx::query_as::<_, (String,)>(
        "SELECT password_hash FROM users WHERE user_id = ?"
    )
        .bind(&payload.user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid Credentials"}))))?;

    //compare passwords and validate
    let valid = bcrypt::verify(&payload.password, &row.0)
        .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))))?;

    if !valid
    {
        return Err((StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid Credentials"}))));
    }

    //build jwt
    let expiry = SystemTime::now()
        .duration_since(UNIX_EPOCH).unwrap()
        .as_secs() as usize + 1800;

    let claims = Claims
    {
        sub : payload.user_id.clone(),
        exp : expiry,
    };

    let token = encode
        (
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET),
        )
        .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))))?;


    Ok(Json(LoginResponse{ token, valid }))

}


async fn get_members(
    AuthUser(_claims) : AuthUser,
    State(pool): State<MySqlPool>,

) //-> Json<Vec<Member>>
{
    /*let members = vec![
        Member { id: 1, name: "Alice".into() },
        Member { id: 2, name: "Bob".into() },
    ];
    Json(members)

     */
}


/*
async fn get_member(
    AuthUser(_claims) : AuthUser,
    State(pool): State<MySqlPool>,
    Json(payload): Json<Member>

) -> Result<Json<Member>, (StatusCode, String)>
{

    let row = sqlx::query_as::<>
        (
            "select * from members where member_id = ?"
        )
        .bind(&payload.member_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid Credentials"}))))?;

    Ok()


}


 */
/*
async fn create_contact(
    AuthUser(_claims) : AuthUser,
    State(pool): State<MySqlPool>,
    Json(contact): Json<Member>) //-> Json<Member>
{
    let contact_id = rand::random_range(1000..9999).to_string();

    println!("Adding contact: {:?}", contact_id);

    sqlx::query(
        "insert into contact_persons (contact_id, member_id, first_name, last_name, designation, email, phone, national_id) values (?, ?, ?, ?, ?, ?, ?, ?)",
    )
        .bind(&contact_id)
        .bind(&contact.contact_member_id)
    .bind(&contact.member_id)
        


}

*/
async fn get_contact(
    AuthUser(_claims) : AuthUser,
    State(pool): State<MySqlPool>,

) //-> Json<Vec<Member>>
{


}


async fn create_member(
    AuthUser(_claims) : AuthUser,
    State(pool): State<MySqlPool>,
    Json(member): Json<Member>) -> Result<Json<Member>, (StatusCode, Json<serde_json::Value>)>
{


    let member_id = rand::random_range(1000..9999).to_string();
    let local_date: NaiveDate = Utc::now().date_naive();

    println!("Adding member: {}:{:?}", member_id, member.company_name);

    sqlx::query(
        "INSERT INTO members (member_id, company_name, trading_name, company_type,
                     registration_number, tax_pin, year_established, website, email, phone_primary,
                     phone_secondary, physical_address, postal_address, city, county_state, country,
                     company_profile, annual_turnover_range, status, membership_category_id, join_date,
                     approved_by)
 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ? )",
    )
        .bind(member_id)
        // .bind(&payload.user_member_id)
        .bind(&member.company_name)
        .bind(&member.trading_name)
        .bind(&member.company_type)
        .bind(&member.registration_number)
        .bind(&member.tax_pin)
        .bind(&member.year_established)
        .bind(&member.website)
        .bind(&member.member_email)
        .bind(&member.member_phone_primary)
        .bind(&member.member_phone_secondary)
        .bind(&member.physical_address)
        .bind(&member.postal_address)
        .bind(&member.city)
        .bind(&member.county_state)
        .bind(&member.country)
        .bind(&member.company_profile)
        .bind(&member.annual_turnover)
        .bind(&member.status)
        .bind(&member.membership_category_id)
        .bind(local_date)
        .bind(&member.approved_by)
        .execute(&pool)
        .await
        .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))))?;


    Ok(Json(member))

}

async fn create_user(
    AuthUser(_claims) : AuthUser,
    State(pool): State<MySqlPool>,
    Json(payload): Json<User>,
) -> Result<Json<User>, (StatusCode, Json<serde_json::Value>)>

{





    let match_error = String::from("Passwords must match.");


    let user_post = payload.clone();
    if payload.user_password == payload.user_password_confirm
    {
        let hash = hash(user_post.user_password, DEFAULT_COST).expect("Failed to hash user password.");
        let user_id = rand::random_range(100..999).to_string();

        println!("Adding User : {}:{}", payload.user_member_id, payload.user_first_name);

        sqlx::query(
            "INSERT INTO users (user_id, first_name, last_name, email, phone, password_hash, role_id)
 VALUES (?, ?, ?, ?, ?, ?, ? )",
        )
            .bind(user_id)
           // .bind(&payload.user_member_id)
            .bind(&payload.user_first_name)
            .bind(&payload.user_last_name)
            .bind(&payload.user_email)
            .bind(&payload.user_phone)
            .bind(hash)
            .bind(&payload.role_id)
            .execute(&pool)
            .await
            .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))))?;


        Ok(Json(payload))
    } else  {
        Err((StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": match_error}))))
    }

}
async fn get_user(
    AuthUser(_claims) : AuthUser,
    State(pool): State<MySqlPool>,

) //-> Json<Vec<Member>>
{
    /*let members = vec![
        Member { id: 1, name: "Alice".into() },
        Member { id: 2, name: "Bob".into() },
    ];
    Json(members)

     */
}

async fn get_users(
    AuthUser(_claims) : AuthUser,
    State(pool): State<MySqlPool>,

) //-> Json<Vec<Member>>
{
    /*let members = vec![
        Member { id: 1, name: "Alice".into() },
        Member { id: 2, name: "Bob".into() },
    ];
    Json(members)

     */
}

