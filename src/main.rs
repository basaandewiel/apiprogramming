//cloned repo

// This repo demonstrates how to get JSON data from an HTTP API
// using the reqwest HTTP client library
// on known and unknown JSON structures and deserialize the results

use reqwest; // HTTP client library
use serde_json; // JSON library
use std::collections::HashMap;

// transform our main function into an asynchronous function.
#[tokio::main]
// Result: Indicates the function can return either a value or an error.
// (): Represents an empty tuple, commonly used to signify success when the function doesn't return a meaningful value.
// Box<dyn std::error::Error>: Specifies that the error type is dynamic and implements
// the std::error::Error trait. **Boxing errors** allows handling multiple error types easily,
// albeit with the drawback of runtime error type resolution.
// This approach simplifies error handling and maintains flexibility.
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // KNOWN RETURN - HashMap
    const URL2: &str = "https://httpbin.org/ip";
    match reqwest::get(URL2).await {
        // result type is Result/Option? that indicates error, or succes with the value resp
        Ok(resp) => {
            // this gives Response { url: Url { scheme: "https", cannot_be_a_base: false, username:...;
            // this is normal http response, also containing 'status'
            println!("resp={:?}", resp);
            println!("");

            // the returned object is a simple JSON data struct with
            // a String of "origin" as a key, and
            // a String value of our internet routable IP address.
            // The struct is as follows: {"origin": "IP ADDRESS"}.
            // Deserialize json response, and store hashmap into json var
            //
            // resp.json() is a method provided by the reqwest crate for deserializing JSON responses from HTTP requests
            //.await is used in asynchronous Rust code to wait for a future to complete before continuing execution.
            // In this context, it waits for the JSON deserialization to complete
            //<HashMap<String, String>> specifies the desired deserialization target. Here, it's a HashMap with String keys and values.
            //? is the error propagation operator in Rust, which will return an error if deserialization fails, and otherwise,
            // it assigns the deserialized JSON to json
            let json = resp.json::<HashMap<String, String>>().await?;
            println!("KNOWN RETURN {:?}", json);
        }
        Err(err) => {
            println!("Reqwest Error: {}", err);
        }
    }

    // UNKNOWN RETURN - serde_json::Value
    const URL1: &str = "https://dummyjson.com/products/1";
    match reqwest::get(URL1).await {
        Ok(resp) => {
            // serde_json::Value: declares variable named json of type serde_json::Value.
            // serde_json::Value is an *recursive* enum representing JSON data that can hold any valid JSON value: null, boolean, number, string, array, or object
            let json: serde_json::Value = resp.json().await?;
            println!("{:?}", json)
        }
        Err(err) => {
            println!("Reqwest Error: {}", err)
        }
    }

    let url =
        "https://enever.nl/api/stroomprijs_vandaag.php?token=e57b044a61bddbcd31b9db8a241fa098"
            .to_string();
    //let url = "http://ip.jsontest.com/".to_string();
    let response = reqwest::get(&url).await?; 
    // Ensure the request was successful (status code 2xx)
    if response.status().is_success() {
        //get json part of response and convert to serde_json::Value
        let response_value: serde_json::Value = response.json().await?; //serde_json::Value represents any valid JSON value.
                                                                       //println!("{:?}", response_json);
        println!("");
        println!("response_value= {:?}", response_value);
        //let response_json: serde_json::Value = serde_json::from_str(response_value);
        println!("Electricity cost of {}", response_value["data"][0]["datum"]);
        println!("Tibber: {}", response_value["data"][0]["prijsTI"]);

        // Print electricity tariffs 
        //for data in response_json.data {
        //    println!("Start Time: {}, End Time: {}, Value: {} NOK", data.start_time, data.end_time, data.value);
        //}
    } else {
        println!("Request failed with status code: {}", response.status());
    }

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: serde_json::Value = serde_json::from_str(data)?;
    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
