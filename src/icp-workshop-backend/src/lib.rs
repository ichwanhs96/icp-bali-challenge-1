#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

//1. IMPORT MANAGEMENT CANISTER
//This includes all methods and types needed
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse, TransformArgs,
    TransformContext, TransformFunc,
};


//Update method using the HTTPS outcalls feature
#[ic_cdk::update]
async fn get_stock_price(stock_symbol: String) -> String {
    let host = "finnhub.io";
    let url = format!(
        "https://{}/api/v1/quote?symbol={}&token=cr2q5ppr01qgsq6mpa1gcr2q5ppr01qgsq6mpa20",
        host, stock_symbol
    );

    // 2.2 Prepare headers for the system http_request call
    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}:443"),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "exchange_rate_canister".to_string(),
        },
    ];

    //note "CanisterHttpRequestArgument" and "HttpMethod" are declared in line 4
    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        method: HttpMethod::GET,
        body: None,               //optional for request
        max_response_bytes: None, //optional for request
        transform: Some(TransformContext {
            // The "method" parameter needs to have the same name as the function name of your transform function
            function: TransformFunc(candid::Func {
                principal: ic_cdk::api::id(),
                method: "transform".to_string(),
            }),
            // The "TransformContext" function does need a context parameter, it can be empty
            context: vec![],
        }),
        headers: request_headers,
    };

    //3. MAKE HTTP REQUEST AND WAIT FOR RESPONSE

    //Note: in Rust, `http_request()` needs to pass cycles if you are using ic_cdk: ^0.9.0
    let cycles = 230_949_972_000;

    match http_request(request, cycles).await {
        //4. DECODE AND RETURN THE RESPONSE

        //See:https://docs.rs/ic-cdk/latest/ic_cdk/api/management_canister/http_request/struct.HttpResponse.html
        Ok((response,)) => {
            //if successful, `HttpResponse` has this structure:
            // pub struct HttpResponse {
            //     pub status: Nat,
            //     pub headers: Vec<HttpHeader>,
            //     pub body: Vec<u8>,
            // }

            //You need to decode that Vec<u8> that is the body into readable text.
            //To do this:
            //  1. Call `String::from_utf8()` on response.body
            //  3. You use a switch to explicitly call out both cases of decoding the Blob into ?Text

            //The API response will look like this:

            // ("[[1682978460,5.714,5.718,5.714,5.714,243.5678]]")

            //Which can be formatted as this
            //  [
            //     [
            //         1682978460, <-- start/timestamp
            //         5.714, <-- low
            //         5.718, <-- high
            //         5.714, <-- open
            //         5.714, <-- close
            //         243.5678 <-- volume
            //     ],
            //  ]

            //Return the body as a string and end the method
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            //Return the error as a string and end the method
            message
        }
    }
}

// Strips all data that is not needed from the original response.
#[ic_cdk::query]
fn transform(raw: TransformArgs) -> HttpResponse {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    };

    if res.status == 200u64 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error from coinbase: err = {:?}", raw));
    }
    res
}