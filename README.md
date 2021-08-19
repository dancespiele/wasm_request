# WASM Request

Create http request with rustwasm saving boilerplate

## How to use

### Include in `Cargo.toml`

```toml
wasm_request = '0.1'
```

### Create http request

```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Eur {
    eur: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CurrentPrice {
    current_price: Eur,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Price {
    market_data: CurrentPrice,
}

async fn get_token_price() -> Price {
    let options = get_options::<Price>(
        "https://api.coingecko.com/api/v3/coins/1million-token",
        Method::Get,
        None,
        None,
    );

    let price_1mt_token: Price = request(options).await.unwrap().into_serde().unwrap();

    price_1mt_token
}
```

### Post with body

```rust
use wasm_request::{get_options, request, Method, DataType};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub email: String,
    pub username: String,
    pub password: String,
}

async fn create_user() {
  let user = User {
    email: "test@example.com".to_string(),
    password: "test43214".to_string(),
  }

  let options = get_options::<User>(
    "https://api.sport-example.com/login",
    Method::Post,
    None,
    Some(DataType::Json(user)),
  );

  request(options).await.unwrap().into_serde().unwrap();  
}
```

### Post with form data

```rust
use wasm_request::{get_options, request, Method, DataType, get_defualt_headers};
use web_sys::FormData;

async fn add_form() {
  let form = FormData::new().unwrap();
  form.append_with_str("name", "Lee").unwrap();
  form.append_with_str("email", "test.example@gmail.com").unwrap();

  let headers = get_defualt_headers();
  headers.delete("Content-Type").unwrap();

  let options = get_options::<()>(
    "https://api.sport-example.com/user",
    Method::Post,
    None,
    Some(DataType::Form(form)),
  );

  request(options).await.unwrap().into_serde().unwrap();
}
```

### Manage Local Storage

```rust
set_storage("food", "ramen");
let food = get_storage("food").unwrap();
delete_storage("food");
```

## Run tests
`wasm-pack test --headless --chrome --firefox`

## Do you like Wasm Request
If you like Yew Styles, help me supporting the project:
- BAT rewards in case that you use [Brave Browser](https://brave.com/)
- Using this link to create an account in [Binance](https://www.binance.com/en/register?ref=DB8EPXF0) (get 10% fee back for every trading)
- [Github Sponsors](https://github.com/sponsors/dancespiele)

Wasm Request is [MIT](LICENSE-MIT.md) and [Apache-2.0](LICENSE-APACHE.md) licensed
