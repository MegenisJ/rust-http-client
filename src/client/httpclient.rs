struct HttpClient <T> {
    base_uri: T,
    http_version : HttpVersion,       //// (enum)
    headers : Vec<(T,T)>,         //Default headers for all requests
    base_64_request :bool ,         //To encode request?
    base_64_response : bool        //To decode response? 
}

pub fn create_client<Str>(
    uri : Str ,
    http_version : Option<HttpVersion>, 
    headers: Option<Vec(Str,Str)>, 
    encode_request: Option<bool>, 
    encode_response: Option<bool>,  ) -> HttpClient<Str>{
    
    return HttpClient {
            base_uri: uri,
            http_version: http_version.unwrap_or(HttpVersion::One), 
            headers: headers.unwrap_or(Vec::new()),
            base_64_request:encode_request.unwrap_or(false),
            base_64_response: encode_response.unwrap_or(false)
    }
}

enum HttpVersion{
    One,
    Two,
    Three
}
