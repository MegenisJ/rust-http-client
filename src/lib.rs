pub mod client;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn create_http_client(uri: &str){
    let y = client::httpclient::create_client(
        uri = "localhost:5000", 
        http_version = None,
        headers = None,
        encode_request = None,
        encode_response = None);
}
/*pub fn how_it_works(){
    
    let client = create_http_client("someuri.com/");
    optionalsforCLient = {
            httpVersion = HttpVersions.x,       //// (enum)
            headers = Vec<(&str,&str>),         //Default headers for all requests
            base64request = true/false,         //To encode request?
            base64response = true/ false        //To decode response? 
    }
    let response = client.GetAsync("resource")?; //returns option<httpResponse , httpClientError>
    

}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
